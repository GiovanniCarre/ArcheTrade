use async_trait::async_trait;
use crate::domain::prediction_point::PredictionPoint;

#[async_trait]
pub trait StockPredictor: Send + Sync {
    fn method_name(&self) -> &str;

    async fn predict(
        &self,
        history: &[PredictionPoint],
    ) -> Vec<PredictionPoint>;
}
