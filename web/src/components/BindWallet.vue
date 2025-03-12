<!-- src/components/BindWallet.vue -->
<script setup lang="ts">
import { useAuthStore } from '../store/AuthStore'

const auth = useAuthStore()

const handleBind = async () => {
  try {
    await auth.bindAddress()
  } catch (e) {
    console.error(e)
  }
}
handleBind()
</script>