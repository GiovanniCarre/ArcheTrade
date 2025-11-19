use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::domain::time_series::StockSegment;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenericStockDataDTO {
    pub symbol: String,
    pub provider: Option<String>,
    pub last_update: Option<DateTime<Utc>>,
    pub historical_segments: Vec<StockSegment>,
}

impl GenericStockDataDTO {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_string(),
            provider: None,
            last_update: None,
            historical_segments: Vec::new(),
        }
    }
}

