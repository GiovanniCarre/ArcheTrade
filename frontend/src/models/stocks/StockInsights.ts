export interface IStockInsights {
    last_price?: number;
    day_change?: number;
    day_change_percent?: number;

    sma_7?: number;
    sma_30?: number;
    ema_7?: number;
    ema_30?: number;

    bollinger_upper?: number;
    bollinger_lower?: number;

    rsi_14?: number;
    macd?: number;
    atr_14?: number;

    max_drawdown_30d?: number;
    trend?: string;
    cumulative_gain_30d?: number;
    volume_avg_30d?: number;
    volatility_30d?: number;

    price_vs_sector?: number;
    alert_overbought?: boolean;
    alert_oversold?: boolean;
}

export class StockInsights implements IStockInsights {
    last_price?: number;
    day_change?: number;
    day_change_percent?: number;

    sma_7?: number;
    sma_30?: number;
    ema_7?: number;
    ema_30?: number;

    bollinger_upper?: number;
    bollinger_lower?: number;

    rsi_14?: number;
    macd?: number;
    atr_14?: number;

    max_drawdown_30d?: number;
    trend?: string;
    cumulative_gain_30d?: number;
    volume_avg_30d?: number;
    volatility_30d?: number;

    price_vs_sector?: number;
    alert_overbought?: boolean;
    alert_oversold?: boolean;

    constructor(init?: Partial<IStockInsights>) {
        Object.assign(this, init);
    }
}