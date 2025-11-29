<template xmlns="http://www.w3.org/1999/html">
  <div>
    <section id="contentPage">
        <h1 id="title">Analyze, Compare and Predict stocks with AI</h1>
        <p id="description">
          Discover the best stocks, optimize your portfolio and make data-driven decisions with our AI-powered tools.
        </p>
      <div id="searchWrapper">
          <div class="search-bar" @keydown.down.prevent="moveDown" @keydown.up.prevent="moveUp" @keydown.enter.prevent="enterKey">
            <div id="searchInputWrapper">
              <input
                  type="text"
                  placeholder="Search stock ticker..."
                  v-model="query"
                  @input="onInput"
              />
              <button id="searchButton" @click="selectedStock && selectStock(selectedStock)" :disabled="!selectedStock">
                Analyze
              </button>
            </div>

              <ul v-if="filteredStocks.length" class="suggestions">
                <li
                    v-for="(stock, index) in filteredStocks"
                    :key="stock.symbol"
                    :class="{ 'active': index === highlightedIndex }"
                    @mousedown.prevent="selectStock(stock)"
                >
                  {{ stock.symbol }} - {{ stock.name }}
                </li>

              </ul>
          </div>

        <p v-if="searchUnavailable" class="search-error">
          Service de recherche indisponible - réessayez plus tard.
        </p>
      </div>
    </section>
  </div>
</template>


<script setup lang="ts">
import { computed, ref } from 'vue'
import { StockService } from '../services/StockService.ts'
import type { StockSummary } from '../models/stocks/StockSummary.ts'

const query = ref('')
const stockSearch = ref<StockSummary[]>([])
const selectedStock = ref<StockSummary | null>(null)
const highlightedIndex = ref(-1)

const searchUnavailable = ref(false)
import { useRouter } from 'vue-router'
const router = useRouter()

const filteredStocks = computed(() => {
  if (!query.value) return []
  return stockSearch.value
})

const stockService = new StockService()

const searchStock = async () => {
  if (!query.value) return
  try {
    searchUnavailable.value = false
    stockSearch.value = await stockService.searchStocks(query.value)
    selectedStock.value = null
  } catch (err) {
    console.error(err)
    searchUnavailable.value = true
  }
}

const onInput = async () => {
  await searchStock()
}

const moveDown = () => {
  if (highlightedIndex.value < filteredStocks.value.length - 1) {
    highlightedIndex.value++
  }
}

const moveUp = () => {
  if (highlightedIndex.value > 0) {
    highlightedIndex.value--
  }
}

const enterKey = () => {
  const stock = filteredStocks.value[highlightedIndex.value]
  if (stock) {
    selectStock(stock)
  } else {
    searchStock()
  }
}

const selectStock = (stock: StockSummary) => {
  selectedStock.value = stock
  query.value = `${stock.symbol} - ${stock.name}`
  stockSearch.value = []
  highlightedIndex.value = -1

  router.push({ path: '/stock', query: { symbol: stock.symbol } })
}
</script>

<style scoped>
#contentPage {
  background: mediumpurple;
  color: white;
  text-align: center;
  padding: 6rem 1rem;
}

#title {
  font-size: 3rem;
  font-weight: bold;
  margin-bottom: 1rem;
}

#description {
  font-size: 1.2rem;
  margin-bottom: 2rem;
}

#searchWrapper{
  text-align: center;
  vertical-align: middle;
  display: inline-block;
}

.search-bar {
  display: inline-block;
}

#searchButton {
  height: 2.5rem;
  background-color: white;
  color: purple;
  border-radius: 8px;
  border: none;
  cursor: pointer;
  transition: all 0.3s;
  font-weight: bolder;
  font-size:1rem;
  padding: 0 0.5rem;
}

.search-bar input {
  border-radius: 8px;
  height: 2.5rem;
  text-align: center;
  color:purple;
  border: none;
  font-weight: bolder;
  font-size:1rem;
  width: 40rem;
}

.search-bar button:hover {
  background-color: white;
}

.suggestions{
  background-color:white;
  border:solid black 2px;
  border-radius: 2rem;
  list-style: none;
  padding: 0;
  width: 40rem;
  overflow: hidden;
}

.suggestions li {
  font-size:1rem;
  padding: 0.5rem 1rem;
  cursor: pointer;
  color:black;
}

.suggestions li.active,
.suggestions li:hover {
  background-color: purple;
  color: white;
}

#searchInputWrapper{
  display: flex;
  gap:2rem;
  align-items: center
}

.search-error {
  margin-top: 1rem;
  background: brown;
  color: pink;
  padding: 0.75rem 1.25rem;
  font-weight: 600;
}

</style>
