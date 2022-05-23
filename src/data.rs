use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Struct representing book data as it exists in the DB.
#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct BookData {
    // Self-explanatory.
    pub title: String,
    pub author: String,
    pub price: f64,
    pub publisher: String,
    pub series: String,
    pub year: i32,
    pub pages: i32,
    /// Cover type (hardcover, softcover, etc.)
    pub cover: String,
    /// URL pointing at the store page to buy the book.
    pub shop_url: String,
}

/// Struct for displaying the full book data.
#[derive(Serialize, Deserialize, Debug)]
pub struct Book {
    pub data: BookData,
    pub description: String,
    pub available: bool,
}

impl From<BookData> for Book {
    fn from(data: BookData) -> Self {
        let (available, description) = if data.price == -1.0 || data.year == -1 || data.pages == -1
        {
            (false, "-".to_string())
        } else {
            let series_string = if data.series.is_empty() {
                "".to_string()
            } else {
                format!(" (part of the {} series)", &data.series)
            };
            (
                true,
                format!(
                    "{}{} by {}, {} {}, {} pages, {} cover",
                    &data.title,
                    series_string,
                    &data.author,
                    &data.publisher,
                    &data.year,
                    &data.pages,
                    &data.cover
                ),
            )
        };
        Self {
            data,
            description,
            available,
        }
    }
}

pub struct BookRepository {
    conn_pool: sqlx::PgPool,
}
impl BookRepository {
    pub fn new(connection_pool: sqlx::PgPool) -> Self {
        Self {
            conn_pool: connection_pool,
        }
    }

    pub async fn get_books(&self) -> Result<Vec<Book>, String> {
        let mut conn = match self.conn_pool.acquire().await {
            Ok(c) => c,
            Err(e) => return Err(e.to_string()),
        };
        let stream = match sqlx::query("SELECT * FROM k_data;")
            .fetch_all(&mut conn)
            .await
        {
            Ok(rows) => rows,
            Err(e) => return Err(e.to_string()),
        };
        let books = stream
            .into_iter()
            .map(|row| {
                BookData::from_row(&row).expect("Book data could not be gotten from Postgres row.")
            })
            .map(|data| Book::from(data))
            .collect::<Vec<_>>();
        Ok(books)
    }
}
