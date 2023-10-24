import { createRouter, createWebHashHistory} from 'vue-router'
import { routes } from './routePath'

// const routes: Array<RouteRecordRaw> = [
  
// ]

const router = createRouter({
  history: createWebHashHistory(),
  routes
})

export default router
