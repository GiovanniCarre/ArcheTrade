use async_trait::async_trait;
use chrono::{Utc, Duration};
use crate::application::predicators::stock_predictor::StockPredictor;
use crate::domain::prediction_point::PredictionPoint;

pub struct NaivePredictor;

#[async_trait]
impl StockPredictor for NaivePredictor {
    fn method_name(&self) -> &str {
        "NAIVE"
    }

    async fn predict(&self, history: &[PredictionPoint]) -> Vec<PredictionPoint> {
        let last_price = history.last().map(|p| p.close).unwrap_or(100.0);

        (1..=30)
            .map(|i| {
                let ts = Utc::now() + Duration::days(i);
                PredictionPoint::new(ts, last_price, last_price, last_price)
            })
            .collect()
    }
}