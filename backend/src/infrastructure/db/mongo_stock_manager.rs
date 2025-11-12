use crate::domain::stock_summary::StockSummary;
use crate::domain::generic_stock_data_dto::GenericStockDataDTO;
use crate::domain::utils::can_be_symbol;
use crate::application::stock_repository::StockRepository;
use mongodb::options::ReplaceOptions;
use anyhow::Result;
use futures::TryStreamExt;
use async_trait::async_trait;
use mongodb::{
    bson::{doc, Regex},
    Client, Collection,
};

pub struct MongoStockManager {
    summary_collection: Collection<StockSummary>,
    data_collection: Collection<GenericStockDataDTO>,
}

#[async_trait]
impl StockRepository for MongoStockManager {
    async fn get_stock_dto(&self, symbol: &str) -> Result<Vec<GenericStockDataDTO>> {
        let filter = doc! { "symbol": symbol };
        let mut cursor = self
            .data_collection
            .find(filter)
            .sort(doc! { "date": -1 })
            .limit(1)
            .await?;

        let opt = cursor.try_next().await?;
        let vec = match opt {
            Some(dto) => vec![dto],
            None => vec![],
        };
        Ok(vec)
    }

    async fn save_stock_dto(&self, data: &[GenericStockDataDTO]) -> Result<()> {
        for dto in data {
            self.save_one_stock_dto(dto).await?;
        }
        Ok(())
    }
}

impl MongoStockManager {
    pub async fn new(uri: &str, db_name: &str) -> Result<Self> {
        let client = Client::with_uri_str(uri).await?;
        let db = client.database(db_name);

        let summary_collection = db.collection::<StockSummary>("summaries");
        let data_collection = db.collection::<GenericStockDataDTO>("stock_data");

        db.run_command(doc! { "ping": 1 }).await?;
        println!("Connexion MongoDB OK (db = {db_name})");

        Ok(Self {
            summary_collection,
            data_collection,
        })
    }

    pub async fn add_stock(&self, stock: StockSummary) -> Result<()> {
        self.summary_collection.insert_one(stock).await?;
        Ok(())
    }

    pub async fn search_by_name(&self, query: &str) -> Result<Vec<StockSummary>> {
        let filter = if can_be_symbol(query) {
            doc! { "symbol": { "$regex": Regex { pattern: query.to_string(), options: "i".into() } } }
        } else {
            doc! { "name": { "$regex": Regex { pattern: query.to_string(), options: "i".into() } } }
        };

        let cursor = self.summary_collection.find(filter).limit(10).await?;
        Ok(cursor.try_collect().await?)
    }

    async fn save_one_stock_dto(&self, dto: &GenericStockDataDTO) -> Result<()> {
        let filter = doc! {
        "symbol": &dto.symbol,
        "date": dto.date.to_string(),
    };

        let opts = ReplaceOptions::builder().upsert(true).build();

        self.data_collection
            .replace_one(filter, dto)
            .await?;

        Ok(())
    }
}