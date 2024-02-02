<template>
  <div class="index-container flex">
    <div class="no-shrink">
      <Search />
    </div>
    <div class="no-shrink">
      <Filter />
    </div>
  </div>
</template>

<script setup lang="ts">
import Search from './components/search.vue';
import Filter from './components/filter.vue';
</script>

<style lang="scss" scoped>
.index-container {
  height: 100%;
  flex-direction: column;
}
</style>
