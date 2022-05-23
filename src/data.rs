/// Struct representing book data as it exists in the DB.
pub struct BookData {
    // Self-explanatory.
    title: String,
    author: String,
    price: f64,
    publisher: String,
    series: String,
    year: i32,
    pages: i32,
    /// Cover type (hardcover, softcover, etc.)
    cover: String,
    /// URL pointing at the store page to buy the book.
    shop_url: String,
}

/// Struct for displaying the full book data.
pub struct Book {
    data: BookData,
    description: String,
    available: bool,
}

impl From<BookData> for Book {
    fn from(data: BookData) -> Self {
        let description = if data.price == -1.0 || data.year == -1 || data.pages == -1 {
            "-".to_string()
        } else {
            let series_string = if data.series.is_empty() {
                "".to_string()
            } else {
                format!(" (part of the {} series)", &data.series)
            };
            format!(
                "{}{} by {}, {} {}, {} pages, {} cover",
                &data.title,
                series_string,
                &data.author,
                &data.publisher,
                &data.year,
                &data.pages,
                &data.cover
            )
        };
        let available = data.price != -1.0;
        Self {
            data,
            description,
            available,
        }
    }
}
