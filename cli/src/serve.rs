use std::net::SocketAddr;

use warp::Filter;

use crate::ServeArgs;

pub async fn serve(args: ServeArgs) {
    use warp::http::Method;

    let http_addr: SocketAddr = args.addr.clone().parse().unwrap();

    // map these files to the root of the github-pages server
    let gh_pages = warp::path("typst-book").and({
        let renderer_wasm = warp::path("renderer").and(warp::fs::dir(
            "frontend/node_modules/@myriaddreamin/typst-ts-renderer",
        ));
        let renderer_js = warp::path("typst-main.js").and(warp::fs::file(
            "frontend/node_modules/@myriaddreamin/typst.ts/dist/main.js",
        ));
        let svg_utils_js =
            warp::path("svg_utils.js").and(warp::fs::file("frontend/src/svg_utils.cjs"));
        let typst_book = warp::path("typst-book.js").and(warp::fs::file("frontend/dist/main.js"));
        let dist_dir = warp::fs::dir("github-pages/dist");

        renderer_wasm
            .or(renderer_js)
            .or(svg_utils_js)
            .or(dist_dir)
            .or(typst_book)
            .or(warp::fs::dir("github-pages"))
    });

    let cors =
        warp::cors().allow_methods(&[Method::GET, Method::POST, Method::DELETE, Method::HEAD]);

    let routes = gh_pages.with(cors).with(warp::compression::gzip());

    let server = warp::serve(routes);

    server.run(http_addr).await
}
