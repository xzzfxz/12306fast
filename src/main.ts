import { createApp } from 'vue';
import './assets/style/reset.scss';
import './assets/style/global.scss';
import App from './App.vue';
import router from '@/router';
import useAntD from './utils/antd';
// import { Modal, Form, Input, Button, Checkbox } from 'ant-design-vue';

createApp(App).use(useAntD).use(router).mount('#app');
