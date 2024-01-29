import { createApp } from 'vue';
import './assets/style/reset.scss';
import './assets/style/global.scss';
import App from './App.vue';
import router from '@/router';

createApp(App).use(router).mount('#app');
