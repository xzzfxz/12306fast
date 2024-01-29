import { RouteRecordRaw, createRouter, createWebHistory } from 'vue-router';
import Layout from '@/layout/index/index.vue';
import Index from '@/views/index/index.vue';

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'indexLayout',
    component: Layout,
    redirect: '/index',
    children: [
      {
        path: 'index',
        name: 'index',
        component: Index,
      },
    ],
  },
];

export const router = createRouter({
  routes,
  history: createWebHistory(),
});

export default router;
