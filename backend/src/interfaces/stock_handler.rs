use axum::{
    extract::{Path, Query},
    response::Json,
    routing::get,
    Router,
};
use axum::extract::Extension;
use crate::application::stock_service::StockService;
use crate::infrastructure::db::stock_repository::StockRepository;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct StockQuery {
    symbol: String,
    start_date: String,
    end_date: String,
}

#[derive(Serialize)]
pub struct StockResponse {
    symbol: String,
    date: String,
    price: f64,
}

#[derive(Deserialize)]
pub struct SearchQuery {
    symbol: String,
}

#[derive(Serialize)]
pub struct PredictionResponse {
    symbol: String,
    predicted_date: String,
    predicted_price: f64,
}

#[derive(Serialize)]
pub struct StockSummaryResponse {
    symbol: String,
    name: String,
    price: f64,
    change_percent: f64,
}

// ---- ROUTER ----
pub fn create_router<R: StockRepository + Clone + Send + Sync + 'static>(
    stock_service: Arc<StockService<R>>,
) -> Router {
    Router::new()
        .route("/stocks/:symbol/history", get(get_stock_history::<R>))
        .route("/stocks/:symbol/predict", get(get_stock_prediction::<R>))
        .route("/stocks/search", get(search_stock::<R>))
        .layer(Extension(stock_service))
}

// ---- HANDLERS ----
async fn get_stock_history<R: StockRepository + Clone + Send + Sync + 'static>(
    Path(symbol): Path<String>,
    Query(query): Query<StockQuery>,
    Extension(stock_service): Extension<Arc<StockService<R>>>,
) -> Json<Vec<StockResponse>> {
    let history = stock_service
        .get_history(&symbol)
        .await;

    Json(history.into_iter().map(|s| StockResponse {
        symbol: s.symbol,
        date: s.date.to_string(),
        price: s.price,
    }).collect())
}

async fn get_stock_prediction<R: StockRepository + Clone + Send + Sync + 'static>(
    Path(symbol): Path<String>,
    Extension(stock_service): Extension<Arc<StockService<R>>>,
) -> Json<PredictionResponse> {
    let prediction = stock_service.predict(&symbol).await;
    Json(PredictionResponse {
        symbol: prediction.symbol,
        predicted_date: prediction.predicted_date.to_string(),
        predicted_price: prediction.predicted_price,
    })
}
async fn search_stock<R: StockRepository + Clone + Send + Sync + 'static>(
    Query(query): Query<SearchQuery>,
    Extension(stock_service): Extension<Arc<StockService<R>>>,
) -> Json<Option<StockSummaryResponse>> {
    match stock_service.search_stock(&query.symbol).await {
        Ok(Some(summary)) => Json(Some(StockSummaryResponse {
            symbol: summary.symbol,
            name: summary.name,
            price: summary.current_price,
            change_percent: summary.change_percent,
        })),
        Ok(None) => Json(None),
        Err(_) => Json(None),
    }
}
