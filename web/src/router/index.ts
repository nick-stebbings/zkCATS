import { createRouter, createWebHistory } from 'vue-router'
import Login from '../components/Login.vue'
import BindWallet from '../components/BindWallet.vue'
import { useAuthStore } from '../store/AuthStore'

export const router = createRouter({
  history: createWebHistory(),
  routes: [
    { 
      path: '/', 
      redirect: () => {
        const auth = useAuthStore()
        return auth.user ? '/bind-wallet' : '/login'
      }
    },
    { 
      path: '/login',    
      name: 'login',
      component: Login
    },
    { 
      path: '/bind-wallet',    
      name: 'bind-wallet', 
      component: BindWallet,
      meta: { requiresAuth: true }
    }
  ]
})

router.beforeEach(async (to, _from, next) => {
  const authStore = useAuthStore()
  
  // If route requires auth
  if (to.meta.requiresAuth) {
    // Check if user is logged in
    if (!authStore.user) {
      // Redirect to login with return path
      return next({ 
        path: '/login', 
        query: { redirect: to.fullPath }
      })
    }
  }

  // If going to login while already authenticated
  if (to.path === '/login' && authStore.user) {
    return next({ path: '/bind-wallet' })
  }

  next()
})