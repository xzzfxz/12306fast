<template>
  <div class="header-container flex">
    <div class="left-container flex">
      <div class="router-item click-active flex-center active">抢票</div>
      <div class="router-item click-active flex-center">历史查询</div>
    </div>
    <div class="login click-active" @click="handleShowLogin">登录</div>
  </div>
  <Login v-if="showLogin" @close="handleCloseLogin" />
</template>

<script setup lang="ts">
import Login from '@/views/login/index.vue';
import { ref } from 'vue';

const showLogin = ref(false);

// 打开登录弹窗
const handleShowLogin = () => {
  showLogin.value = true;
};

// 关闭登录弹窗
const handleCloseLogin = () => {
  showLogin.value = false;
};
</script>

<style lang="scss" scoped>
.header-container {
  padding: 0 $paddingWidth;
  margin-top: 2px;
  justify-content: space-between;
  align-items: center;
  font-size: 14px;
}
.left-container {
  align-items: center;
  .router-item {
    width: 100px;
    border: 1px solid $normalBorderColor;
    &.active {
      color: $primaryColor;
      border-color: $primaryColor;
    }
  }
}
.login {
  color: $primaryColor;
}
</style>
