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
        println!("🔍 Recherche du stock '{}'", symbol);
        if !local_data.is_empty() {
            println!("✅ Données trouvées en local : {} éléments", local_data.len());

            return Ok(local_data);
        }
        println!("⚠️ Pas de données en local, recherche dans les dépôts externes...");

        for (i, repo) in self.external_repos.iter().enumerate() {
            println!("🌐 Recherche dans le dépôt externe #{}", i + 1);

            let external_data = repo.get_stock_dto(symbol).await?;
            if !external_data.is_empty() {
                println!("✅ Données trouvées dans le dépôt externe #{} : {} éléments", i + 1, external_data.len());
                self.local_repo.save_stock_dto(&external_data).await?;
                return Ok(external_data);
            } else {
                println!("❌ Aucun résultat dans le dépôt externe #{}", i + 1);
            }
        }

        println!("⚠️ Aucun résultat trouvé pour '{}'", symbol);
        Ok(Vec::new())
    }
}
