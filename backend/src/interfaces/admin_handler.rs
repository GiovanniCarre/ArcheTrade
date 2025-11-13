use axum::{
    extract::Extension,
    response::Json,
    routing::get,
    Router,
};
use crate::infrastructure::db::mongo_stock_manager::MongoStockManager;
use std::sync::Arc;
use crate::infrastructure::external_api::job_fetch_symbol::job_fetch_finnhub::fetch_all_stocks_from_finnhub;

// ---- ROUTER ADMIN ----
pub fn admin_router(mongo_manager: Arc<MongoStockManager>) -> Router {
    Router::new()
        .route("/admin/fill-stocks", get(fill_stocks_handler))
        .layer(Extension(mongo_manager))
}

// ---- HANDLER ----
async fn fill_stocks_handler(
    Extension(mongo_manager): Extension<Arc<MongoStockManager>>,
) -> Json<String> {
    match fetch_all_stocks_from_finnhub(&mongo_manager).await {
        Ok(_) => Json("Stocks importés avec succès !".to_string()),
        Err(err) => {
            eprintln!("Erreur lors de l'import : {:?}", err);
            Json(format!("Erreur : {:?}", err))
        }
    }
}
