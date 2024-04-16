import './assets/main.css'
import { createMemoryHistory, createRouter } from 'vue-router'

import LibraryView from './components/Library.vue'
import BookView from './components/Books.vue'

const routes = [
  { path: '/', component: LibraryView },
  { path: '/books/:isbn', component: BookView }
]

const router = createRouter({
  history: createMemoryHistory(),
  routes,
}) 


import { createApp } from 'vue'
import App from './App.vue'

createApp(App).use(router).mount('#app')
