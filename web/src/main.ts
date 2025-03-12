import { createApp } from 'vue'
import { createPinia } from 'pinia'

import {
  Quasar,
  QLayout,
  QHeader,
  QToolbar,
  QToolbarTitle,
  QTabs,
  QTab,
  QBtn,
  QPageContainer,
  QCard,
  QCardSection,
  QForm,
  QInput,
  QIcon,
  QSpinner,
  QPage,
  Notify
} from 'quasar';
import App from './App.vue'
import { router } from './router/index'

// Import Quasar css
import '@quasar/extras/material-icons/material-icons.css'
import 'quasar/dist/quasar.css'

const app = createApp(App)
app.use(createPinia())
app.use(router)
app.use(Quasar, {
  plugins: { Notify },
  components: {
    QLayout,
    QHeader,
    QToolbar,
    QToolbarTitle,
    QTabs,
    QTab,
    QBtn,
    QPageContainer,
    QCard,
    QCardSection,
    QForm,
    QInput,
    QIcon,
    QSpinner,
    QPage
  },
  config: {
    notify: {
      position: 'top',
      timeout: 2500
    }
  }
})
app.mount('#app')