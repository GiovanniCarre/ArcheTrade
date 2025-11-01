use crate::domain::stock::Stock;
use crate::domain::stock_summary::StockSummary;
use crate::infrastructure::db::stock_repository::StockRepository;
use async_trait::async_trait;
use chrono::{NaiveDateTime};
use serde_json::Value;
use anyhow::Result;

#[derive(Clone)]
pub struct YahooFinanceRepository;

impl YahooFinanceRepository {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl StockRepository for YahooFinanceRepository {

    // Recherche d'un résumé d'action (StockSummary)
    async fn search_stock(&self, symbol: &str) -> Result<Option<StockSummary>> {
     println!("Symbole recherché: {}", symbol);
        let url = format!(
            "https://query1.finance.yahoo.com/v7/finance/quote?symbols={}",
            symbol
        );

        let client = reqwest::Client::new();
        let resp_text = client
            .get(&url)
            .header("User-Agent", "marketpulse/0.1")
            .send()
            .await?
            .text()
            .await?;
        println!("JSON reçu de Yahoo: {}", resp_text);
        let resp_json: Value = serde_json::from_str(&resp_text)?;
        let result = &resp_json["quoteResponse"]["result"];

        if let Some(stock_json) = result.as_array().and_then(|arr| arr.first()) {
            let symbol = stock_json["symbol"].as_str().unwrap_or(symbol);
            let name = stock_json["shortName"].as_str().unwrap_or("Unknown");
            let price = stock_json["regularMarketPrice"].as_f64().unwrap_or(0.0);
            let change_percent = stock_json["regularMarketChangePercent"].as_f64().unwrap_or(0.0);

            return Ok(Some(StockSummary::new(symbol, name, price, change_percent)));
        }

        Ok(None)
    }

    // Historique d'une action (Stock)
    async fn get_history(&self, symbol: &str) -> Vec<Stock> {
        let url = format!(
            "https://query1.finance.yahoo.com/v8/finance/chart/{}?interval=1d&range=1mo",
            symbol
        );

        let client = reqwest::Client::new();
        let resp_text = match client.get(&url).header("User-Agent", "marketpulse/0.1").send().await {
            Ok(r) => match r.text().await {
                Ok(t) => t,
                Err(e) => {
                    eprintln!("Erreur lecture du body: {}", e);
                    return vec![];
                }
            },
            Err(e) => {
                eprintln!("Erreur appel API Yahoo: {}", e);
                return vec![];
            }
        };

        let resp_json: Value = match serde_json::from_str(&resp_text) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("JSON invalide: {}\nBody reçu: {}", e, resp_text);
                return vec![];
            }
        };

        let timestamps = resp_json["chart"]["result"][0]["timestamp"].as_array();
        let closes = resp_json["chart"]["result"][0]["indicators"]["quote"][0]["close"].as_array();

        let mut stocks = Vec::new();
        if let (Some(ts), Some(cs)) = (timestamps, closes) {
            for (t, c) in ts.iter().zip(cs.iter()) {
                if let (Some(ts_val), Some(price_val)) = (t.as_i64(), c.as_f64()) {
                    // Conversion correcte du timestamp en NaiveDate
                    let date = NaiveDateTime::from_timestamp(ts_val, 0).date();
                    stocks.push(Stock::new(symbol, date, price_val));
                }
            }
        }

        stocks
    }
}
