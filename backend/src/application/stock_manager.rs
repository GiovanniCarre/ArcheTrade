use std::sync::Arc;
use crate::application::stock_repository::StockRepository;

/// StockManager : gère une "piscine" de plusieurs repositories
pub struct StockManager {
    repos: Vec<Arc<dyn StockRepository>>,
}

impl StockManager {
    /// Crée un StockManager avec plusieurs repositories
    pub fn new(repos: Vec<Arc<dyn StockRepository>>) -> Self {
        Self { repos }
    }
}

