// src/lib/notify.ts
import { Notify } from 'quasar'

export const showError = (message: string) => {
  Notify.create({
    type: 'negative',
    message,
    icon: 'error'
  })
}

export const showSuccess = (message: string) => {
  Notify.create({
    type: 'positive',
    message,
    icon: 'check'
  })
}