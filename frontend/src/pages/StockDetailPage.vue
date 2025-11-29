<template>
  <div v-if="!stockData || !chartDataInitialized" class="text-center p-10">
    <p>Aucune donnée pour ce stock.</p>
  </div>

  <section v-else id="stockInfosWrapper">

    <div id="generalInfos">
      <h1>{{ stockData.symbol }}</h1>
      <p v-if="stockData.provider">Par {{ stockData.provider }}</p>
      <p>Mis à jour le {{ stockData.last_update ? stockData.last_update.toLocaleString('fr-FR') : '-' }}</p>
    </div>

    <div id="metricSelector">
      <label for="metric">Choisir une métrique : </label>
      <select id="metric" v-model="selectedMetric">
        <option value="close">Prix de clôture</option>
        <option value="open">Prix d'ouverture</option>
        <option value="high">Haut</option>
        <option value="low">Bas</option>
        <option value="volume">Volume</option>
        <option value="gain">Gain / Perte</option>
      </select>
    </div>

    <div id="graphiqueSection">
      <LineChart ref="lineChart" :data="chartData" :options="chartOptions" />
    </div>

    <div id="insightWrapper" v-if="stockData?.insights">
      <h2 class="insight-title">Insights du stock</h2>
      <div class="insights-grid">
        <div class="insight-card" title="Dernier prix du stock">
          <strong>Dernier prix</strong>
          <p>{{ formatNumber(stockData.insights.last_price) }}</p>
        </div>
        <div class="insight-card" title="Variation du jour">
          <strong>Variation du jour</strong>
          <p>{{ formatNumber(stockData.insights.day_change) }}</p>
        </div>
        <div class="insight-card" title="% Variation du jour">
          <strong>% Variation</strong>
          <p>{{ formatNumber(stockData.insights.day_change_percent) }}</p>
        </div>
        <div class="insight-card" title="Moyenne mobile simple sur 7 jours">
          <strong>SMA 7</strong>
          <p>{{ formatNumber(stockData.insights.sma_7) }}</p>
        </div>
        <div class="insight-card" title="Moyenne mobile simple sur 30 jours">
          <strong>SMA 30</strong>
          <p>{{ formatNumber(stockData.insights.sma_30) }}</p>
        </div>
        <div class="insight-card" title="Moyenne mobile exponentielle sur 7 jours">
          <strong>EMA 7</strong>
          <p>{{ formatNumber(stockData.insights.ema_7) }}</p>
        </div>
        <div class="insight-card" title="Moyenne mobile exponentielle sur 30 jours">
          <strong>EMA 30</strong>
          <p>{{ formatNumber(stockData.insights.ema_30) }}</p>
        </div>
        <div class="insight-card" title="Bollinger supérieur">
          <strong>Bollinger ↑</strong>
          <p>{{ formatNumber(stockData.insights.bollinger_upper) }}</p>
        </div>
        <div class="insight-card" title="Bollinger inférieur">
          <strong>Bollinger ↓</strong>
          <p>{{ formatNumber(stockData.insights.bollinger_lower) }}</p>
        </div>
        <div class="insight-card" title="RSI sur 14 jours">
          <strong>RSI 14</strong>
          <p>{{ formatNumber(stockData.insights.rsi_14) }}</p>
        </div>
        <div class="insight-card" title="MACD">
          <strong>MACD</strong>
          <p>{{ formatNumber(stockData.insights.macd) }}</p>
        </div>
        <div class="insight-card" title="ATR sur 14 jours">
          <strong>ATR 14</strong>
          <p>{{ formatNumber(stockData.insights.atr_14) }}</p>
        </div>
        <div class="insight-card" title="Max drawdown sur 30 jours">
          <strong>Max Drawdown 30j</strong>
          <p>{{ formatNumber(stockData.insights.max_drawdown_30d) }}</p>
        </div>
        <div class="insight-card" title="Tendance actuelle">
          <strong>Tendance</strong>
          <p>{{ stockData.insights.trend ?? '-' }}</p>
        </div>
        <div class="insight-card" title="Gain cumulatif sur 30 jours">
          <strong>Gain 30j</strong>
          <p>{{ formatNumber(stockData.insights.cumulative_gain_30d) }}</p>
        </div>
        <div class="insight-card" title="Volume moyen sur 30 jours">
          <strong>Volume 30j</strong>
          <p>{{ formatNumber(stockData.insights.volume_avg_30d) }}</p>
        </div>
        <div class="insight-card" title="Volatilité sur 30 jours">
          <strong>Volatilité 30j</strong>
          <p>{{ formatNumber(stockData.insights.volatility_30d) }}</p>
        </div>
        <div class="insight-card" title="Prix vs secteur">
          <strong>Prix vs Secteur</strong>
          <p>{{ formatNumber(stockData.insights.price_vs_sector) }}</p>
        </div>
        <div class="insight-card" title="Alerte surachat">
          <strong>Surachat</strong>
          <p>{{ stockData.insights.alert_overbought ? 'Oui' : 'Non' }}</p>
        </div>
        <div class="insight-card" title="Alerte survente">
          <strong>Survente</strong>
          <p>{{ stockData.insights.alert_oversold ? 'Oui' : 'Non' }}</p>
        </div>
      </div>
    </div>


    <div class="actionBoutton">
      <button @click="goToPredict">Predict the future of the action</button>
    </div>

  </section>
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from "vue";
import { useRoute, useRouter } from "vue-router";
import { StockService } from "../services/StockService";
import type { GenericStockDataDTO } from "@/models/stocks/GenericStockDataDTO.ts";

import {
  Chart as ChartJS,
  Title,
  Tooltip,
  Legend,
  LineElement,
  BarElement,
  CategoryScale,
  LinearScale,
  PointElement,
  BarController,
  LineController
} from "chart.js";
import { Line } from "vue-chartjs";
import type {StockPoint} from "@/models/stocks/StockPoint.ts";
import type {StockSegment} from "@/models/stocks/StockSegment.ts";

ChartJS.register(
    Title,
    Tooltip,
    Legend,
    LineElement,
    BarElement,
    LineController,
    BarController,
    CategoryScale,
    LinearScale,
    PointElement
);

const LineChart = Line;

const route = useRoute();
const router = useRouter();
const stockService = new StockService();

const stockData = ref<GenericStockDataDTO | null>(null);

const selectedMetric = ref<'close'|'open'|'high'|'low'|'volume'|'gain'>('close');

function formatNumber(value: number | undefined | null, digits = 4): string {
  if (value === undefined || value === null) return '-';
  return value.toFixed(digits);
}

const chartData = ref({
  labels: [] as string[],
  datasets: [
    {
      label: "Prix",
      type: 'line' as const,
      data: [] as number[],
      borderColor: "blue",
      backgroundColor: "rgba(0,0,255,0.1)",
      tension: 0.3,
      yAxisID: 'y',
    }
  ]
});

function goToPredict() {
  if (stockData.value) {

    localStorage.setItem("selectedStock", JSON.stringify(stockData.value));
    router.push({path: '/predict',});
  }
}

const chartOptions = ref({
  responsive: true,
  interaction: { mode: 'index' as const, intersect: false },
  stacked: false,
  plugins: { legend: { display: true }, tooltip: { enabled: true } },
  scales: {
    y: { type: 'linear', position: 'left', title: { display: true, text: 'Prix / Volume' } },
    y1: { type: 'linear', position: 'right', title: { display: true, text: 'Gain / Perte' }, grid: { drawOnChartArea: false } }
  }
});
const chartDataInitialized = ref(false);
function updateChartData() {
  if (!stockData.value) return;

  const allPoints: StockPoint[] = stockData.value.historical_segments.flatMap(
      (seg: StockSegment) => seg.data_points
  );
  allPoints.sort((a, b) => new Date(a.timestamp).getTime() - new Date(b.timestamp).getTime());
  chartData.value.labels = allPoints.map(p => new Date(p.timestamp).toLocaleDateString('fr-FR'));

  switch (selectedMetric.value) {
    case 'close':
      chartData.value = {
        labels: allPoints.map(p => new Date(p.timestamp).toLocaleDateString('fr-FR')),
        datasets: [{
          type: 'line',
          label: 'Prix de clôture',
          data: allPoints.map(p => p.close),
          borderColor: 'purple',
          backgroundColor: 'rgba(0,0,255,0.1)',
          tension: 0.3,
          yAxisID: 'y'
        }]
      };
      break;

    case 'open':
      chartData.value = {
        labels: allPoints.map(p => new Date(p.timestamp).toLocaleDateString('fr-FR')),
        datasets: [{
          type: 'line',
          label: 'Prix d\'ouverture',
          data: allPoints.map(p => p.open),
          borderColor: 'cyan',
          backgroundColor: 'rgba(0,255,255,0.1)',
          tension: 0.3,
          yAxisID: 'y'
        }]
      };
      break;

    case 'high':
      chartData.value = {
        labels: allPoints.map(p => new Date(p.timestamp).toLocaleDateString('fr-FR')),
        datasets: [{
          type: 'line',
          label: 'Haut',
          data: allPoints.map(p => p.high),
          borderColor: 'green',
          backgroundColor: 'rgba(0,255,0,0.1)',
          tension: 0.3,
          yAxisID: 'y'
        }]
      };
      break;

    case 'low':
      chartData.value = {
        labels: allPoints.map(p => new Date(p.timestamp).toLocaleDateString('fr-FR')),
        datasets: [{
          type: 'line',
          label: 'Bas',
          data: allPoints.map(p => p.low),
          borderColor: 'red',
          backgroundColor: 'rgba(255,0,0,0.1)',
          tension: 0.3,
          yAxisID: 'y'
        }]
      };
      break;

    case 'volume':
      chartData.value = {
        labels: allPoints.map(p => new Date(p.timestamp).toLocaleDateString('fr-FR')),
        datasets: [{
          type: 'bar',
          label: 'Volume',
          data: allPoints.map(p => p.volume),
          backgroundColor: 'purple',
          yAxisID: 'y'
        }]
      };
      break;

    case 'gain':
      chartData.value = {
        labels: allPoints.map(p => new Date(p.timestamp).toLocaleDateString('fr-FR')),
        datasets: [{
          type: 'bar',
          label: 'Gain / Perte',
          data: allPoints.map(p => p.close - p.open),
          backgroundColor: allPoints.map(p => (p.close - p.open) >= 0 ? 'green' : 'red'),
          yAxisID: 'y1'
        }]
      };
      break;

    default:
      console.warn("⛔ Métrique inconnue :", selectedMetric.value);
      chartData.value = {
        labels: [],
        datasets: [{
          type: 'line',
          label: 'Inconnu',
          data: [],
          borderColor: 'gray',
          backgroundColor: 'rgba(128,128,128,0.1)',
          yAxisID: 'y'
        }]
      };
      break;
  }
}

watch([selectedMetric, stockData], updateChartData);

onMounted(async () => {
  const symbol = route.query.symbol as string;
  if (!symbol) { router.push("/"); return; }

  try {
    stockData.value = await stockService.getStockInfo(symbol);
    chartDataInitialized.value = true;
  } catch (err) {
    console.error("Erreur lors de la récupération du stock :", err);
  }
});
</script>

<style scoped>
#stockInfosWrapper {
  max-width: 900px;
  margin: 2rem auto;
  padding: 1rem;
  background-color: white;
  border-radius: 1rem;
}

#generalInfos {
  text-align: center;
  margin-bottom: 2rem;
}

#generalInfos h1 {
  font-size: 2rem;
  margin-bottom: 0.5rem;
  color: purple;
}

#generalInfos p {
  font-size: 1rem;
  color: gray;
}

#metricSelector {
  display: flex;
  justify-content: center;
  align-items: center;
  margin-bottom: 1.5rem;
  gap: 0.5rem;
}

#metricSelector label {
  font-weight: bold;
  color: gray;
}

#metricSelector select {
  padding: 0.4rem 0.6rem;
  border-radius: 6px;
  border: 1px solid white;
  font-size: 1rem;
}

#graphiqueSection {
  width: 100%;
  height: 400px;
  margin-bottom: 2rem;
}

#insightWrapper {
  padding: 1rem;
  background-color: white;
  border-radius: 8px;
  margin-bottom: 2rem;
  text-align: center;
  color: purple;
  font-weight: 500;
}

.actionBoutton {
  display: flex;
  justify-content: center;
  gap: 1rem;
}

.actionBoutton button {
  padding: 0.6rem 1.2rem;
  border: none;
  border-radius: 8px;
  font-weight: bold;
  cursor: pointer;
  transition: background-color 0.2s ease;
}

.actionBoutton button:first-child {
  background-color: purple;
  color: white;
}

.actionBoutton button:first-child:hover {
  background-color: purple;
}

.actionBoutton button:last-child {
  color: white;
  border-radius:1rem;
  border: 2px solid purple;
}

.actionBoutton button:last-child:hover {
  background-color: transparent;
  color:purple;
}

.text-center {
  text-align: center;
  padding: 4rem 1rem;
  color: gray;
}

#insightWrapper {
  margin-bottom: 2rem;
}

.insight-title {
  text-align: center;
  font-size: 1.5rem;
  font-weight: bold;
  color: purple;
  margin-bottom: 1rem;
}

.insights-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(150px, 1fr));
  gap: 1rem;
}

.insight-card {
  background-color: white;
  border: 1px solid white;
  border-radius: 8px;
  padding: 0.8rem;
  text-align: center;
  color: purple;
  transition: transform 0.2s ease;
}

.insight-card:hover {
  transform: translateY(-3px);
  box-shadow: 0 4px 8px rgba(0,0,0,0.1);
}

.insight-card strong {
  display: block;
  font-size: 0.9rem;
  margin-bottom: 0.3rem;
}

</style>

