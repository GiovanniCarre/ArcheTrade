use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionPoint {
    pub timestamp: DateTime<Utc>,
    pub close: f64,
    pub upper: f64,
    pub lower: f64,
}

impl PredictionPoint {
    pub fn new(timestamp: DateTime<Utc>, close: f64, upper: f64, lower: f64) -> Self {
        Self { timestamp, close, upper, lower }
    }
}
