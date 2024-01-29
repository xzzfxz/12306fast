<template>
  <div class="flex layout-container">
    <div class="login-header">
      <LoginHeader />
    </div>
    <div class="router-container"><RouterView /></div>
  </div>
</template>

<script setup lang="ts">
import LoginHeader from './components/loginHeader.vue';
</script>

<style lang="scss" scoped>
.layout-container {
  height: 100%;
  flex-direction: column;
  .router-container {
    height: 0;
    flex-grow: 1;
  }
}
</style>
