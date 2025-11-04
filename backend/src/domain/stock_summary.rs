use serde::{Deserialize};

#[derive(Debug, Clone, serde::Serialize, Deserialize)]
pub struct StockSummary {
    pub symbol: String,
    pub name: String,
    pub provider:String,
}

impl StockSummary {
    pub fn new(symbol: &str, name: &str, provider: &str) -> Self {
        Self {
            symbol: symbol.to_string(),
            name: name.to_string(),
            provider: provider.to_string(),
        }
    }
}