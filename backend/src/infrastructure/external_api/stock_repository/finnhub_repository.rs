use async_trait::async_trait;
use anyhow::{anyhow, Result};
use chrono::{Utc, TimeZone, Duration};
use reqwest::Client;
use serde::Deserialize;

use crate::application::stock_repository::StockRepository;
use crate::domain::generic_stock_data_dto::GenericStockDataDTO;
use crate::domain::time_series::{StockPoint, StockSegment, TimeInterval};

#[derive(Debug, Deserialize)]
struct CandleResponse {
    c: Option<Vec<f64>>,
    h: Option<Vec<f64>>,
    l: Option<Vec<f64>>,
    o: Option<Vec<f64>>,
    v: Option<Vec<f64>>,
    t: Option<Vec<i64>>,
    s: Option<String>,
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

    async fn fetch_candles(
        &self,
        symbol: &str,
        from: i64,
        to: i64,
        resolution: &str,
    ) -> Result<CandleResponse> {
        let url = format!(
            "https://finnhub.io/api/v1/stock/candle?symbol={symbol}&resolution={resolution}&from={from}&to={to}&token={api_key}",
            symbol = symbol,
            resolution = resolution,
            from = from,
            to = to,
            api_key = self.api_key
        );

        let resp = self.client.get(&url).send().await.map_err(|e| {
            anyhow!("Erreur réseau: {}", e)
        })?;

        let status = resp.status();
        let body = resp.text().await.map_err(|e| {
             anyhow!("Erreur lecture body: {}", e)
        })?;

        if !status.is_success() {
            return Err(anyhow!("Requête Finnhub échouée avec statut {}", status));
        }

        serde_json::from_str::<CandleResponse>(&body).map_err(|e| {
            anyhow!("Erreur parsing JSON: {}", e)
        })
    }

    /// 🧩 Conversion CandleResponse → Vec<StockPoint>
    fn build_stock_points(&self, candle: &CandleResponse) -> Vec<StockPoint> {
        let (times, opens, highs, lows, closes, vols) = match (
            &candle.t,
            &candle.o,
            &candle.h,
            &candle.l,
            &candle.c,
            &candle.v,
        ) {
            (Some(t), Some(o), Some(h), Some(l), Some(c), Some(v)) => (t, o, h, l, c, v),
            _ => return vec![],
        };

        times
            .iter()
            .enumerate()
            .map(|(i, ts)| StockPoint {
                timestamp: Utc.timestamp_opt(*ts, 0).single().unwrap(),
                open: opens[i],
                high: highs[i],
                low: lows[i],
                close: closes[i],
                volume: vols[i],
            })
            .collect()
    }
}

#[async_trait]
impl StockRepository for FinnhubRepository {
    async fn get_stock_dto(&self, symbol: &str) -> Result<Option<GenericStockDataDTO>> {
        let now = Utc::now().timestamp();
        let thirty_days_ago = (Utc::now() - Duration::days(30)).timestamp();

        match self.fetch_candles(symbol, thirty_days_ago, now, "D").await {
            Ok(candle) => {
                if candle.s.as_deref() != Some("ok") {
                    return Ok(None);
                }

                let points = self.build_stock_points(&candle);
                if points.is_empty() {
                    return Ok(None);
                }

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

                Ok(Some(dto))
            }
            Err(err) => {
                eprintln!("❌ Erreur lors de la récupération du stock {symbol}: {err:?}");
                Ok(None)
            }
        }
    }
}