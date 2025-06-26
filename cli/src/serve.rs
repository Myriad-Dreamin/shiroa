use std::convert::Infallible;
use std::net::SocketAddr;

use crate::project::ServeEvent;
use crate::tui_hint;
use crate::{project::Project, ServeArgs};
use axum::http::Uri;
use axum::response::sse::{Event, KeepAlive};
use axum::routing::get;
use axum::Router;
use reflexo_typst::error::prelude::*;
use tokio::io::AsyncReadExt;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::decompression::RequestDecompressionLayer;
use tower_http::services::ServeDir;

const LIVE_RELOAD_SERVER_EVENT: &str = r#"
<script>
  console.log("Live reload script loaded");
  const eventSource = new EventSource("/live-reload");
  eventSource.onmessage = (event) => {
    if (event.data === "reload") {
      window.location.reload();
    }
  };
</script>
"#;

pub async fn serve(args: ServeArgs) -> Result<()> {
    let mut proj = Project::new(args.compile)?;

    let http_addr: SocketAddr = args
        .addr
        .clone()
        .parse()
        .map_err(map_string_err("ParseServeAddr"))?;
    // run our app with hyper, listening globally on port 3000
    let dest_dir = proj.dest_dir.clone();

    let (backend_tx, _) = tokio::sync::broadcast::channel(128);

    let tx = backend_tx.clone();
    let server = Router::new()
        .nest_service("/dev", ServeDir::new(""))
        .route(
            "/live-reload",
            get(async move |uri: Uri| {
                let mut backend_rx = tx.subscribe();
                axum::response::sse::Sse::new(async_stream::stream! {
                    let _ = uri;
                    while let  Ok(ServeEvent::FsChange) =  backend_rx.recv().await {
                        tui_hint!("File system change detected, reloading...");
                        yield Ok::<Event, Infallible>(Event::default().data("reload"));
                    }
                })
                .keep_alive(KeepAlive::default())
            }),
        )
        .fallback(get(async move |uri: Uri| {
            let path = uri.path().trim_start_matches('/');
            let mut path = dest_dir.join(path);

            if path.is_dir() {
                path = path.join("index.html");
            }

            let mut file = match tokio::fs::File::open(&path)
                .await
                .context("failed to open file")
            {
                Ok(file) => file,
                Err(e) => {
                    tui_hint!("Failed to open file: {e}");
                    return axum::response::Response::builder()
                        .status(404)
                        .body(axum::body::Body::empty())
                        .unwrap();
                }
            };

            let mut data = Vec::new();
            if let Err(e) = file.read_to_end(&mut data).await {
                tui_hint!("Failed to read file: {e}");
                return axum::response::Response::builder()
                    .status(404)
                    .body(axum::body::Body::empty())
                    .unwrap();
            }

            if path.extension().and_then(|s| s.to_str()) == Some("html") {
                data.extend_from_slice(LIVE_RELOAD_SERVER_EVENT.as_bytes());
            }
            let guess = mime_guess::from_path(path);

            axum::response::Response::builder()
                .status(200)
                .header("Content-Type", guess.first_or_octet_stream().as_ref())
                .body(axum::body::Body::from(data))
                .unwrap()
        }))
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
        tokio::spawn(async move { proj.watch(backend_tx, Some(addr)).await });
    };

    axum::serve(listener, server)
        .await
        .context("failed to serve")?;

    Ok(())
}
