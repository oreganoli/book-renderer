use axum::{response::Html, routing::get, Router};

async fn serve_index() -> Html<&'static str> {
    Html(include_str!("../templates/index.html"))
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(serve_index));
    axum::Server::bind(
        &"127.0.0.1:8080"
            .parse()
            .expect("could not bind to port 8080"),
    )
    .serve(app.into_make_service())
    .await
    .unwrap();
}
