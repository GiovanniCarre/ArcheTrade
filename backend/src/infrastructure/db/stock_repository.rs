use crate::domain::stock::Stock;
use async_trait::async_trait;

#[async_trait]
pub trait StockRepository: Send + Sync {
    async fn get_history(&self, symbol: &str) -> Vec<Stock>;
}
