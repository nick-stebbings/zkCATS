<!-- src/components/Login.vue -->
<template>
  <div class="login">
    <h2>Login</h2>
    <form @submit.prevent="handleLogin">
      <input v-model="username" type="username" placeholder="Username" required>
      <input v-model="password" type="password" placeholder="Password" required>
      <button type="submit">Login</button>
    </form>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue'
import { useAuthStore } from '../store/AuthStore'

export default defineComponent({
  name: 'Login',
  setup() {
    const auth = useAuthStore()
    const username = ref('')
    const password = ref('')

    const handleLogin = async () => {
      try {
        await auth.login(username.value, password.value)
      } catch (e) {
        console.error(e)
      }
    }

    return {
      username,
      password,
      handleLogin
    }
  }
})
</script>