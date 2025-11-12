mod application;
mod domain;
mod infrastructure;
mod interfaces;
use axum::{
    Router,
    extract::Extension,
};
use crate::application::stock_repository::StockRepository;
use crate::infrastructure::db::mongo_stock_manager::MongoStockManager;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use interfaces::stock_handler::create_router;
use application::stock_manager::StockManager;
use dotenv::dotenv;
use std::env;
use interfaces::admin_handler;

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any);

    dotenv().ok();

    let mongo_uri = env::var("MONGO_URI").expect("MONGO_URI manquant dans .env");
    let db_name = env::var("MONGO_DB").expect("manquant dans .env");

    println!("MONGO_URI = {}", mongo_uri);
    println!("MONGO_DB = {}", db_name);

    println!("Connexion à MongoDB: {}", db_name);
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

    let external_repos: Vec<Arc<dyn StockRepository>> = vec![];

    let stock_manager = Arc::new(StockManager::new(mongo_manager.clone(), external_repos));

    let app = Router::new()
        .nest("/api", create_router(mongo_manager.clone(), stock_manager.clone()))
        .nest("/api", admin_handler::admin_router(mongo_manager.clone()))
        .layer(cors)
       .layer(Extension(mongo_manager.clone()));

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
