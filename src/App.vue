<template>
  <a-config-provider :locale="zh">
    <div class="app-container">
      <RouterView />
    </div>
  </a-config-provider>
</template>

<script setup lang="ts">
import zh from 'ant-design-vue/es/locale/zh_CN';
</script>

<style scoped lang="scss">
.app-container {
  width: 100vw;
  height: 100vh;
  user-select: none;
}
</style>
