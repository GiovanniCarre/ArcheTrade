use axum::{
    extract::Query,
    response::Json,
    routing::get,
    Router,
    extract::Extension,
};
use crate::application::stock_manager::StockManager;
use crate::domain::stock_summary::StockSummary;
use crate::infrastructure::db::mongo_stock_manager::MongoStockManager;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

// --- QUERY STRUCTS ---
#[derive(Deserialize)]
pub struct SearchQuery {
    query: String,
}

#[derive(Deserialize)]
pub struct StockQuery {
    symbol: String,
}

// --- RESPONSE STRUCTS ---
#[derive(Serialize)]
pub struct StockSummaryResponse {
    symbol: String,
    name: String,
    provider: String,
}

#[derive(Serialize)]
pub struct StockPointResponse {
    timestamp: chrono::DateTime<chrono::Utc>,
    open: f64,
    close: f64,
    high: f64,
    low: f64,
    volume: f64,
}

#[derive(Serialize)]
pub struct StockResponse {
    symbol: String,
    provider: Option<String>,
    historical_segments: Vec<StockSegmentResponse>,
}

#[derive(Serialize)]
pub struct StockSegmentResponse {
    start_date: chrono::DateTime<chrono::Utc>,
    end_date: chrono::DateTime<chrono::Utc>,
    interval: String,
    data_points: Vec<StockPointResponse>,
}

// ---- ROUTER ----
pub fn create_router(
    mongo_manager: Arc<MongoStockManager>,
    stock_manager: Arc<StockManager>,
) -> Router {
    Router::new()
        .route("/stocks/search", get(search_stock))
        .route("/stocks/info", get(get_stock_info))
        .layer(Extension(stock_manager))
        .layer(Extension(mongo_manager))
}

// ---- HANDLERS ----
async fn search_stock(
    Query(query): Query<SearchQuery>,
    Extension(mongo_manager): Extension<Arc<MongoStockManager>>,
) -> Json<Vec<StockSummaryResponse>> {
    match mongo_manager.search_by_name(&query.query).await {
        Ok(stocks) => {
            let response: Vec<StockSummaryResponse> = stocks
                .into_iter()
                .map(|s: StockSummary| StockSummaryResponse {
                    symbol: s.symbol,
                    name: s.name,
                    provider: s.provider,
                })
                .collect();
            Json(response)
        }
        Err(err) => {
            eprintln!("Erreur lors de la recherche : {:?}", err);
            Json(vec![])
        }
    }
}

async fn get_stock_info(
    Query(query): Query<StockQuery>,
    Extension(stock_manager): Extension<Arc<StockManager>>,
) -> Json<Vec<StockResponse>> {
    match stock_manager.get_stock_dto(&query.symbol).await {
        Ok(data) if !data.is_empty() => {
            let response = data
                .into_iter()
                .map(|dto| StockResponse {
                    symbol: dto.symbol,
                    provider: dto.provider.clone(),
                    historical_segments: dto.historical_segments.into_iter().map(|seg| StockSegmentResponse {
                        start_date: seg.start_date,
                        end_date: seg.end_date,
                        interval: format!("{:?}", seg.interval),
                        data_points: seg.data_points.into_iter().map(|pt| StockPointResponse {
                            timestamp: pt.timestamp,
                            open: pt.open,
                            close: pt.close,
                            high: pt.high,
                            low: pt.low,
                            volume: pt.volume,
                        }).collect(),
                    }).collect(),
                })
                .collect();
            Json(response)
        }
        Ok(_) => Json(vec![]),
        Err(err) => {
            eprintln!("Erreur lors de la récupération du stock : {:?}", err);
            Json(vec![])
        }
    }
}
