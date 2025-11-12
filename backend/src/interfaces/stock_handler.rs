use axum::{
    extract::{Query},
    response::Json,
    routing::get,
    Router,
};
use crate::application::stock_manager::StockManager;
use chrono::NaiveDate;
use axum::extract::Extension;
use crate::domain::stock_summary::StockSummary;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::infrastructure::db::mongo_stock_manager::MongoStockManager;

#[derive(Deserialize)]
pub struct SearchQuery {
    query: String,
}

#[derive(Serialize)]
pub struct StockSummaryResponse {
    symbol: String,
    name: String,
    provider: String,
}

#[derive(Deserialize)]
pub struct StockQuery {
    symbol: String,
}
#[derive(Serialize)]
pub struct StockResponse {
    symbol: String,
    provider: Option<String>,
    date: NaiveDate,
    open: Option<f64>,
    close: Option<f64>,
    high: Option<f64>,
    low: Option<f64>,
    volume: Option<f64>,
    extra: Option<serde_json::Value>,
}


// ---- ROUTER ----
pub fn create_router(
    mongo_manager: Arc<MongoStockManager>,
    stock_manager: Arc<StockManager>
) -> Router {
    Router::new()
        .route("/stocks/search", get(search_stock))
        .route("/stocks/info", get(get_stock_info))
        .layer(Extension(stock_manager))
        .layer(Extension(mongo_manager))
}

// ---- HANDLER ----
async fn search_stock(
    Query(query): Query<SearchQuery>,
    Extension(mongo_manager): Extension<Arc<MongoStockManager>>,
) -> Json<Vec<StockSummaryResponse>> {
    match mongo_manager.search_by_name(&query.query).await {
        Ok(stocks) => {
            let response: Vec<StockSummaryResponse> = stocks.into_iter()
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
                    provider: dto.provider,
                    date: dto.date,
                    open: dto.open,
                    close: dto.close,
                    high: dto.high,
                    low: dto.low,
                    volume: dto.volume,
                    extra: dto.extra,
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