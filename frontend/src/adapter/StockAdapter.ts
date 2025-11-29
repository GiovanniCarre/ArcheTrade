import type { GenericStockDataDTO } from "@/models/stocks/GenericStockDataDTO";
import type {IStockInsights} from "@/models/stocks/StockInsights.ts";

export function mapToGenericStockDataDTO(data: any): GenericStockDataDTO {
    return {
        symbol: data.symbol,
        provider: data.provider ?? null,
        last_update: data.last_update ?? null,
        historical_segments: (data.historical_segments ?? []).map((seg: any) => ({
            start_date: new Date(seg.start_date),
            end_date: new Date(seg.end_date),
            interval: seg.interval,
            data_points: seg.data_points.map((pt: any) => ({
                timestamp: new Date(pt.timestamp),
                open: pt.open,
                close: pt.close,
                high: pt.high,
                low: pt.low,
                volume: pt.volume,
            })),
        })),
        insights: data.insights ? {
            last_price: data.insights.last_price,
            day_change: data.insights.day_change,
            day_change_percent: data.insights.day_change_percent,
            sma_7: data.insights.sma_7,
            sma_30: data.insights.sma_30,
            ema_7: data.insights.ema_7,
            ema_30: data.insights.ema_30,
            bollinger_upper: data.insights.bollinger_upper,
            bollinger_lower: data.insights.bollinger_lower,
            rsi_14: data.insights.rsi_14,
            macd: data.insights.macd,
            atr_14: data.insights.atr_14,
            max_drawdown_30d: data.insights.max_drawdown_30d,
            trend: data.insights.trend,
            cumulative_gain_30d: data.insights.cumulative_gain_30d,
            volume_avg_30d: data.insights.volume_avg_30d,
            volatility_30d: data.insights.volatility_30d,
            price_vs_sector: data.insights.price_vs_sector,
            alert_overbought: data.insights.alert_overbought,
            alert_oversold: data.insights.alert_oversold,
        } as IStockInsights : {} as IStockInsights,
    };
}