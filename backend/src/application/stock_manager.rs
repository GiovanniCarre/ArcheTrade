use std::sync::Arc;
use crate::application::stock_repository::StockRepository;
use crate::domain::generic_stock_data_dto::GenericStockDataDTO;
use anyhow::Result;

pub struct StockManager {
    local_repo: Arc<dyn StockRepository>,
    external_repos: Vec<Arc<dyn StockRepository>>,
}

impl StockManager {
    pub fn new(
        local_repo: Arc<dyn StockRepository>,
        external_repos: Vec<Arc<dyn StockRepository>>,
    ) -> Self {
        Self {
            local_repo,
            external_repos,
        }
    }

    pub async fn get_stock_dto(&self, symbol: &str) -> Result<Vec<GenericStockDataDTO>> {
        let local_data = self.local_repo.get_stock_dto(symbol).await?;
        if !local_data.is_empty() {
            return Ok(local_data);
        }
        for repo in &self.external_repos {
            let external_data = repo.get_stock_dto(symbol).await?;
            if !external_data.is_empty() {
                self.local_repo.save_stock_dto(&external_data).await?;
                return Ok(external_data);
            }
        }

        Ok(Vec::new())
    }
}
