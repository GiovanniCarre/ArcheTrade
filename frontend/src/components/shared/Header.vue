<template>
  <header class="header">
    <div class="container">
      <router-link to="/">
        <img :src="logo" alt="MarketPulse Logo" class="logo" />
      </router-link>

      <nav>
        <ul class="nav-list">
          <li><router-link to="/">Home</router-link></li>
          <li><a href="#">Features</a></li>
          <li><a href="#">Pricing</a></li>
          <li><a href="#">Contact</a></li>
        </ul>
      </nav>
    </div>
  </header>
</template>

<script setup>
import logo from '../../assets/archeTrade.png'
</script>

<style scoped>
.header {
  background-color: #6c5ce7; /* violet premium */
  color: #fff;
  padding: 1rem 0;
  box-shadow: 0 4px 10px rgba(0,0,0,0.1);
}
.container {
  max-width: 1200px;
  margin: 0 auto;
  display: flex;
  align-items: center;
  justify-content: space-between;
}
.logo {
  height: 50px;
}
.nav-list {
  list-style: none;
  display: flex;
  gap: 2rem;
}
.nav-list a {
  color: #fff;
  text-decoration: none;
  font-weight: 600;
}
.nav-list a:hover {
  text-decoration: underline;
}
</style>
