<script setup lang="ts">
import Login from './components/Login.vue'
</script>

<template>
  <nav>
    <router-link to="/login">Login</router-link> |
    <router-link to="/bind-wallet">Bind Wallet</router-link>
  </nav>
  <router-view></router-view>
</template>
