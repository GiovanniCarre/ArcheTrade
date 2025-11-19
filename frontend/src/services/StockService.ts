import type {StockSummary} from "../models/stocks/StockSummary.ts";
import type {GenericStockDataDTO} from "@/models/stocks/GenericStockDataDTO.ts";
import {mapToGenericStockDataDTO} from "@/adapter/StockAdapter.ts";

const API_BASE = import.meta.env.VITE_BACKEND_URL || "http://localhost:3000";

export class StockService {
    private baseUrl: string;

    constructor(baseUrl: string = API_BASE) {
        this.baseUrl = baseUrl;
    }

    async searchStocks(query: string): Promise<StockSummary[]> {
        const url = `${this.baseUrl}/stocks/search?query=${encodeURIComponent(query)}`;
        const res = await fetch(url);

        if (!res.ok) {
            throw new Error(`Erreur :${res.status}`);
        }

        return await res.json();
    }

    async getStockInfo(symbol: string): Promise<GenericStockDataDTO | null> {
        const url = `${this.baseUrl}/stocks/info?symbol=${encodeURIComponent(symbol)}`;
        const res = await fetch(url);

        if (!res.ok) {
            throw new Error(`Erreur : ${res.status}`);
        }

        const rawData = await res.json();
        return rawData ? mapToGenericStockDataDTO(rawData) : null;
    }
}

