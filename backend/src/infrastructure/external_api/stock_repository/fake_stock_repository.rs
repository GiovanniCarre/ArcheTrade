use async_trait::async_trait;
use crate::application::stock_repository::StockRepository;
use crate::domain::generic_stock_data_dto::GenericStockDataDTO;
use crate::domain::time_series::{StockPoint, StockSegment, TimeInterval};
use chrono::{Utc, Duration};
use rand::Rng;

pub struct FakeStockRepository;

#[async_trait]
impl StockRepository for FakeStockRepository {
    async fn get_stock_dto(&self, symbol: &str) -> anyhow::Result<Option<GenericStockDataDTO>> {
        let mut rng = rand::rng();
        let now = Utc::now();

        let mut points = Vec::new();
        let mut last_close: f64 = 100.0 + rng.gen_range(-10.0..10.0);

        for i in 0..30 {
            let date = now - Duration::days(30 - i);
            let change: f64 = rng.gen_range(-2.0..2.0);

            let open: f64 = last_close + rng.gen_range(-1.0..1.0);
            let close: f64 = open + change;
            let high: f64 = open.max(close) + rng.gen_range(0.0..1.5);
            let low: f64 = open.min(close) - rng.gen_range(0.0..1.5);
            let volume: f64 = rng.gen_range(800.0..2000.0);

            points.push(StockPoint {
                timestamp: date,
                open,
                high,
                low,
                close,
                volume,
            });

            last_close = close;
        }

        let segment = StockSegment {
            start_date: points.first().unwrap().timestamp,
            end_date: points.last().unwrap().timestamp,
            interval: TimeInterval::Day,
            data_points: points,
        };

        let dto = GenericStockDataDTO::new(
            symbol.to_string(),
            Some("FakeData".to_string()),
            Some(Utc::now()),
            vec![segment],
        );

        Ok(Some(dto))
    }
}
