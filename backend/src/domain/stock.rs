use chrono::NaiveDate;

/// Représente un prix d'action à une date donnée
#[derive(Debug, Clone)]
pub struct Stock {
    pub symbol: String,
    pub date: NaiveDate,
    pub price: f64,
}

impl Stock {
    pub fn new(symbol: &str, date: NaiveDate, price: f64) -> Self {
        Self {
            symbol: symbol.to_string(),
            date,
            price,
        }
    }
}
