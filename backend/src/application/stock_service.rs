use crate::domain::prediction::PredictionResult;
use crate::domain::stock::Stock;
use crate::infrastructure::db::stock_repository::StockRepository;
use chrono::Utc;

/// Service principal pour gérer les actions et prédictions
#[derive(Clone)]
pub struct StockService<R: StockRepository + Clone> {
    repo: R,
}

impl<R: StockRepository + Clone> StockService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    /// Récupère l'historique des prix pour un symbole donné
    pub async fn get_history(&self, symbol: &str) -> Vec<Stock> {
        self.repo.get_history(symbol).await
    }

    /// Calcule une prédiction simple pour un symbole
    /// Ici, moyenne simple des prix historiques
    pub async fn predict(&self, symbol: &str) -> PredictionResult {
        let history = self.get_history(symbol).await;

        let predicted_price = if !history.is_empty() {
            let sum: f64 = history.iter().map(|s| s.price).sum();
            sum / history.len() as f64
        } else {
            0.0
        };

        // Par défaut, prédiction pour le jour suivant
        let predicted_date = Utc::now().date_naive() + chrono::Duration::days(1);

        PredictionResult::new(symbol, predicted_date, predicted_price)
    }
}
