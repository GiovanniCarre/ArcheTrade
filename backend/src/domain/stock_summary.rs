#[derive(Debug, Clone, serde::Serialize)]
pub struct StockSummary {
    pub symbol: String,
    pub name: String,
    pub current_price: f64,
    pub change_percent: f64,
}

impl StockSummary {
    pub fn new(symbol: &str, name: &str, current_price: f64, change_percent: f64) -> Self {
        Self {
            symbol: symbol.to_string(),
            name: name.to_string(),
            current_price,
            change_percent,
        }
    }
}