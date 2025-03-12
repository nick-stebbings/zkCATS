import { createRouter, createWebHistory } from 'vue-router'
import Login from '../components/Login.vue'
import BindWallet from '../components/BindWallet.vue'

export const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: '/login', component: Login },
    { 
      path: '/bind-wallet', 
      component: BindWallet,
      meta: { requiresAuth: true }
    }
  ]
})