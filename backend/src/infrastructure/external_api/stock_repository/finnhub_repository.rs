use async_trait::async_trait;
use std::sync::Arc;
use anyhow::Result;
use crate::domain::generic_stock_data_dto::{GenericStockDataDTO};
use crate::domain::time_series::{StockPoint, StockSegment, TimeInterval};
use crate::application::stock_repository::StockRepository;
use reqwest::Client;
use chrono::{DateTime, Utc, TimeZone, Duration};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct CandleResponse {
    c: Vec<f64>,
    h: Vec<f64>,
    l: Vec<f64>,
    o: Vec<f64>,
    v: Vec<f64>,
    t: Vec<i64>,
    s: String,
}


pub struct FinnhubRepository {
    api_key: String,
    client: Client,
}

impl FinnhubRepository {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: Client::new(),
        }
    }

    /// Appel HTTP vers l'API Candle de Finnhub
    async fn fetch_candles(&self, symbol: &str, from: i64, to: i64, resolution: &str) -> Result<CandleResponse> {
        let url = format!(
            "https://finnhub.io/api/v1/stock/candle?symbol={symbol}&resolution={resolution}&from={from}&to={to}&token={api_key}",
            symbol = symbol,
            resolution = resolution,
            from = from,
            to = to,
            api_key = self.api_key
        );

        let resp = self.client.get(&url).send().await?;
        let candle: CandleResponse = resp.json().await?;
        Ok(candle)
    }

    /// Transforme la réponse en StockPoints
    fn build_stock_points(&self, candle: &CandleResponse) -> Vec<StockPoint> {
        candle.t.iter().enumerate().map(|(i, ts)| {
            StockPoint {
                timestamp: Utc.timestamp(*ts, 0),
                open: candle.o[i],
                high: candle.h[i],
                low: candle.l[i],
                close: candle.c[i],
                volume: candle.v[i],
            }
        }).collect()
    }
}

#[async_trait]
impl StockRepository for FinnhubRepository {
    async fn get_stock_dto(&self, symbol: &str) -> Result<Vec<GenericStockDataDTO>> {

        // Définir la période : par défaut 30 derniers jours
        let now = Utc::now().timestamp();
        let thirty_days_ago = (Utc::now() - Duration::days(30)).timestamp();

        let candle = self.fetch_candles(symbol, thirty_days_ago, now, "D").await?;

        if candle.s != "ok" || candle.t.is_empty() {
            return Ok(Vec::new());
        }

        let points = self.build_stock_points(&candle);

        let segment = StockSegment {
            start_date: points.first().unwrap().timestamp,
            end_date: points.last().unwrap().timestamp,
            interval: TimeInterval::Day,
            data_points: points,
        };

        let dto = GenericStockDataDTO {
            symbol: symbol.to_string(),
            provider: Some("Finnhub".to_string()),
            last_update: Some(Utc::now()),
            historical_segments: vec![segment],
        };

        Ok(vec![dto])
    }
}
