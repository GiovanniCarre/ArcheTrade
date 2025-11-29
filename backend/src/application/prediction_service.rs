use std::sync::Arc;
use crate::domain::prediction_point::PredictionPoint;
use crate::application::predicators::StockPredictor;

pub struct PredictionService {
    predictors: Vec<Arc<dyn StockPredictor>>,
}

impl PredictionService {
    pub fn new(predictors: Vec<Arc<dyn StockPredictor>>) -> Self {
        Self { predictors }
    }

    pub async fn predict_from_history(
        &self,
        method: &str,
        history: &[PredictionPoint],
    ) -> Vec<PredictionPoint> {
        let predictor = match self.predictors
            .iter()
            .find(|p| p.method_name().eq_ignore_ascii_case(method))
        {
            Some(p) => p,
            None => {
                eprintln!("Predictor '{}' introuvable", method);
                return vec![];
            }
        };

        predictor.predict(history).await
    }
}
