use axum::{
    extract::{Query},
    response::Json,
    routing::get,
    Router,
};
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

// ---- ROUTER ----
pub fn create_router(
    mongo_manager: Arc<MongoStockManager>,
) -> Router {
    Router::new()
        .route("/stocks/search", get(search_stock))
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