<template>
  <template v-if="props.record.businessSeat === '没有票了'">--</template>
  <div v-else class="column-container flex-center">
    <div class="top-line line flex-center warn" v-if="record.businessSeat === '候补'">候补</div>
    <div class="top-line line flex-center success" v-else-if="record.businessSeat === '有'">有</div>
    <div class="top-line line flex-center" v-else>2</div>
    <div class="bottom-line line flex-center warn">￥1873.00</div>
  </div>
</template>

<script setup lang="ts">
const props = defineProps({
  record: {
    type: Object,
    default: () => ({})
  },
  // 列名
  field: {
    type: String,
    default: ''
  }
});
</script>

<style scoped lang="scss">
.warn {
  color: $warningColor;
}
.success {
  color: $successColor;
}
.line {
  line-height: 14px;
}
.bottom-line {
  margin-top: 2px;
}
</style>
