use async_trait::async_trait;
use chrono::{Utc, Duration};
use crate::application::predicators::stock_predictor::StockPredictor;
use crate::domain::prediction_point::PredictionPoint;

pub struct SmaPredictor;

#[async_trait]
impl StockPredictor for SmaPredictor {
    fn method_name(&self) -> &str {
        "SMA"
    }

    async fn predict(&self, history: &[PredictionPoint]) -> Vec<PredictionPoint> {
        if history.is_empty() {
            return vec![];
        }

        let mut closes: Vec<f64> = history.iter().map(|p| p.close).collect();
        closes.sort_by(|a, b| a.partial_cmp(b).unwrap());
        closes.reverse();
        closes.truncate(20);

        let sma = closes.iter().copied().sum::<f64>() / closes.len() as f64;

        (1..=31)
            .map(|i| {
                let ts = Utc::now() + Duration::days(i);
                PredictionPoint::new(ts, sma, sma, sma)
            })
            .collect()
    }
}
