use chrono::NaiveDate;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stock {
    pub symbol: String,
    pub name: String,
    pub provider: String,
    pub history: Vec<StockEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockEntry {
    pub date: NaiveDate,

    // Métriques principales
    pub price: f64,
    pub change_percent: Option<f64>,
    pub moving_average_5: Option<f64>,
    pub moving_average_10: Option<f64>,
    pub rsi: Option<f64>,
    pub volatility: Option<f64>,

    // Métriques supplémentaires
    pub extra_metrics: HashMap<String, f64>,
}

impl Stock {
    pub fn new(symbol: &str, name: &str, provider: &str) -> Self {
        Self {
            symbol: symbol.to_string(),
            name: name.to_string(),
            provider: provider.to_string(),
            history: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, entry: StockEntry) {
        self.history.push(entry);
    }
}

impl StockEntry {
    pub fn new(
        date: NaiveDate,
        price: f64,
        change_percent: Option<f64>,
        moving_average_5: Option<f64>,
        moving_average_10: Option<f64>,
        rsi: Option<f64>,
        volatility: Option<f64>,
    ) -> Self {
        Self {
            date,
            price,
            change_percent,
            moving_average_5,
            moving_average_10,
            rsi,
            volatility,
            extra_metrics: HashMap::new(),
        }
    }

    pub fn add_extra_metric(&mut self, name: &str, value: f64) {
        self.extra_metrics.insert(name.to_string(), value);
    }
}
