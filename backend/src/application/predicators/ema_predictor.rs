use async_trait::async_trait;
use chrono::{Utc, Duration};
use crate::application::predicators::stock_predictor::StockPredictor;
use crate::domain::prediction_point::PredictionPoint;

pub struct EmaPredictor;

#[async_trait]
impl StockPredictor for EmaPredictor {
    fn method_name(&self) -> &str {
        "EMA"
    }

    async fn predict(&self, history: &[PredictionPoint]) -> Vec<PredictionPoint> {
        if history.is_empty() {
            return vec![];
        }

        let alpha = 2.0 / (history.len() as f64 + 1.0);
        let mut ema = history[0].close;

        for p in history.iter().skip(1) {
            ema = alpha * p.close + (1.0 - alpha) * ema;
        }

        (1..=31)
            .map(|i| {
                let ts = Utc::now() + Duration::days(i);
                PredictionPoint::new(ts, ema, ema, ema)
            })
            .collect()
    }
}
