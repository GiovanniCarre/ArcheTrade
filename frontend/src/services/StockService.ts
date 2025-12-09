import type {StockSummary} from "../models/stocks/StockSummary.ts";
import type {GenericStockDataDTO} from "@/models/stocks/GenericStockDataDTO.ts";
import {mapToGenericStockDataDTO} from "@/adapter/StockAdapter.ts";
import type {PredictionPoint} from "@/models/stocks/PredictionPoint.ts";
import type {StockSegment} from "@/models/stocks/StockSegment.ts";

const API_BASE = import.meta.env.VITE_BACKEND_URL || "/api";

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
        console.log("Raw: ",JSON.stringify(rawData))
        return rawData ? mapToGenericStockDataDTO(rawData) : null;
    }

    async predictStock(history: StockSegment[], method: string): Promise<PredictionPoint[]> {
        try {
            const url = `${this.baseUrl}/stock/predict`;

            const predictionHistory: PredictionPoint[] = history.flatMap(segment =>
                segment.data_points.map(pt => ({
                    timestamp: new Date(pt.timestamp).toISOString(),
                    close: pt.close,
                    upper: pt.close,
                    lower: pt.close,
                }))
            );

            const response = await fetch(url, {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify({ method, history: predictionHistory }),
            });

            if (!response.ok) {
                throw new Error(`Erreur réseau lors de la prédiction : ${response.status}`);
            }

            const result: PredictionPoint[] = await response.json();
            console.log("Réponse prédiction:", result);
            return result;

        } catch (err) {
            console.error("Erreur predictStock:", err);
            return [];
        }
    }
}

