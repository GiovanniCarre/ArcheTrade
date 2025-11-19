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

    pub async fn get_stock_dto(&self, symbol: &str) -> Result<Option<GenericStockDataDTO>> {
        println!("Recherche du stock '{}'", symbol);

        let local_data = self.local_repo.get_stock_dto(symbol).await?;
        if let Some(dto) = local_data {
            return Ok(Some(dto));
        }

        for (i, repo) in self.external_repos.iter().enumerate() {
            let external_data = repo.get_stock_dto(symbol).await?;
            if let Some(dto) = external_data {

                //Sauvegarde en local
                if let Err(e) = self.local_repo.save_stock_dto(&dto).await { eprintln!("⚠️ Échec sauvegarde locale : {:?}", e);
                }

                return Ok(Some(dto));
            }
        }

        println!("Aucun résultat trouvé pour '{}'", symbol);
        Ok(None)
    }
}
