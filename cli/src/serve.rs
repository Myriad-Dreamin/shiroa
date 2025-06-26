use std::convert::Infallible;
use std::future::Future;
use std::net::SocketAddr;
use std::path::Path;
use std::pin::Pin;

use crate::project::{ServeEvent, WatchSignal};
use crate::tui_hint;
use crate::{project::Project, ServeArgs};
use axum::extract::Request;
use axum::http::Uri;
use axum::response::sse::{Event, KeepAlive};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::Router;
use notify::Watcher;
use reflexo_typst::error::prelude::*;
use reflexo_typst::ImmutStr;
use tokio::io::AsyncReadExt;
use tokio_util::io::ReaderStream;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::decompression::RequestDecompressionLayer;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::ServiceExt;

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
    #[cfg(feature = "tokio-console")]
    console_subscriber::init();

    let mut proj = Project::new(args.compile)?;

    let http_addr: SocketAddr = args
        .addr
        .clone()
        .parse()
        .map_err(map_string_err("ParseServeAddr"))?;
    let dest_dir = proj.dest_dir.clone();

    let (hb_tx, hb_rx) = tokio::sync::mpsc::unbounded_channel();
    let (backend_tx, _) = tokio::sync::broadcast::channel(128);

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

    let tx = backend_tx.clone();
    let server = Router::new()
        .nest_service("/dev", ServeDir::new(""))
        .route(
            "/live-reload",
            get(async move || {
                let mut backend_rx = tx.subscribe();
                axum::response::sse::Sse::new(async_stream::stream! {
                    while let Ok(WatchSignal::Reload) = backend_rx.recv().await {
                        yield Ok::<Event, Infallible>(Event::default().data("reload"));
                    }
                })
                .keep_alive(KeepAlive::default())
            }),
        )
        .route(
            "/heartbeat",
            get(async move |uri: Uri| {
                // get location from query params
                let path: ImmutStr = uri
                    .query()
                    .and_then(|q| {
                        url::form_urlencoded::parse(q.as_bytes())
                            .find(|(k, _)| k == "location")
                            .map(|(_, v)| v)
                    })
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| "index.html".to_string())
                    .into();
                let _ = hb_tx.send(ServeEvent::HoldPath(path.clone(), true));
                tokio::spawn(async move {
                    // Simulate some work
                    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                    let _ = hb_tx.send(ServeEvent::HoldPath(path, false));
                });

                Response::builder()
                    .status(200)
                    .body(axum::body::Body::empty())
                    .unwrap()
            }),
        )
        .fallback_service(FileService::new(dest_dir))
        .layer(
            ServiceBuilder::new()
                .layer(RequestDecompressionLayer::new())
                .layer(CompressionLayer::new()),
        );

    let listener = tokio::net::TcpListener::bind(http_addr)
        .await
        .context("failed to bind address")?;
    let addr = listener
        .local_addr()
        .context("failed to get local address")?;
    tui_hint!("Server started at http://{addr}");

    // Build the book if it hasn't been built yet
    if !args.no_build {
        tokio::spawn(async move { proj.watch(hb_rx, backend_tx, Some(addr)).await });
    };

    axum::serve(listener, server)
        .await
        .context("failed to serve")?;

    Ok(())
}

#[derive(Debug, Clone)]
struct FileService {
    dest_dir: std::path::PathBuf,
}

impl FileService {
    pub fn new(dest_dir: std::path::PathBuf) -> Self {
        Self { dest_dir }
    }
}

type FileFuture = Pin<Box<dyn Future<Output = Result<Response, Infallible>> + Send>>;

impl tower::Service<Request<axum::body::Body>> for FileService {
    type Response = Response;
    type Error = Infallible;
    type Future = FileFuture;

    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<axum::body::Body>) -> Self::Future {
        let path = req.uri().path().trim_start_matches('/');
        let mut path = self.dest_dir.join(path);

        if path.is_dir() {
            path = path.join("index.html");
        }

        if path.extension().and_then(|s| s.to_str()) == Some("html") {
            Box::pin(async move {
                let file = match tokio::fs::File::open(&path)
                    .await
                    .context("failed to open file")
                {
                    Ok(file) => tokio::io::BufReader::new(file),
                    Err(e) => {
                        tui_hint!("Failed to open file: {e}");
                        return Ok(Response::builder()
                            .status(404)
                            .body(axum::body::Body::empty())
                            .unwrap()
                            .into_response());
                    }
                };
                let body = axum::body::Body::from_stream(ReaderStream::new(
                    file.chain(LIVE_RELOAD_SERVER_EVENT.as_bytes()),
                ));

                Ok(Response::builder()
                    .status(200)
                    .header("Content-Type", "text/html; charset=utf-8")
                    .body(body)
                    .unwrap()
                    .into_response())
            }) as FileFuture
        } else {
            Box::pin(
                ServeFile::new(path)
                    .map_response_body(axum::body::Body::new)
                    .call(req),
            )
        }
    }
}
