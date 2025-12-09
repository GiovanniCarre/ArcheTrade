<template>
  <Header />
  <router-view />
  <Footer/>
</template>

<script setup lang="ts">
import Header from "@/components/shared/Header.vue";
import Footer from "./components/shared/Footer.vue";
</script>

<style>
body {
  margin: 0;
  font-family: "Poppins", sans-serif;
  background-color: white;
}
</style>
