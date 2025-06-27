use std::convert::Infallible;
use std::net::SocketAddr;
use std::path::Path;

use crate::project::{ServeEvent, WatchSignal};
use crate::tui_hint;
use crate::{project::Project, ServeArgs};
use notify::Watcher;
use reflexo_typst::error::prelude::*;
use reflexo_typst::ImmutStr;
use serde::{Deserialize, Serialize};
use tokio::io::AsyncReadExt;
use warp::Filter;
use warp::Reply;

const LIVE_RELOAD_SERVER_EVENT: &str = r#"
<script>
  console.log("Live reload script loaded");
  const u = new URL("/live-reload", window.location.origin);
  u.searchParams.set("location", window.location.pathname);
  const eventSource = new EventSource(u);

  eventSource.onmessage = (event) => {
    if (event.data === "reload") {
      window.location.reload();
    }
  };

  const heartbeat = () => {
    const u = new URL("/heartbeat", window.location.origin);
    u.searchParams.set("location", window.location.pathname);
    fetch(u).catch((err) => console.error("Failed to send heartbeat:", err));
  };
  heartbeat();
  setInterval(heartbeat, 3000);
</script>
"#;

// todo: clean code here, but I'm tired.
pub async fn serve(args: ServeArgs) -> Result<()> {
    // #[cfg(feature = "tokio-console")]
    // console_subscriber::init();

    let mut proj = Project::new(args.compile)?;

    let http_addr: SocketAddr = args
        .addr
        .clone()
        .parse()
        .map_err(map_string_err("ParseServeAddr"))?;
    let dest_dir = proj.dest_dir.clone();

    let (hb_tx, hb_rx) = tokio::sync::mpsc::unbounded_channel();
    let (backend_tx, _) = tokio::sync::broadcast::channel(128);
    let btx = backend_tx.clone();

    // watch theme files
    let mut _watcher_stack = None;
    if let Some(theme_dir) = proj.args.theme.clone() {
        let hb_tx2 = hb_tx.clone();
        let mut watcher =
            notify::recommended_watcher(move |res: notify::Result<notify::Event>| match res {
                Ok(event) => {
                    let paths = event.paths.into_iter().collect::<Vec<_>>();
                    hb_tx2
                        .send(ServeEvent::ThemeChange(paths))
                        .unwrap_or_else(|e| {
                            tui_hint!("Failed to send heartbeat: {e}");
                        });
                }
                Err(e) => tui_hint!("Watcher error: {e}"),
            })
            .context_ut("Failed to create file watcher")?;

        if let Err(e) = watcher.watch(Path::new(&theme_dir), notify::RecursiveMode::Recursive) {
            tui_hint!("Failed to watch theme directory: {e}");
        } else {
            tui_hint!("Watching theme directory: {theme_dir:?}");
        }

        _watcher_stack = Some(watcher);
    }

    #[derive(Serialize, Deserialize)]
    struct LocationQuery {
        location: String,
    }

    let live_reload = warp::path("live-reload").and(warp::get()).map(move || {
            let mut backend_rx = btx.subscribe();
            warp::sse::reply(warp::sse::keep_alive().stream(async_stream::stream! {
                while let Ok(WatchSignal::Reload) = backend_rx.recv().await {
                    tui_hint!("Live reload triggered");
                    yield Ok::<warp::sse::Event, Infallible>(warp::sse::Event::default().data("reload"));
                }
            }))
        });
    let heartbeat = warp::path("heartbeat")
        .and(warp::get())
        .and(warp::query::<LocationQuery>())
        .map(move |query: LocationQuery| {
            let location = ImmutStr::from(query.location);
            let _ = hb_tx.send(ServeEvent::HoldPath(location.clone(), true));
            let hb_tx = hb_tx.clone();
            tokio::spawn(async move {
                // Simulate some work
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                let _ = hb_tx.send(ServeEvent::HoldPath(location, false));
            });
            warp::reply::with_status("", warp::http::StatusCode::OK)
        });
    let fallback = warp::fs::dir(dest_dir).map(|reply: warp::filters::fs::File| {
        if reply.path().extension().is_some_and(|ext| ext == "html") {
            let file = match std::fs::File::open(reply.path()) {
                Ok(file) => file,
                Err(e) => {
                    tui_hint!("Failed to open file: {e}");
                    return warp::reply::with_status(
                        "File not found",
                        warp::http::StatusCode::NOT_FOUND,
                    )
                    .into_response();
                }
            };
            let body = tokio::fs::File::from_std(file);

            let stream =
                tokio_util::io::ReaderStream::new(body.chain(LIVE_RELOAD_SERVER_EVENT.as_bytes()));

            let mut resp = warp::reply::Response::new(warp::hyper::body::Body::wrap_stream(stream));
            resp.headers_mut()
                .insert("Content-Type", "text/html".parse().unwrap());
            resp
        } else {
            reply.into_response()
        }
    });

    let server = live_reload.boxed().or(heartbeat
        .boxed()
        .or(fallback.boxed())
        .or(warp::path("dev").and(warp::fs::dir("")))
        .with(warp::compression::gzip()));

    let (addr, server) = warp::serve(server).bind_ephemeral(http_addr);
    tui_hint!("Server started at http://{addr}");

    // Build the book if it hasn't been built yet
    if !args.no_build {
        tokio::spawn(async move { proj.watch(hb_rx, backend_tx, Some(addr)).await });
    };

    server.await;

    Ok(())
}
