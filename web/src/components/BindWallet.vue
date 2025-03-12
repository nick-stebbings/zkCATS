<!-- src/components/BindWallet.vue -->
<script lang="ts">
import { defineComponent } from 'vue'
import { useAuthStore } from '../store/AuthStore'
import { showError, showSuccess } from '../lib/notify'

export default defineComponent({
  name: 'BindWallet',
  setup() {
    const auth = useAuthStore()

    const handleBind = async () => {
      try {
        await auth.bindAddress()
        showSuccess('Wallet connected successfully')
      } catch (e: any) {
        if (e.message?.includes('MetaMask')) {
          showError('Please install MetaMask to continue')
        } else if (e.code === 4001) {
          showError('Connection rejected. Please try again')
        } else {
          showError('Failed to connect wallet. Please try again')
        }
      }
    }

    handleBind()
  }
})
</script>