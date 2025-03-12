<!-- src/App.vue -->
<template>
  <nav>
    <template v-if="!auth.user">
      <router-link to="/login">Login</router-link>
    </template>
    <template v-else>
      <router-link to="/bind-wallet">Bind Wallet</router-link> | 
      <a href="#" @click.prevent="handleLogoff">Log Off</a>
    </template>
  </nav>
  <router-view></router-view>
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from './store/AuthStore'

export default defineComponent({
  setup() {
    const router = useRouter()
    const auth = useAuthStore()

    const handleLogoff = async () => {
      await auth.logoff()
      router.replace('/login')
    }

    return {
      auth,
      handleLogoff
    }
  }
})
</script>