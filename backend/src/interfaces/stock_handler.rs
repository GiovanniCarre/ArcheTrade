use axum::{
    extract::{Query, Path, Extension},
    response::Json,
    routing::get,
    Router,
};
use crate::application::stock_manager::StockManager;
use crate::domain::stock_summary::StockSummary;
use crate::infrastructure::db::mongo_stock_manager::MongoStockManager;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use axum::routing::post;
use crate::application::prediction_service::PredictionService;
use crate::domain::prediction_point::PredictionPoint;

// --- QUERY STRUCTS ---
#[derive(Deserialize)]
pub struct SearchQuery {
    query: String,
}

#[derive(Deserialize)]
pub struct PredictQuery {
    method: String,
}

#[derive(Deserialize)]
pub struct PredictRequest {
    pub method: String,
    pub history: Vec<PredictionPoint>,
}

#[derive(Deserialize)]
pub struct StockQuery {
    symbol: String,
}

#[derive(Serialize)]
pub struct StockSummaryResponse {
    symbol: String,
    name: String,
    provider: String,
}


#[derive(Serialize)]
pub struct StockInsightsResponse {
    last_price: Option<f64>,
    day_change: Option<f64>,
    day_change_percent: Option<f64>,
    sma_7: Option<f64>,
    sma_30: Option<f64>,
    ema_7: Option<f64>,
    ema_30: Option<f64>,
    bollinger_upper: Option<f64>,
    bollinger_lower: Option<f64>,
    rsi_14: Option<f64>,
    macd: Option<f64>,
    atr_14: Option<f64>,
    max_drawdown_30d: Option<f64>,
    trend: Option<String>,
    cumulative_gain_30d: Option<f64>,
    volume_avg_30d: Option<f64>,
    volatility_30d: Option<f64>,
    price_vs_sector: Option<f64>,
    alert_overbought: Option<bool>,
    alert_oversold: Option<bool>,
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
    insights: StockInsightsResponse,
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
        .route("/stock/predict", post(predict_stock))
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
) -> Json<Option<StockResponse>> {
    match stock_manager.get_stock_dto(&query.symbol).await {
        Ok(Some(dto)) => {
            let historical_segments: Vec<StockSegmentResponse> = dto
                .historical_segments
                .iter() // pas de move ici
                .map(|seg| StockSegmentResponse {
                    start_date: seg.start_date,
                    end_date: seg.end_date,
                    interval: format!("{:?}", seg.interval),
                    data_points: seg
                        .data_points
                        .iter()
                        .map(|pt| StockPointResponse {
                            timestamp: pt.timestamp,
                            open: pt.open,
                            close: pt.close,
                            high: pt.high,
                            low: pt.low,
                            volume: pt.volume,
                        })
                        .collect(),
                })
                .collect();

            let insights_ref = dto.insights(); // maintenant sûr, pas de move conflict

            let insights_response = StockInsightsResponse {
                last_price: insights_ref.last_price,
                day_change: insights_ref.day_change,
                day_change_percent: insights_ref.day_change_percent,
                sma_7: insights_ref.sma_7,
                sma_30: insights_ref.sma_30,
                ema_7: insights_ref.ema_7,
                ema_30: insights_ref.ema_30,
                bollinger_upper: insights_ref.bollinger_upper,
                bollinger_lower: insights_ref.bollinger_lower,
                rsi_14: insights_ref.rsi_14,
                macd: insights_ref.macd,
                atr_14: insights_ref.atr_14,
                max_drawdown_30d: insights_ref.max_drawdown_30d,
                trend: insights_ref.trend.clone(),
                cumulative_gain_30d: insights_ref.cumulative_gain_30d,
                volume_avg_30d: insights_ref.volume_avg_30d,
                volatility_30d: insights_ref.volatility_30d,
                price_vs_sector: insights_ref.price_vs_sector,
                alert_overbought: insights_ref.alert_overbought,
                alert_oversold: insights_ref.alert_oversold,
            };

            Json(Some(StockResponse {
                symbol: dto.symbol.clone(),
                provider: dto.provider.clone(),
                historical_segments,
                insights: insights_response,
            }))
        }
        Ok(None) => Json(None),
        Err(err) => {
            eprintln!("Erreur lors de la récupération du stock : {:?}", err);
            Json(None)
        }
    }
}

#[axum::debug_handler]
pub async fn predict_stock(
    Extension(prediction_service): Extension<Arc<PredictionService>>,
    axum::Json(req): axum::Json<PredictRequest>,
) -> axum::Json<Vec<PredictionPoint>> {
    let predictions = prediction_service
        .predict_from_history(&req.method, &req.history)
        .await;

    axum::Json(predictions)
}
