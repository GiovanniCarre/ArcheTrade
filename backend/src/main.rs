use axum::debug_handler;
use serde::{Serialize, Deserialize};
use axum::{
    extract::{Extension, Path},
    Json, Router,
};
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

mod application;
mod domain;
mod infrastructure;
mod interfaces;

use interfaces::stock_router;
use application::stock_service::StockService;
use domain::prediction::PredictionResult;
use infrastructure::external_api::yahoo_client::YahooFinanceRepository;

#[derive(Serialize)]
struct HelloReply {
    msg: String,
}

#[derive(Deserialize)]
struct EchoRequest {
    text: String,
}

#[derive(Serialize)]
struct EchoReply {
    text: String,
}

async fn hello() -> Json<HelloReply> {
    Json(HelloReply {
        msg: "Hello from Rust backend!".to_string(),
    })
}

async fn echo(Json(payload): Json<EchoRequest>) -> Json<EchoReply> {
    Json(EchoReply { text: payload.text })
}

// handler prédiction — utilise #[debug_handler] (tu as la feature "macros" activée)
#[debug_handler]
async fn predict_stock(
    Path(symbol): Path<String>,
    Extension(stock_service): Extension<Arc<StockService<YahooFinanceRepository>>>,
) -> Json<PredictionResult> {
    let result = stock_service.predict(&symbol).await;
    Json(result)
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any);

    let yahoo_repo = YahooFinanceRepository::new();
    let stock_service = Arc::new(StockService::new(yahoo_repo));

    let app = Router::new()
        .nest("/api", stock_router(stock_service.clone()))
        .layer(cors)
        .layer(Extension(stock_service));

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("🚀 Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
