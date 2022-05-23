use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Struct representing book data as it exists in the DB.
#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct BookData {
    // Self-explanatory.
    pub title: String,
    pub author: String,
    pub price: Decimal,
    pub publisher: String,
    pub series: String,
    pub year: i16,
    pub pages: i16,
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
    pub link: Option<String>,
}
// Helper function for possibly missing fields.
fn hyphenize_empty(string: &str) -> String {
    if string.is_empty() {
        "-".to_string()
    } else {
        string.to_owned()
    }
}
impl From<BookData> for Book {
    fn from(data: BookData) -> Self {
        let mut data = data;
        let available = data.price != Decimal::NEGATIVE_ONE;
        let description = {
            let price_text = if data.price == Decimal::NEGATIVE_ONE {
                "-".to_string()
            } else {
                data.price.to_string().replace(".", ",") // European decimals
            };
            let year_text = if data.year == -1 {
                "-".to_string()
            } else {
                data.year.to_string()
            };
            let pages_text = if data.pages == -1 {
                "-".to_string()
            } else {
                data.pages.to_string()
            };

            format!(
                "Autor: {}, cena: {}, wydawca: {}, rok wydania: {}, liczba stron: {}, okładka: {}",
                hyphenize_empty(&data.author),
                price_text,
                hyphenize_empty(&data.publisher),
                year_text,
                pages_text,
                hyphenize_empty(&data.cover)
            )
        };
        let link = if data.shop_url.is_empty() {
            None
        } else {
            Some(data.shop_url.to_owned())
        };
        Self {
            data,
            description,
            available,
            link,
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
