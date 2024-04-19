import './assets/main.css'
import { createMemoryHistory, createRouter } from 'vue-router'
import BookView from './components/Books.vue'
import LibraryView from './components/Library.vue'
import NewbookView from './components/Newbook.vue'


const routes = [
  { path: '/', component: LibraryView },
  { path: '/books/:isbn', component: BookView },
  { path: '/addbook', component: NewbookView }
]

const router = createRouter({
  history: createMemoryHistory(),
  routes,
}) 


import { createApp } from 'vue'
import App from './App.vue'

createApp(App).use(router).mount('#app')
