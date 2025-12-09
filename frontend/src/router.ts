import { createRouter as _createRouter, createWebHistory } from 'vue-router';
import HomePage from './pages/HomePage.vue';
import StockDetailPage from './pages/StockDetailPage.vue';
import PredictPage from "@/pages/PredictPage.vue";

export function createRouter() {
    return _createRouter({
        history: createWebHistory(),
        routes: [
            { path: '/archetrade/', component: HomePage },
            { path: '/stock', component: StockDetailPage },
            { path: '/predict', component: PredictPage }
        ],
    });
}
