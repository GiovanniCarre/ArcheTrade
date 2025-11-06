import type { StockSummary } from "../models/StockSummary";

const API_BASE = import.meta.env.VITE_BACKEND_URL;

export class StockService {
    private baseUrl: string;

    constructor(baseUrl: string = API_BASE) {
        this.baseUrl = baseUrl;
    }

    async searchStocks(query: string): Promise<StockSummary[]> {
        const url = `${this.baseUrl}/stocks/search?query=${encodeURIComponent(query)}`;
        const res = await fetch(url);

        if (!res.ok) {
            throw new Error(`Erreur API: ${res.status}`);
        }

        const data: StockSummary[] = await res.json();
        return data;
    }
}