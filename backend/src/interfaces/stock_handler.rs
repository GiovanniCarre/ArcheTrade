use axum::{
    extract::{Path, Query},
    response::Json,
    routing::get,
    Router,
};
use crate::application::stock_service::StockService;
use crate::domain::prediction::PredictionResult;
use crate::domain::stock::Stock;
use chrono::NaiveDate;
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

#[derive(Serialize)]
pub struct PredictionResponse {
    symbol: String,
    predicted_date: String,
    predicted_price: f64,
}

pub fn create_router<R: StockRepository + Clone + Send + Sync + 'static>(
    stock_service: Arc<StockService<R>>,
) -> Router {
    Router::new()
        .route("/stocks/:symbol/history", get(get_stock_history))
        .route("/stocks/:symbol/predict", get(get_stock_prediction))
        .layer(Extension(stock_service))
}

async fn get_stock_history(
    Path(symbol): Path<String>,
    Query(query): Query<StockQuery>,
    Extension(stock_service): Extension<Arc<StockService>>,
) -> Json<Vec<StockResponse>> {
    let history = stock_service
        .get_history(&symbol, &query.start_date, &query.end_date)
        .await;
    Json(history.into_iter().map(|s| StockResponse {
        symbol: s.symbol,
        date: s.date.to_string(),
        price: s.price,
    }).collect())
}

async fn get_stock_prediction(
    Path(symbol): Path<String>,
    Extension(stock_service): Extension<Arc<StockService>>,
) -> Json<PredictionResponse> {
    let prediction = stock_service.predict(&symbol).await;
    Json(PredictionResponse {
        symbol: prediction.symbol,
        predicted_date: prediction.predicted_date.to_string(),
        predicted_price: prediction.predicted_price,
    })
}
