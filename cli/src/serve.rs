use std::net::SocketAddr;

use warp::Filter;

use crate::ServeArgs;

pub async fn serve(args: ServeArgs) {
    use warp::http::Method;

    let http_addr: SocketAddr = args.addr.clone().parse().unwrap();

    // map these files to the root of the github-pages server
    let gh_pages = warp::path("typst-book").and({
        let renderer = warp::path("renderer").and(warp::fs::dir(
            "frontend/node_modules/@myriaddreamin/typst-ts-renderer",
        ));
        let typst_book = warp::path("typst-book.js").and(warp::fs::file("frontend/dist/main.js"));
        let theme_dir = warp::fs::dir("themes/typst-book");

        renderer
            .or(theme_dir)
            .or(typst_book)
            .or(warp::fs::dir("github-pages"))
    });

    let cors =
        warp::cors().allow_methods(&[Method::GET, Method::POST, Method::DELETE, Method::HEAD]);

    let routes = gh_pages.with(cors).with(warp::compression::gzip());

    let server = warp::serve(routes);

    server.run(http_addr).await
}
