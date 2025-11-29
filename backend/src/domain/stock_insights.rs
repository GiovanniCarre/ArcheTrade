use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockInsights {
    pub last_price: Option<f64>,
    pub day_change: Option<f64>,
    pub day_change_percent: Option<f64>,

    pub sma_7: Option<f64>,
    pub sma_30: Option<f64>,
    pub ema_7: Option<f64>,
    pub ema_30: Option<f64>,

    pub bollinger_upper: Option<f64>,
    pub bollinger_lower: Option<f64>,

    pub rsi_14: Option<f64>,
    pub macd: Option<f64>,
    pub atr_14: Option<f64>,

    pub max_drawdown_30d: Option<f64>,
    pub trend: Option<String>,
    pub cumulative_gain_30d: Option<f64>,
    pub volume_avg_30d: Option<f64>,
    pub volatility_30d: Option<f64>,

    pub price_vs_sector: Option<f64>,
    pub alert_overbought: Option<bool>,
    pub alert_oversold: Option<bool>,
}

impl StockInsights {
    pub fn new() -> Self {
        Self {
            last_price: None,
            day_change: None,
            day_change_percent: None,
            sma_7: None,
            sma_30: None,
            ema_7: None,
            ema_30: None,
            bollinger_upper: None,
            bollinger_lower: None,
            rsi_14: None,
            macd: None,
            atr_14: None,
            max_drawdown_30d: None,
            trend: None,
            cumulative_gain_30d: None,
            volume_avg_30d: None,
            volatility_30d: None,
            price_vs_sector: None,
            alert_overbought: None,
            alert_oversold: None,
        }
    }
}

impl Default for StockInsights {
    fn default() -> Self {
        Self::new()
    }
}