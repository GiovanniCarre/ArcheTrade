use async_trait::async_trait;
use chrono::{Utc, Duration};
use crate::application::predicators::stock_predictor::StockPredictor;
use crate::domain::prediction_point::PredictionPoint;

pub struct LinearRegressionPredictor;

#[async_trait]
impl StockPredictor for LinearRegressionPredictor {
    fn method_name(&self) -> &str {
        "LINEAR_REGRESSION"
    }

    async fn predict(&self, history: &[PredictionPoint]) -> Vec<PredictionPoint> {
        if history.len() < 2 {
            return vec![];
        }

        let n = history.len() as f64;
        let sum_x: f64 = (0..history.len()).map(|i| i as f64).sum();
        let sum_y: f64 = history.iter().map(|p| p.close).sum();
        let sum_xy: f64 = history.iter().enumerate().map(|(i, p)| i as f64 * p.close).sum();
        let sum_x2: f64 = (0..history.len()).map(|i| (i as f64).powi(2)).sum();

        let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_x2 - sum_x.powi(2));
        let intercept = (sum_y - slope * sum_x) / n;

        (1..=31)
            .map(|i| {
                let ts = Utc::now() + Duration::days(i);
                let pred = intercept + slope * (history.len() as f64 + i as f64 - 1.0);
                PredictionPoint::new(ts, pred, pred, pred)
            })
            .collect()
    }
}
