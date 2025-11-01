use crate::domain::stock::Stock;
use async_trait::async_trait;
use crate::domain::stock_summary::StockSummary;

#[async_trait]
pub trait StockRepository: Send + Sync {
    async fn get_history(&self, symbol: &str) -> Vec<Stock>;
    async fn search_stock(&self, symbol: &str) -> anyhow::Result<Option<StockSummary>>;
}
