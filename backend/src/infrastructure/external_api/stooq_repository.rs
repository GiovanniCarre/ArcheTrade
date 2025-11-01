use crate::domain::stock::Stock;
use crate::domain::stock_summary::StockSummary;
use crate::infrastructure::db::stock_repository::StockRepository;
use async_trait::async_trait;
use chrono::NaiveDate;
use anyhow::Result;

#[derive(Clone)]
pub struct StooqRepository;

impl StooqRepository {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl StockRepository for StooqRepository {
    async fn search_stock(&self, symbol: &str) -> Result<Option<StockSummary>> {
        // Simulation : retourne un prix fixe
        Ok(Some(StockSummary::new(
            symbol,
            "Entreprise Stooq".to_string(),
            123.45,
            0.5,
        )))
    }

    async fn get_history(&self, symbol: &str) -> Vec<Stock> {
        // Simulation : 10 jours de données fictives
        (0..10)
            .map(|i| Stock::new(symbol, NaiveDate::from_ymd_opt(2025, 11, 1).unwrap(), 100.0 + i as f64))
            .collect()
    }
}
