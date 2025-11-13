use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

/// Niveau le plus fin : un point de données temporel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockPoint {
    pub timestamp: DateTime<Utc>,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

/// Type d’intervalle temporel du segment (granularité)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimeInterval {
    Tick,
    Minute,
    Hour,
    Day,
    Week,
    Month,
}

/// Un segment temporel contigu de données
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockSegment {
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub interval: TimeInterval,
    pub data_points: Vec<StockPoint>,
}