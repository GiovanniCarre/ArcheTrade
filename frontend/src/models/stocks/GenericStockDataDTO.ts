import type { StockSegment } from './StockSegment';
import type {IStockInsights} from "@/models/stocks/StockInsights.ts";

export interface GenericStockDataDTO {
    symbol: string;
    provider?: string | null;
    last_update?: string | null;
    historical_segments: StockSegment[];
    insights: IStockInsights;
}