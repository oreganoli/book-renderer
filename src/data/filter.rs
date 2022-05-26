use rust_decimal::Decimal;

pub struct SearchCriteria {
    pub title_contains: Option<String>,
    pub min_price: Option<Decimal>,
    pub max_price: Option<Decimal>,
}

/// Enum representing sorting criteria.
pub enum SortBy {
    Alphabetically,
    PriceAscending,
    PriceDescending,
}
