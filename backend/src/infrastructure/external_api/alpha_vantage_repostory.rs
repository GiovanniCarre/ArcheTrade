use crate::domain::stock::Stock;
use crate::domain::stock_summary::StockSummary;
use crate::infrastructure::db::stock_repository::StockRepository;
use async_trait::async_trait;
use chrono::NaiveDate;
use serde_json::Value;
use anyhow::Result;

#[derive(Clone)]
pub struct AlphaVantageRepository {
    api_key: String,
}

impl AlphaVantageRepository {
    pub fn new(api_key: String) -> Self { Self { api_key } }
}

#[async_trait]
impl StockRepository for AlphaVantageRepository {
    async fn search_stock(&self, symbol: &str) -> Result<Option<StockSummary>> {
        let url = format!(
            "https://www.alphavantage.co/query?function=GLOBAL_QUOTE&symbol={}&apikey={}",
            symbol, self.api_key
        );

        let resp_text = reqwest::get(&url).await?.text().await?;
        let resp_json: Value = serde_json::from_str(&resp_text)?;
        let quote = &resp_json["Global Quote"];

        if quote.is_object() {
            let price = quote["05. price"].as_str().unwrap_or("0").parse::<f64>().unwrap_or(0.0);
            let change_percent = quote["10. change percent"].as_str().unwrap_or("0%")
                .trim_end_matches('%').parse::<f64>().unwrap_or(0.0);
            return Ok(Some(StockSummary::new(symbol, "Entreprise AlphaVantage".to_string(), price, change_percent)));
        }

        Ok(None)
    }

    async fn get_history(&self, _symbol: &str) -> Vec<Stock> {
        vec![]
    }
}
