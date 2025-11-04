use async_trait::async_trait;

#[async_trait]
pub trait StockRepository: Send + Sync {
}
