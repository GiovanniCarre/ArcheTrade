use crate::domain::time_series::StockPoint;

pub fn last_close(points: &[StockPoint]) -> f64 {
    points.last().map(|p| p.close).unwrap_or(0.0)
}

pub fn day_change(points: &[StockPoint]) -> f64 {
    points.last().map(|p| p.close - p.open).unwrap_or(0.0)
}

pub fn day_change_percent(points: &[StockPoint]) -> Option<f64> {
    points.last().map(|p| (p.close - p.open) / p.open * 100.0)
}

pub fn sma(points: &[StockPoint], period: usize) -> Option<f64> {
    if points.len() < period { return None; }
    let sum: f64 = points.iter().rev().take(period).map(|p| p.close).sum();
    Some(sum / period as f64)
}

pub fn ema(points: &[StockPoint], period: usize) -> Option<f64> {
    if points.len() < period { return None; }
    let k = 2.0 / (period as f64 + 1.0);
    let mut ema_prev = points.iter().rev().take(period).last()?.close;
    for p in points.iter().rev().take(period).skip(1) {
        ema_prev = p.close * k + ema_prev * (1.0 - k);
    }
    Some(ema_prev)
}

pub fn bollinger(points: &[StockPoint], period: usize) -> (Option<f64>, Option<f64>) {
    if points.len() < period { return (None, None); }
    let slice = points.iter().rev().take(period).collect::<Vec<_>>();
    let mean = slice.iter().map(|p| p.close).sum::<f64>() / period as f64;
    let variance = slice.iter().map(|p| (p.close - mean).powi(2)).sum::<f64>() / period as f64;
    let stddev = variance.sqrt();
    (Some(mean + 2.0*stddev), Some(mean - 2.0*stddev))
}

pub fn rsi(points: &[StockPoint], period: usize) -> Option<f64> {
    if points.len() < period { return None; }
    let slice = points.iter().rev().take(period);
    let gains: f64 = slice.clone().map(|p| (p.close - p.open).max(0.0)).sum();
    let losses: f64 = slice.map(|p| (p.open - p.close).max(0.0)).sum();
    if losses == 0.0 { Some(100.0) } else { Some(100.0 - 100.0 / (1.0 + gains / losses)) }
}

pub fn macd(points: &[StockPoint]) -> Option<f64> {
    let ema7 = ema(points, 7)?;
    let ema30 = ema(points, 30)?;
    Some(ema7 - ema30)
}

pub fn atr(points: &[StockPoint], period: usize) -> Option<f64> {
    if points.len() < period { return None; }
    let slice = points.iter().rev().take(period).collect::<Vec<_>>();
    let tr_sum: f64 = slice.iter()
        .map(|p| (p.high - p.low).abs())
        .sum();
    Some(tr_sum / period as f64)
}

pub fn max_drawdown(points: &[StockPoint], period: usize) -> Option<f64> {
    if points.len() < period { return None; }
    let slice = points.iter().rev().take(period).map(|p| p.close).collect::<Vec<_>>();
    let max_price = slice.iter().copied().fold(f64::MIN, f64::max);
    let min_price = slice.iter().copied().fold(f64::MAX, f64::min);
    Some(min_price - max_price)
}

pub fn trend(points: &[StockPoint]) -> Option<String> {
    if points.is_empty() { return None; }
    let first = points.first()?.close;
    let last = points.last()?.close;
    Some(if last >= first { "Haussier".to_string() } else { "Baissier".to_string() })
}

pub fn cumulative_gain(points: &[StockPoint], period: usize) -> Option<f64> {
    if points.len() < period { return None; }
    let slice = points.iter().rev().take(period);
    Some(slice.map(|p| p.close - p.open).sum())
}

pub fn volume_avg(points: &[StockPoint], period: usize) -> Option<f64> {
    if points.len() < period { return None; }
    let slice = points.iter().rev().take(period);
    Some(slice.map(|p| p.volume as f64).sum::<f64>() / period as f64)
}

pub fn volatility(points: &[StockPoint], period: usize) -> Option<f64> {
    if points.len() < period { return None; }
    let slice = points.iter().rev().take(period).collect::<Vec<_>>();
    let mean = slice.iter().map(|p| p.close).sum::<f64>() / period as f64;
    let variance = slice.iter().map(|p| (p.close - mean).powi(2)).sum::<f64>() / period as f64;
    Some(variance.sqrt())
}
