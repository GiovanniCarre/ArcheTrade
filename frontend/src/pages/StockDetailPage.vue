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
      <LineChart :data="chartData" :options="chartOptions" />
    </div>

    <div id="insightWrapper"></div>

    <div class="actionBoutton">
      <button>Compare with</button>
      <button>Predict the future of the action</button>
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

  switch(selectedMetric.value) {
    case 'close':
      chartData.value.datasets = [{
        type: 'line',
        label: 'Prix de clôture',
        data: allPoints.map(p => p.close),
        borderColor: 'purple',
        backgroundColor: 'rgba(0,0,255,0.1)',
        tension: 0.3,
        yAxisID: 'y'
      }];
      break;
    case 'open':
      chartData.value.datasets = [{
        type: 'line',
        label: 'Prix d\'ouverture',
        data: allPoints.map(p => p.open),
        borderColor: 'cyan',
        backgroundColor: 'rgba(0,255,255,0.1)',
        tension: 0.3,
        yAxisID: 'y'
      }];
      break;
    case 'high':
      chartData.value.datasets = [{
        type: 'line',
        label: 'Haut',
        data: allPoints.map(p => p.high),
        borderColor: 'green',
        backgroundColor: 'rgba(0,255,0,0.1)',
        tension: 0.3,
        yAxisID: 'y'
      }];
      break;
    case 'low':
      chartData.value.datasets = [{
        type: 'line',
        label: 'Bas',
        data: allPoints.map(p => p.low),
        borderColor: 'red',
        backgroundColor: 'rgba(255,0,0,0.1)',
        tension: 0.3,
        yAxisID: 'y'
      }];
      break;
    case 'volume':
      chartData.value.datasets = [{
        type: 'bar',
        label: 'Volume',
        data: allPoints.map(p => p.volume),
        backgroundColor: 'purple',
        yAxisID: 'y'
      }];
      break;
    case 'gain':
      chartData.value.datasets = [{
        type: 'bar',
        label: 'Gain / Perte',
        data: allPoints.map(p => p.close - p.open),
        backgroundColor: allPoints.map(p => (p.close - p.open) >= 0 ? 'green' : 'red'),
        yAxisID: 'y1'
      }];
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
  color: yellow;
}

.actionBoutton button:last-child:hover {
  background-color: yellow;
}

.text-center {
  text-align: center;
  padding: 4rem 1rem;
  color: gray;
}
</style>

