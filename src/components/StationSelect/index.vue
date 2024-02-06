<template>
  <div class="station-select-container">
    <a-select
      v-model:value="model"
      show-search
      allowClear
      :placeholder="props.placeholder"
      :style="{ width: props.width + 'px' }"
      size="small"
      :fieldNames="{ label: props.labelName, value: props.valueName }"
      :show-arrow="false"
      :filterOption="filterStation"
      :not-found-content="null"
      :options="props.options"
      @search="handlePlaceSearch"
    >
      <template #dropdownRender>
        <div class="select-dropdown-container">
          <div
            class="select-container flex click-active"
            :class="{ selected: item[props.valueName] === model }"
            v-for="item in props.options"
            :key="item[props.valueName]"
            @mousedown="handleSelect(item)"
          >
            <div class="left flex">{{ item[props.labelName] }}</div>
            <div class="right flex">{{ item[props.valueName] }}</div>
          </div>
        </div>
      </template>
    </a-select>
  </div>
</template>

<script setup lang="ts">
import { PropType, ref, watch } from 'vue';

const emit = defineEmits(['update:value', 'search', 'change']);

const props = defineProps({
  value: {
    type: String,
    default: ''
  },
  options: {
    type: Array as PropType<any[]>,
    default: () => []
  },
  width: {
    type: Number,
    default: 140
  },
  labelName: {
    type: String,
    default: 'name'
  },
  valueName: {
    type: String,
    default: 'id'
  },
  placeholder: {
    type: String,
    default: '出发地'
  },
  filterOption: {
    type: Function as PropType<(input: string, option: any) => boolean>
  }
});

const model = ref<unknown>(undefined);

// 过滤站点
const filterStation = (value: string, current: any) => {
  if (!value) {
    return true;
  }
  if (current.name?.includes(value.toLowerCase())) {
    return true;
  }
  // if (
  //   current.name?.includes(value.toLowerCase()) ||
  //   current.jianName?.includes(value.toLowerCase()) ||
  //   current.jianPin?.includes(value.toLowerCase()) ||
  //   current.quanPin?.includes(value.toLowerCase())
  // ) {
  //   return false;
  // }
  return false;
};

// 搜索地点
const handlePlaceSearch = (value: string) => {
  emit('search', value);
};

// 选中选项
const handleSelect = (current: any) => {
  model.value = current[props.valueName];
  emit('update:value', model.value);
  emit('change', current);
};

watch(
  () => props.value,
  val => {
    if (!val) {
      model.value = undefined;
      return;
    }
    model.value = val;
  },
  { immediate: true }
);
</script>

<style scoped lang="scss">
.select-container {
  height: 24px;
  padding: 0 6px;
  border-radius: 6px;
  align-items: center;
  justify-content: space-between;
  font-size: 13px;
  &:hover {
    background-color: #f5f5f5;
  }
  &.selected {
    background-color: #e9f4fe;
    font-weight: bold;
  }
}
</style>
<style lang="scss">
.select-dropdown-container {
  height: 200px;
  overflow-y: auto;
  &::-webkit-scrollbar {
    width: 4px;
  }
  &::-webkit-scrollbar-thumb {
    background-color: #7a7a7a;
    border-radius: 4px;
  }
}
</style>
