use crate::domain::stock_summary::StockSummary;
use mongodb::{Client, Collection};
use mongodb::bson::{doc, Regex};
use anyhow::Result;
use crate::domain::utils::can_be_symbol;
use futures::stream::TryStreamExt;

pub struct MongoStockManager {
    collection: Collection<StockSummary>,
}

impl MongoStockManager {
    pub async fn new(uri: &str, db_name: &str, collection_name: &str) -> Result<Self> {
        let client = Client::with_uri_str(uri).await?;
        let db = client.database(db_name);
        let collection = db.collection::<StockSummary>(collection_name);

        db.run_command(doc! { "ping": 1 }).await?;
        println!("Ping MongoDB réussi, connexion OK !");

        Ok(Self { collection })
    }

    pub async fn add_stock(&self, stock: StockSummary) -> Result<()> {
        self.collection.insert_one(stock).await?;
        Ok(())
    }

    pub async fn search_by_name(&self, name_query: &str) -> Result<Vec<StockSummary>> {
        let filter = if can_be_symbol(name_query) {
            doc! { "symbol": { "$regex": Regex { pattern: name_query.to_string(), options: "i".to_string() } } }
        } else {
            doc! { "name": { "$regex": Regex { pattern: name_query.to_string(), options: "i".to_string() } } }
        };

        let mut cursor = self.collection.find(filter).await?;
        let mut results = Vec::new();
        while let Some(stock) = cursor.try_next().await? {
            results.push(stock);
        }
        Ok(results)
    }
}
