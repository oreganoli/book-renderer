use axum::{
    body::{self, Empty, Full},
    extract::Path,
    http::{header, HeaderValue, Response, StatusCode},
    response::{Html, IntoResponse},
    routing::get,
    Extension, Router,
};
use book_renderer::data::{Book, BookData};
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

async fn serve_index(Extension(renderer): Extension<Tera>) -> impl IntoResponse {
    let html_string = renderer
        .render("index.html", &tera::Context::new())
        .unwrap_or(
            "<h1>ERROR</h1><p>Error rendering HTML template. Consult main.rs.</p>".to_string(),
        );
    Html(html_string)
}

#[tokio::main]
async fn main() {
    // Create some bogus data.
    let books = vec![
        BookData {
            author: "Terry Pratchett".into(),
            title: "Wintersmith".into(),
            cover: "soft".into(),
            pages: 262,
            price: 29.90,
            publisher: "Prószyński i S-ka".into(),
            year: 2006,
            series: "Tiffany Aching".into(),
            shop_url: "http://example.com/1".into(),
        },
        BookData {
            author: "Terry Pratchett".into(),
            title: "I Shall Wear Midnight".into(),
            cover: "soft".into(),
            pages: 300,
            price: 29.97,
            publisher: "Prószyński i S-ka".into(),
            year: 2010,
            series: "Tiffany Aching".into(),
            shop_url: "http://example.com/2".into(),
        },
        BookData {
            author: "Invalid Author".into(),
            title: "Ayy Lmao".into(),
            cover: "soft".into(),
            pages: -1,
            price: 29.97,
            publisher: "Prószyński i S-ka".into(),
            year: 2010,
            series: "Placeholder Series".into(),
            shop_url: "http://example.com/2".into(),
        },
        BookData {
            author: "Invalid Author".into(),
            title: "Jajco".into(),
            cover: "soft".into(),
            pages: 2,
            price: -1.0,
            publisher: "Prószyński i S-ka".into(),
            year: 2010,
            series: "".into(),
            shop_url: "http://example.com/2".into(),
        },
        BookData {
            author: "Invalid Author".into(),
            title: "Jajco 2".into(),
            cover: "soft".into(),
            pages: 2,
            price: 1.56,
            publisher: "Prószyński i S-ka".into(),
            year: -1,
            series: "".into(),
            shop_url: "http://example.com/2".into(),
        },
    ]
    .into_iter()
    .map(|book_data| Book::from(book_data))
    .collect::<Vec<_>>();

    // Create Tera template engine.
    let tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    // Create Axum web app.
    let app = Router::new()
        .route("/", get(serve_index))
        .route("/static/*path", get(serve_statics))
        .layer(Extension(tera));
    axum::Server::bind(
        &"127.0.0.1:8080"
            .parse()
            .expect("could not bind to port 8080"),
    )
    .serve(app.into_make_service())
    .await
    .unwrap();
}
