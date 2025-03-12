<!-- src/components/Login.vue -->
<template>
  <div class="row justify-center items-center" style="min-height: 80vh;">
    <q-card class="login-card q-pa-lg">
      <q-card-section class="text-center">
        <h4 class="text-h4 q-mb-md">Welcome to zkCATS</h4>
        <p class="text-subtitle1 text-grey-7">Please login to continue</p>
      </q-card-section>

      <q-card-section>
        <q-form @submit="handleLogin" class="q-gutter-md">
          <q-input
            v-model="username"
            label="Username"
            filled
            dense
            class="q-px-md"
            :rules="[val => !!val || 'Username is required']"
            lazy-rules
          >
            <template v-slot:prepend>
              <q-icon name="person" />
            </template>
          </q-input>

          <q-input
            v-model="password"
            label="Password"
            filled
            dense
            class="q-px-md"
            :type="isPwd ? 'password' : 'text'"
            :rules="[val => !!val || 'Password is required']"
            lazy-rules
          >
            <template v-slot:prepend>
              <q-icon name="lock" />
            </template>
            <template v-slot:append>
              <q-icon
                :name="isPwd ? 'visibility_off' : 'visibility'"
                class="cursor-pointer"
                @click="isPwd = !isPwd"
              />
            </template>
          </q-input>

          <div class="row justify-center q-mt-md">
            <q-btn
              type="submit"
              color="primary"
              label="Login"
              :loading="loading"
              class="full-width"
            />
          </div>
        </q-form>
      </q-card-section>
    </q-card>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue'
import { useAuthStore } from '../store/AuthStore'
import { router } from '../router/index'
import { showError, showSuccess } from '../lib/notify'

export default defineComponent({
  name: 'Login',
  setup() {
    const auth = useAuthStore()
    const username = ref('')
    const password = ref('')
    const isPwd = ref(true)
    const loading = ref(false)

    const handleLogin = async () => {
      loading.value = true
      try {
        const loginResult = await auth.login(username.value, password.value)
        if(loginResult) {
          showSuccess('Login successful')
          router.push('/bind-wallet')
        }
      } catch (e) {
        if (e.response?.status === 403) {
          showError('Invalid username or password')
        } else {
          console.error(e)
          showError('Login failed. Please try again')
        }
      } finally {
        loading.value = false
      }
    }

    return {
      username,
      password,
      isPwd,
      loading,
      handleLogin
    }
  }
})
</script>

<style scoped>
.login-card {
  width: 100%;
  max-width: 400px;
}
</style>
