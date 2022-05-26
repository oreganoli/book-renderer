use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchCriteria {
    pub title_contains: Option<String>,
    pub min_price: Option<Decimal>,
    pub max_price: Option<Decimal>,
    pub sort_by: Option<SortBy>,
}
impl Default for SearchCriteria {
    fn default() -> Self {
        Self {
            title_contains: None,
            min_price: None,
            max_price: None,
            sort_by: Some(SortBy::default()),
        }
    }
}
/// Enum representing sorting criteria.
#[derive(Debug, Deserialize, Serialize)]
pub enum SortBy {
    #[serde(rename = "a_to_z")]
    Alphabetically,
    #[serde(rename = "price_asc")]
    PriceAscending,
    #[serde(rename = "price_desc")]
    PriceDescending,
}
impl Default for SortBy {
    fn default() -> Self {
        Self::Alphabetically
    }
}
