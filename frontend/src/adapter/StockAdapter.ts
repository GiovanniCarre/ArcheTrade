import type { GenericStockDataDTO } from "@/models/stocks/GenericStockDataDTO";

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
    };
}
