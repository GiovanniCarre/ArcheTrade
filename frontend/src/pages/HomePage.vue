<template>
  <div>
    <!-- Header -->
    <Header />

    <!-- Hero Section -->
    <section class="hero">
      <div class="hero-content">
        <h1 class="hero-title">Analyze, Compare & Predict Stocks with AI</h1>
        <p class="hero-desc">
          Discover the best stocks, optimize your portfolio and make data-driven decisions with our AI-powered tools.
        </p>
        <div class="search-bar">
          <input type="text" placeholder="Search stock ticker..." v-model="query" />
          <button @click="searchStock">Analyze</button>
        </div>
      </div>
    </section>

    <!-- Top Stocks Section -->
    <section class="top-stocks">
      <h2>Top Stocks of the Day</h2>
      <div class="stocks-grid">
        <div class="stock-card" v-for="stock in topStocks" :key="stock.symbol">
          <h3>{{ stock.symbol }}</h3>
          <p>{{ stock.name }}</p>
          <p :class="{'positive': stock.change >= 0, 'negative': stock.change < 0}">
            {{ stock.change >= 0 ? '+' : ''}}{{ stock.change }}%
          </p>
        </div>
      </div>
    </section>

    <!-- Footer -->
    <footer class="footer">
      <p>© 2025 MarketPulse. All rights reserved.</p>
    </footer>
  </div>
</template>

<script setup>
import Header from '../components/Header.vue'
import { ref } from 'vue'

const query = ref('')

// Dummy top stocks data
const topStocks = ref([
  { symbol: 'AAPL', name: 'Apple Inc.', change: 1.2 },
  { symbol: 'TSLA', name: 'Tesla, Inc.', change: -0.8 },
  { symbol: 'AMZN', name: 'Amazon.com, Inc.', change: 0.5 },
  { symbol: 'GOOGL', name: 'Alphabet Inc.', change: 2.1 }
])

const searchStock = () => {
  alert(`Analyzing ${query.value}... (AI feature coming soon!)`)
}
</script>

<style scoped>
/* Hero Section */
.hero {
  background: linear-gradient(135deg, #6c5ce7 0%, #a29bfe 100%);
  color: #fff;
  text-align: center;
  padding: 6rem 1rem;
  clip-path: polygon(0 0, 100% 0, 100% 85%, 0 100%); /* hexagonal-ish cut */
}
.hero-title {
  font-size: 3rem;
  font-weight: 700;
  margin-bottom: 1rem;
}
.hero-desc {
  font-size: 1.2rem;
  margin-bottom: 2rem;
}
.search-bar {
  display: flex;
  justify-content: center;
  gap: 0.5rem;
}
.search-bar input {
  padding: 0.75rem 1rem;
  border-radius: 8px;
  border: none;
  width: 300px;
  font-size: 1rem;
}
.search-bar button {
  padding: 0.75rem 1.5rem;
  background-color: #fff;
  color: #6c5ce7;
  font-weight: 700;
  border-radius: 8px;
  border: none;
  cursor: pointer;
  transition: all 0.3s;
}
.search-bar button:hover {
  background-color: #f0f0f0;
}

/* Top Stocks Section */
.top-stocks {
  max-width: 1200px;
  margin: 4rem auto;
  padding: 0 1rem;
  text-align: center;
}
.stocks-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 2rem;
  margin-top: 2rem;
}
.stock-card {
  background: #6c5ce7;
  color: #fff;
  padding: 1.5rem;
  border-radius: 12px;
  transition: transform 0.3s;
}
.stock-card:hover {
  transform: translateY(-5px);
}
.stock-card .positive {
  color: #00ff99;
  font-weight: 700;
}
.stock-card .negative {
  color: #ff4d4d;
  font-weight: 700;
}

/* Footer */
.footer {
  background-color: #2d2d2d;
  color: #fff;
  text-align: center;
  padding: 2rem 0;
  margin-top: 4rem;
}
</style>
