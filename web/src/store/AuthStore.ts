import { BrowserProvider } from 'ethers' 
import { defineStore } from 'pinia'
import { api } from '../lib/axios'

interface UserLoginResult {
  success: boolean,
  id: number | null
}

interface User {
  id: number
  username: string
}

interface AuthState {
  user: User | null
  ethAddress: string | null
}

export const useAuthStore = defineStore('auth', {
  state: (): AuthState => ({
    user: null,
    ethAddress: null
  }),
  actions: {
    async login(username: string, password: string) : Promise<boolean> {
      const res = await api.post<UserLoginResult>('/api/login', { username, pwd: password })
      if(res.data?.success){
        this.user = {
          username,
          id: res.data.id!
        } 
      }
      return !!(res.data?.success)
    },

    async logoff() {
      try {
        await api.post('/api/logoff', { logoff: true })
        this.user = null
        this.ethAddress = null
      } catch (e) {
        console.error('Logoff error:', e)
      }
    },

    async bindAddress() {
      // Wait for window.ethereum to be injected
      if (typeof window.ethereum === 'undefined') {
        throw new Error('Please install MetaMask!')
      }

      try {
        // Use ethers v6 syntax
        const provider = new BrowserProvider(window.ethereum)
        await window.ethereum.request({ method: 'eth_requestAccounts' })
        const signer = await provider.getSigner()
        const address = await signer.getAddress()

        console.log('address :>> ', address);
        // const { data } = await axios.post<{message_to_sign: string}>('/api/bind-address', { 
        //   eth_address: address 
        // })

        // const signature = await signer.signMessage(data.message_to_sign)

        // await axios.post('/api/bind-address/verify', {
        //   eth_address: address,
        //   signature
        // })

        this.ethAddress = address
      } catch (error) {
        console.error('Wallet connection error:', error)
        throw error
      }
    }
  }
})