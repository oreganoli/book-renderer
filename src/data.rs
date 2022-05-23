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
