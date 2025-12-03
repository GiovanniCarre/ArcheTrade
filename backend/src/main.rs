mod application;
mod domain;
mod infrastructure;
mod interfaces;
use axum::{
    Router,
    extract::Extension,
};

use infrastructure::external_api::stock_repository::{
    finnhub_repository::FinnhubRepository,
    fake_stock_repository::FakeStockRepository,
};
use crate::application::stock_repository::StockRepository;
use crate::infrastructure::db::mongo_stock_manager::MongoStockManager;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use interfaces::stock_handler::create_router;
use application::stock_manager::StockManager;
use std::env;
use interfaces::admin_handler;
use crate::application::predicators::{NaivePredictor, SmaPredictor, StockPredictor};
use crate::application::predicators::ar_predictor::ArPredictor;
use crate::application::predicators::ema_predictor::EmaPredictor;
use crate::application::predicators::linear_regression_predictor::LinearRegressionPredictor;
use crate::application::prediction_service::PredictionService;

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any);

    let mongo_uri = env::var("MONGO_URI").expect("MONGO_URI manquant");
    let db_name = env::var("MONGO_DB").expect("MONGO_DB manquant");
    let finnhub_api_key = env::var("FINNHUB_API_KEY").expect("FINNHUB_API_KEY manquant").trim().to_string();;

    println!("MONGO_URI = {}", mongo_uri);
    println!("MONGO_DB = {}", db_name);

    println!("Connexion à MongoDB: {}...", db_name);
    let mongo_manager = match MongoStockManager::new(&mongo_uri, &db_name).await {
        Ok(manager) => {
            println!("Connexion à MongoDB réussie !");
            Arc::new(manager)
        }
        Err(e) => {
            eprintln!("Échec de la connexion à MongoDB : {:?}", e);
            std::process::exit(1);
        }
    };


    let finnhub_repo = Arc::new(FinnhubRepository::new(finnhub_api_key));
    let fake_repo = Arc::new(FakeStockRepository);

    let external_repos: Vec<Arc<dyn StockRepository>> = vec![finnhub_repo, fake_repo];

    let stock_manager = Arc::new(StockManager::new(mongo_manager.clone(), external_repos));
    let predictors: Vec<Arc<dyn StockPredictor>> = vec![
        Arc::new(NaivePredictor),
        Arc::new(SmaPredictor),
        Arc::new(EmaPredictor),
        Arc::new(LinearRegressionPredictor),
        Arc::new(ArPredictor),
    ];

    let prediction_service = Arc::new(PredictionService::new(predictors));

    let app = Router::new()
        .nest(
            "/api",
            create_router(mongo_manager.clone(), stock_manager.clone())
                .merge(admin_handler::admin_router(mongo_manager.clone()))
        )
        .layer(cors)
        .layer(Extension(mongo_manager.clone()))
        .layer(Extension(prediction_service));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
