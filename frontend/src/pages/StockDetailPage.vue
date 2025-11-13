<template>
  <Header />

    <p>Aucune donnée pour ce stock.</p>

  <Footer />
</template>

<script setup lang="ts">
import { onMounted } from "vue";
import { useRoute, useRouter } from "vue-router";
import Header from '../components/shared/Header.vue';
import Footer from '../components/shared/Footer.vue';
import { StockService } from "../services/StockService";

const route = useRoute();
const router = useRouter();

const stockService = new StockService();

onMounted(async () => {
  const symbol = route.query.symbol as string;

  if (!symbol) {
    console.warn("Pas de symbol dans l'URL");
    router.push("/");
    return;
  }

  try {
    await stockService.getStockInfo(symbol);
  } catch (err) {
    console.error("Erreur lors de la récupération du stock :", err);
  }
});
</script>

<style scoped>
</style>
