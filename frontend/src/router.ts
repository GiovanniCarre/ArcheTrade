import { createRouter as _createRouter, createWebHistory } from 'vue-router';
import HomePage from './pages/HomePage.vue';
import StockDetailPage from './pages/StockDetailPage.vue';

export function createRouter() {
    return _createRouter({
        history: createWebHistory(),
        routes: [
            { path: '/', component: HomePage },
            { path: '/stock', component: StockDetailPage }
        ],
    });
}
