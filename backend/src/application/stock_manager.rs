use async_trait::async_trait;
use std::sync::Arc;
use crate::domain::stock::Stock;
use crate::domain::stock_summary::StockSummary;
use crate::infrastructure::db::stock_repository::StockRepository;
use anyhow::Result;

/// StockManager : gère une "piscine" de plusieurs repositories
pub struct StockManager {
    repos: Vec<Arc<dyn StockRepository>>,
}

impl StockManager {
    /// Crée un StockManager avec plusieurs repositories
    pub fn new(repos: Vec<Arc<dyn StockRepository>>) -> Self {
        Self { repos }
    }

    /// Recherche un résumé d'action
    /// Retourne le premier résultat disponible parmi les repositories
    pub async fn search_stock(&self, symbol: &str) -> Result<Option<StockSummary>> {
        for repo in &self.repos {
            match repo.search_stock(symbol).await {
                Ok(Some(summary)) => return Ok(Some(summary)),
                Ok(None) => continue, // passe au repo suivant
                Err(e) => {
                    eprintln!("Erreur sur repo {}: {}", symbol, e);
                    continue;
                }
            }
        }
        Ok(None)
    }

    /// Récupère l'historique depuis le premier repo qui a des données
    pub async fn get_history(&self, symbol: &str) -> Vec<Stock> {
        for repo in &self.repos {
            let history = repo.get_history(symbol).await;
            if !history.is_empty() {
                return history;
            }
        }
        Vec::new()
    }
}

/// Implémentation du trait StockRepository pour StockManager
/// Utile si tu veux passer directement StockManager à ton StockService
#[async_trait]
impl StockRepository for StockManager {
    async fn get_history(&self, symbol: &str) -> Vec<Stock> {
        self.get_history(symbol).await
    }

    async fn search_stock(&self, symbol: &str) -> Result<Option<StockSummary>> {
        self.search_stock(symbol).await
    }
}
