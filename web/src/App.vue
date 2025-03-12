<!-- src/App.vue -->
<template>
  <q-layout view="hHh lpR fFf">
    <q-header elevated class="bg-primary text-white">
      <q-toolbar>
        <q-toolbar-title>
          zkCATS
        </q-toolbar-title>
        <q-tabs v-model="tab" align="right">
          <template v-if="!auth.user && route.path !== '/login'">
            <q-tab name="login" to="/login" label="Login" />
          </template>
          <template v-if="auth.user">
            <q-tab 
              v-if="route.path !== '/bind-wallet'"
              name="bind-wallet" 
              to="/bind-wallet" 
              label="Bind Wallet" 
            />
            <q-btn 
              flat 
              label="Log Off" 
              @click="handleLogoff" 
              class="q-ml-sm" 
            />
          </template>
        </q-tabs>
      </q-toolbar>
    </q-header>

    <q-page-container>
      <router-view />
    </q-page-container>
  </q-layout>
</template>

<script lang="ts">
import { defineComponent, computed, ref } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { useAuthStore } from './store/AuthStore'
import { showError, showSuccess } from './lib/notify'

export default defineComponent({
  setup() {
    const router = useRouter()
    const route = useRoute()
    const auth = useAuthStore()
    const tab = ref(null)

    const handleLogoff = async () => {
      try {
        await auth.logoff();
        showSuccess('Logged out successfully');
        router.replace('/login');
      } catch (e: any) {
        console.error(e);
        showError('Error logging off!');
      }
    }

    return {
      auth,
      handleLogoff,
      tab,
      route
    }
  }
})
</script>

<style>
.q-toolbar {
  min-height: 50px;
}
</style>