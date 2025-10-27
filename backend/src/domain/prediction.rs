use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Résultat d'une prédiction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionResult {
    pub symbol: String,
    pub predicted_date: NaiveDate,
    pub predicted_price: f64,
}

impl PredictionResult {
    pub fn new(symbol: &str, predicted_date: NaiveDate, predicted_price: f64) -> Self {
        Self {
            symbol: symbol.to_string(),
            predicted_date,
            predicted_price,
        }
    }
}
