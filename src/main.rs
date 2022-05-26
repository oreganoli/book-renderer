use std::{net::SocketAddr, sync::Arc};

use axum::{
    body::{self, Empty, Full},
    extract::{Path, Query},
    http::{header, HeaderValue, Response, StatusCode},
    response::{Html, IntoResponse, Redirect},
    routing::get,
    Extension, Router,
};
use book_renderer::data::{BookRepository, SearchCriteria};
use include_dir::{include_dir, Dir};
use tera::Tera;

// Static file serving.
static STATICS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/static/");
async fn serve_statics(Path(path): Path<String>) -> impl IntoResponse {
    let path = path.trim_start_matches('/');
    let mime_type = mime_guess::from_path(path).first_or_text_plain();

    match STATICS_DIR.get_file(path) {
        None => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(body::boxed(Empty::new()))
            .unwrap(),
        Some(file) => Response::builder()
            .status(StatusCode::OK)
            .header(
                header::CONTENT_TYPE,
                HeaderValue::from_str(mime_type.as_ref()).unwrap(),
            )
            .body(body::boxed(Full::from(file.contents())))
            .unwrap(),
    }
}

async fn books(
    Extension(renderer): Extension<Tera>,
    Extension(repo): Extension<Arc<BookRepository>>,
    criteria: Option<Query<SearchCriteria>>,
) -> impl IntoResponse {
    let mut ctx = tera::Context::new();
    let criteria = criteria.map(|c| c.0).unwrap_or_default();
    ctx.insert("criteria", &criteria);
    dbg!(&criteria);

    let books = match repo.get_books(criteria).await {
        Ok(books) => books,
        Err(e) => {
            return Html(format!(
                "<h1>ERROR</h1><p>Error retrieving book data: {}</p>",
                e
            ))
        }
    };
    ctx.insert("books", &books);

    let html_string = renderer.render("index.html", &ctx).unwrap_or(
        "<h1>ERROR</h1><p>Error rendering HTML template. Consult main.rs.</p>".to_string(),
    );
    Html(html_string)
}

async fn index_redirect() -> impl IntoResponse {
    Redirect::to("/books")
}

#[tokio::main]
async fn main() {
    // Create Tera template engine.
    let tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
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
    let port_string = match std::env::var("PORT") {
        Ok(port) => port,
        Err(_) => panic!("Provide a PORT environment variable."),
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
        .route("/books", get(books))
        .route("/static/*path", get(serve_statics))
        .layer(Extension(tera))
        .layer(Extension(repo));
    axum::Server::bind(&socket_addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
