use rust_decimal::Decimal;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SearchCriteria {
    pub title_contains: Option<String>,
    pub min_price: Option<Decimal>,
    pub max_price: Option<Decimal>,
}
impl Default for SearchCriteria {
    fn default() -> Self {
        Self {
            title_contains: None,
            min_price: None,
            max_price: None,
        }
    }
}
/// Enum representing sorting criteria.
#[derive(Debug, Deserialize)]
pub enum SortBy {
    Alphabetically,
    PriceAscending,
    PriceDescending,
}
impl Default for SortBy {
    fn default() -> Self {
        Self::Alphabetically
    }
}
