use std::{net::SocketAddr, sync::Arc};

use axum::{routing::get, Extension, Router};
use book_renderer::data::BookRepository;
mod routes;
use routes::*;

#[tokio::main]
async fn main() {
    // Create Postgres connection pool.
    let database_url = match std::env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => panic!("Provide a DATABASE_URL environment variable."),
    };
    let pool = match sqlx::PgPool::connect(&database_url).await {
        Ok(pool) => pool,
        Err(e) => panic!("Database connection error: {}", e.to_string()),
    };
    let repo = Arc::new(BookRepository::new(pool));
    // Create Axum web app.
    let port_string = if let Ok(port) = std::env::var("PORT") {
        format!("0.0.0.0:{}", port)
    } else {
        panic!("Provide a PORT environment variable.")
    };
    let socket_addr = port_string
        .parse::<SocketAddr>()
        .expect("The PORT environment variable did not resolve to a valid port number.");
    eprintln!(
        "Strona odpalona pod adresem http://{}/",
        port_string.replace("0.0.0.0", "127.0.0.1")
    );
    let app = Router::new()
        .route("/", get(index_redirect))
        .route("/books", get(books_view))
        .route("/api/books", get(books_json))
        .route("/static/*path", get(serve_statics))
        .layer(Extension(repo));
    axum::Server::bind(&socket_addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
