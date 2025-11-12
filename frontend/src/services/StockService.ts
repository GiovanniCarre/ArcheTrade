import type {StockSummary} from "../models/StockSummary";

// ❌ plus besoin de dotenv
// ✅ utilise import.meta.env à la place
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
}
