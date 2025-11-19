use async_trait::async_trait;
use crate::domain::generic_stock_data_dto::GenericStockDataDTO;

#[async_trait]
pub trait StockRepository: Send + Sync {
    async fn get_stock_dto(&self, symbol: &str) -> anyhow::Result<Option<GenericStockDataDTO>>;
    async fn save_stock_dto(&self, _data: &GenericStockDataDTO) -> anyhow::Result<()> {
        Ok(())
    }
}
