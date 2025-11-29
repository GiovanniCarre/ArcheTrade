<template>
  <section v-if="stock" id="predictPageWrapper">
    <div v-if="loading" class="loader-wrapper">
      <div class="loader"></div>
      <p>Calcul des prédictions...</p>
    </div>

    <div v-else>
      <h1>Prédiction pour {{ stock.symbol }}</h1>

      <div class="method-wrapper">
        <select v-model="selectedMethod" @change="fetchPrediction" class="method-select">
          <option v-for="m in methods" :key="m" :value="m">{{ m }}</option>
        </select>
      </div>

      <div id="chartWrapper">
        <LineChart :data="chartData" :options="chartOptions" />
      </div>
    </div>

  </section>

  <div v-else class="text-center p-10">
    <p>Aucun stock sélectionné pour la prédiction.</p>
    <button @click="router.push('/')" class="btn-return">Retour</button>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useRouter } from "vue-router";
import { Line } from "vue-chartjs";

import {
  Chart as ChartJS,
  Title,
  Tooltip,
  Legend,
  LinearScale,
  CategoryScale,
  PointElement,
  LineElement,
  Filler
} from "chart.js";

import type { GenericStockDataDTO } from "@/models/stocks/GenericStockDataDTO";
import type { PredictionPoint } from "@/models/stocks/PredictionPoint";
import { StockService } from "@/services/StockService";

// Register Chart.js modules
ChartJS.register(
    Title,
    Tooltip,
    Legend,
    LinearScale,
    CategoryScale,
    PointElement,
    LineElement,
    Filler
);

const LineChart = Line;
const router = useRouter();
const stockService = new StockService();

const stock = ref<GenericStockDataDTO | null>(null);
const selectedMethod = ref("LSTM");
const loading = ref(false);

const methods = ["NAIVE", "SMA", "EMA", "LINEAR_REGRESSION", "AR"];

const chartData = ref({
  labels: [] as string[],
  datasets: [
    {
      label: "Historique",
      data: [] as (number | null)[],
      borderColor: "blue",
      backgroundColor: "rgba(0,0,255,0.15)",
      tension: 0.3,
      pointRadius: 0,
      spanGaps: false
    },
    {
      label: "Prédiction future",
      data: [] as (number | null)[],
      borderDash: [5, 5],
      borderColor: "purple",
      backgroundColor: "rgba(150, 0, 255, 0.15)",
      tension: 0.3,
      pointRadius: 0,
      spanGaps: false
    },
    {
      label: "Incertitude haute",
      data: [] as (number | null)[],
      borderWidth: 0,
      fill: "+1",
      backgroundColor: "rgba(180, 90, 255, 0.18)",
      spanGaps: false
    },
    {
      label: "Incertitude basse",
      data: [] as (number | null)[],
      borderWidth: 0,
      spanGaps: false
    }
  ]
});

const chartOptions = ref({
  animation: {
    duration: 1400,
    easing: "easeOutQuart",
  },
  responsive: true,
  plugins: {
    legend: { display: true }
  },
  spanGaps: false
});

onMounted(() => {
  const raw = localStorage.getItem("selectedStock");
  try {
    stock.value = raw ? JSON.parse(raw) : null;
  } catch {
    stock.value = null;
  }

  if (!stock.value) return;

  loadHistorical();
  fetchPrediction();
});

// Load historical
function loadHistorical() {
  const points = stock.value!.historical_segments
      .flatMap(s => s.data_points)
      .sort((a, b) => new Date(a.timestamp).getTime() - new Date(b.timestamp).getTime());

  chartData.value.labels = points.map(p =>
      new Date(p.timestamp).toLocaleDateString("fr-FR")
  );

  // Historique
  chartData.value.datasets[0].data = points.map(p => p.close);

  // Préparer les datasets prédiction à null
  const histLen = chartData.value.labels.length;
  chartData.value.datasets[1].data = Array(histLen).fill(null);
  chartData.value.datasets[2].data = Array(histLen).fill(null);
  chartData.value.datasets[3].data = Array(histLen).fill(null);
}

function resetPredictionData() {
  const histLen = chartData.value.datasets[0].data.length;
  chartData.value.datasets[1].data = Array(histLen).fill(null);
  chartData.value.datasets[2].data = Array(histLen).fill(null);
  chartData.value.datasets[3].data = Array(histLen).fill(null);
  chartData.value.labels = chartData.value.labels.slice(0, histLen);
}

async function fetchPrediction() {
  if (!stock.value) return;

  loading.value = true;

  const result: PredictionPoint[] =
      await stockService.predictStock(stock.value.historical_segments, selectedMethod.value);

  // Debug : points passés
  const now = new Date();
  const hasPastPoints = result.some(p => new Date(p.timestamp) <= now);
  console.log("Points dans le passé présents ?", hasPastPoints);
  console.log("Prédictions reçues :", result);

  // Reset avant d'appliquer nouvelle prédiction
  resetPredictionData();
  applyPredictionAligned(result);

  loading.value = false;
}

/**
 * Aligne les tableaux : on prepend des `null` pour la longueur historique
 * puis on concatène les valeurs de prédiction.
 */
function applyPredictionAligned(predPoints: PredictionPoint[]) {
  const histLabels = [...chartData.value.labels];
  const histLen = histLabels.length;

  const futurLabels = predPoints.map(p => new Date(p.timestamp).toLocaleDateString("fr-FR"));
  const futurData = predPoints.map(p => p.close);
  const upperData = predPoints.map(p => p.upper);
  const lowerData = predPoints.map(p => p.lower);

  const prefix = Array(histLen).fill(null);

  chartData.value.datasets[1].data = [...prefix, ...futurData];
  chartData.value.datasets[2].data = [...prefix, ...upperData];
  chartData.value.datasets[3].data = [...prefix, ...lowerData];

  chartData.value.labels = [...histLabels, ...futurLabels];

  // Rafraîchissement Vue/Chart
  chartData.value = JSON.parse(JSON.stringify(chartData.value));
}
</script>

<style scoped>
#predictPageWrapper {
  max-width: 900px;
  margin: 2rem auto;
  padding: 1.5rem;
  background: white;
  border-radius: 1rem;
  box-shadow: 0 4px 14px rgba(0,0,0,0.08);
}

.method-wrapper {
  margin: 1.5rem 0;
  display: flex;
  align-items: center;
  gap: 1rem;
}

.method-select {
  padding: 0.6rem 1rem;
  background: #f7edff;
  border: 2px solid #a66cff;
  border-radius: 10px;
  color: #6a00d7;
  font-weight: 600;
  cursor: pointer;
}

#chartWrapper {
  margin-top: 2rem;
  height: 420px;
}

.loader-wrapper {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 3rem;
  gap: 1rem;
}

.loader {
  width: 55px;
  height: 55px;
  border: 6px solid #e6d4ff;
  border-top-color: #7a00ff;
  border-radius: 50%;
  animation: spin 0.9s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.btn-return {
  margin-top: 1rem;
  padding: .6rem 1rem;
  border-radius: 6px;
  background: purple;
  color: white;
  cursor: pointer;
}
</style>
