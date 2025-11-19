import type { StockSegment } from './StockSegment';

export interface GenericStockDataDTO {
    symbol: string;
    provider?: string | null;
    last_update?: string | null;
    historical_segments: StockSegment[];
}