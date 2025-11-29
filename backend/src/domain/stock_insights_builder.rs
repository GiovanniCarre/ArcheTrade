use crate::domain::indicators;
use crate::domain::stock_insights::StockInsights;
use crate::domain::time_series::StockSegment;
use std::time::Instant;

pub struct StockInsightsBuilder;

impl StockInsightsBuilder {
    pub fn build(historical_segments: &[StockSegment]) -> StockInsights {
        let start = Instant::now();

        let mut insights = StockInsights::new();

        let mut points = historical_segments
            .iter()
            .flat_map(|s| s.data_points.clone())
            .collect::<Vec<_>>();

        if points.is_empty() {
            return insights;
        }

        points.sort_by_key(|p| p.timestamp);

        insights.last_price = Some(indicators::last_close(&points));
        insights.day_change = Some(indicators::day_change(&points));
        insights.day_change_percent = indicators::day_change_percent(&points);

        insights.sma_7 = indicators::sma(&points, 7);
        insights.sma_30 = indicators::sma(&points, 30);

        insights.ema_7 = indicators::ema(&points, 7);
        insights.ema_30 = indicators::ema(&points, 30);

        let (upper, lower) = indicators::bollinger(&points, 20);
        insights.bollinger_upper = upper;
        insights.bollinger_lower = lower;

        insights.rsi_14 = indicators::rsi(&points, 14);

        insights.macd = indicators::macd(&points);

        insights.atr_14 = indicators::atr(&points, 14);

        insights.max_drawdown_30d = indicators::max_drawdown(&points, 30);

        insights.trend = indicators::trend(&points);

        insights.cumulative_gain_30d = indicators::cumulative_gain(&points, 30);

        insights.volume_avg_30d = indicators::volume_avg(&points, 30);
        insights.volatility_30d = indicators::volatility(&points, 30);

        insights.price_vs_sector = Some(insights.last_price.unwrap_or(0.0) / 100.0);

        if let Some(rsi) = insights.rsi_14 {
            insights.alert_overbought = Some(rsi > 70.0);
            insights.alert_oversold = Some(rsi < 30.0);
        }
        println!("StockInsightsBuilder: calcul terminé en {:?}", start.elapsed());

        insights
    }
}
