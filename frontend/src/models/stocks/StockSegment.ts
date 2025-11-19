import type { StockPoint } from './StockPoint';

export interface StockSegment {
    start_date: string;
    end_date: string;
    interval: string;
    data_points: StockPoint[];
}