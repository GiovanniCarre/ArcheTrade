use crate::domain::stock::Stock;
use crate::infrastructure::db::stock_repository::StockRepository;
use async_trait::async_trait;
use chrono::NaiveDateTime;
use serde_json::Value;

#[derive(Clone)]
pub struct YahooFinanceRepository;

impl YahooFinanceRepository {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl StockRepository for YahooFinanceRepository {
    async fn get_history(&self, symbol: &str) -> Vec<Stock> {
        let url = format!(
            "https://query1.finance.yahoo.com/v8/finance/chart/{}?interval=1d&range=1mo",
            symbol
        );

        // Crée un client avec User-Agent
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

        // Parse JSON
        let resp_json: Value = match serde_json::from_str(&resp_text) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("JSON invalide: {}\nBody reçu: {}", e, resp_text);
                return vec![];
            }
        };

        // Extraire timestamps et prix de clôture
        let timestamps = resp_json["chart"]["result"][0]["timestamp"].as_array();
        let closes = resp_json["chart"]["result"][0]["indicators"]["quote"][0]["close"].as_array();

        let mut stocks = Vec::new();
        if let (Some(ts), Some(cs)) = (timestamps, closes) {
            for (t, c) in ts.iter().zip(cs.iter()) {
                if let (Some(ts_val), Some(price_val)) = (t.as_i64(), c.as_f64()) {
                    let date = NaiveDateTime::from_timestamp(ts_val, 0).date();
                    stocks.push(Stock::new(symbol, date, price_val));
                }
            }
        }

        stocks
    }
}
