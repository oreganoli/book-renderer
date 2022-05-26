use std::sync::Arc;

use axum::{
    body::{self, Empty, Full},
    extract::{Path, Query},
    http::{header, HeaderValue, Response, StatusCode},
    response::{Html, IntoResponse, Redirect},
    Extension, Json,
};
use book_renderer::data::{BookRepository, SearchCriteria};
use tera::Tera;
// Static file serving.
#[cfg(not(debug_assertions))]
use include_dir::{include_dir, Dir};
#[cfg(not(debug_assertions))]
static STATICS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/static/");
#[cfg(not(debug_assertions))]
pub async fn serve_statics(Path(path): Path<String>) -> impl IntoResponse {
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
// Static file serving for development use - loads files on demand instead of having them baked in at build time.
#[cfg(debug_assertions)]
use tokio::io::AsyncReadExt;
#[cfg(debug_assertions)]
pub async fn serve_statics(Path(path): Path<String>) -> impl IntoResponse {
    let path = "static".to_owned() + &path;
    let mime_type = mime_guess::from_path(&path).first_or_text_plain();
    let mut file_buf = vec![];
    let file = match tokio::fs::File::open(&path).await {
        Ok(mut f) => {
            f.read_to_end(&mut file_buf).await.unwrap();
            Response::builder()
                .status(StatusCode::OK)
                .header(
                    header::CONTENT_TYPE,
                    HeaderValue::from_str(mime_type.as_ref()).unwrap(),
                )
                .body(body::boxed(Full::from(file_buf)))
                .unwrap()
        }
        Err(_) => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(body::boxed(Empty::new()))
            .unwrap(),
    };
    file
}

/// Redirect the default request to / to /books and display the HTML view.
pub async fn index_redirect() -> impl IntoResponse {
    Redirect::to("/books")
}
/// Our main view's HTML template, baked into the compiled executable.
static HTML_TEMPLATE: &'static str = include_str!("../templates/index.html");
/// Our main route. Displays a rendered HTML view of the book store.
pub async fn books_view(
    Extension(repo): Extension<Arc<BookRepository>>,
    criteria: Option<Query<SearchCriteria>>,
) -> impl IntoResponse {
    let mut ctx = tera::Context::new();
    //dbg!(&criteria);
    let criteria = criteria.map(|c| c.0).unwrap_or_default();
    ctx.insert("criteria", &criteria);
    //dbg!(&criteria);

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
    let html_string = match Tera::one_off(HTML_TEMPLATE, &ctx, true) {
        Ok(html) => html,
        Err(e) => format!("<h1>ERROR</h1><p>Error rendering HTML template: {}</p>", e),
    };
    Html(html_string)
}
/// The API route we use to get data for our JS frontend.
pub async fn books_json(
    Extension(repo): Extension<Arc<BookRepository>>,
    criteria: Option<Query<SearchCriteria>>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let criteria = criteria.map(|c| c.0).unwrap_or_default();
    //dbg!(&criteria);
    match repo.get_books(criteria).await {
        Ok(books) => Ok(Json(books)),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(format!("Error retrieving book data: {}", e)),
        )),
    }
}
