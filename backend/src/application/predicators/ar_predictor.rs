use async_trait::async_trait;
use chrono::{Utc, Duration};
use crate::application::predicators::stock_predictor::StockPredictor;
use crate::domain::prediction_point::PredictionPoint;

pub struct ArPredictor;

#[async_trait]
impl StockPredictor for ArPredictor {
    fn method_name(&self) -> &str {
        "AR"
    }

    async fn predict(&self, history: &[PredictionPoint]) -> Vec<PredictionPoint> {
        if history.is_empty() {
            return vec![];
        }

        let last = history.last().unwrap().close;

        (1..=31)
            .map(|i| {
                let ts = Utc::now() + Duration::days(i);
                PredictionPoint::new(ts, last, last, last)
            })
            .collect()
    }
}
