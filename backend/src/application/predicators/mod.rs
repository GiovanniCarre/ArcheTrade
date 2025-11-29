pub mod stock_predictor;
pub mod sma_predictor;
pub mod naive_predictor;
pub mod ema_predictor;
pub mod linear_regression_predictor;
pub mod ar_predictor;

pub use stock_predictor::StockPredictor;
pub use sma_predictor::SmaPredictor;
pub use naive_predictor::NaivePredictor;