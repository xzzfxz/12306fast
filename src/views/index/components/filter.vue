<template>
  <div class="filter-container">
    <div class="filter-item-container flex">
      <div class="filter-label no-shrink">
        车次类型：
        <span class="all click-active" :class="{ selected: !trainType.length }">全部</span>
      </div>
      <div class="filter-content flex">
        <a-checkbox-group v-model:value="trainType">
          <div class="checkbox-item" v-for="item in trainTypeList" :key="item.id">
            <a-checkbox :value="item.id">{{ item.name }}</a-checkbox>
          </div>
        </a-checkbox-group>
      </div>
    </div>
    <div class="filter-item-container flex">
      <div class="filter-label no-shrink">
        出发车站：
        <span class="all click-active" :class="{ selected: !startPlace.length }">全部</span>
      </div>
      <div class="filter-content flex">
        <a-checkbox-group v-model:value="startPlace">
          <div class="checkbox-item" v-for="item in props.startPlaceList" :key="item.id">
            <a-checkbox :value="item.id">{{ item.name }}</a-checkbox>
          </div>
        </a-checkbox-group>
      </div>
    </div>
    <div class="filter-item-container flex">
      <div class="filter-label no-shrink">
        到达车站：
        <span class="all click-active" :class="{ selected: !endPlace.length }">全部</span>
      </div>
      <div class="filter-content flex">
        <a-checkbox-group v-model:value="endPlace">
          <div class="checkbox-item" v-for="item in endPlaceList" :key="item.id">
            <a-checkbox :value="item.id">{{ item.name }}</a-checkbox>
          </div>
        </a-checkbox-group>
      </div>
    </div>
    <div class="filter-item-container flex">
      <div class="filter-label no-shrink">
        车次席别：
        <span class="all click-active" :class="{ selected: !seatType.length }">全部</span>
      </div>
      <div class="filter-content flex">
        <a-checkbox-group v-model:value="seatType">
          <div class="checkbox-item" v-for="item in seatTypeList" :key="item.id">
            <a-checkbox :value="item.id">{{ item.name }}</a-checkbox>
          </div>
        </a-checkbox-group>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { PropType, ref } from 'vue';
import { FilterItem } from '@/interface';

const props = defineProps({
  startPlaceList: {
    type: Array as PropType<FilterItem[]>,
    default: () => []
  },
  endPlaceList: {
    type: Array as PropType<FilterItem[]>,
    default: () => []
  }
});

const trainType = ref([]);
const trainTypeList = ref<FilterItem[]>([
  { id: 'GC', name: 'GC-高铁/城际' },
  { id: 'D', name: 'D-动车' },
  { id: 'Z', name: 'Z-直达' },
  { id: 'T', name: 'T-特快' },
  { id: 'K', name: 'K-快速' },
  { id: '其他', name: '其他' },
  { id: '复兴号', name: '复兴号' },
  { id: '智能动车组', name: '智能动车组' }
]);

const startPlace = ref([]);
const endPlace = ref([]);

const seatType = ref([]);
const seatTypeList = ref<FilterItem[]>([
  { id: '商务座', name: '商务座' },
  { id: '一等座', name: '一等座' },
  { id: '二等座', name: '二等座' },
  { id: '高级软卧', name: '高级软卧' },
  { id: '软卧', name: '软卧' },
  { id: '动卧', name: '动卧' },
  { id: '硬卧', name: '硬卧' },
  { id: '软座', name: '软座' },
  { id: '硬座', name: '硬座' },
  { id: '无座', name: '无座' },
  { id: '其他', name: '其他' }
]);
</script>

<style lang="scss" scoped>
.filter-container {
  margin: $paddingWidth;
  padding: 5px;
  border: 1px solid $primaryColor;
  border-radius: 4px;
  font-size: 12px;
}
.filter-item-container {
  align-items: center;
  .filter-label {
    font-weight: bold;
    .all {
      padding: 0 4px;
      border: 1px solid $normalBorderColor;
      border-radius: 4px;
      display: inline-block;
      color: $normalBorderColor;
      line-height: 16px;
      &.selected {
        background-color: $primaryColor;
        color: #ffffff;
      }
    }
  }
}
.filter-content {
  margin-left: 16px;
  .checkbox-item {
    width: 100px;
  }
  :deep(.ant-checkbox-wrapper) {
    width: 100%;
    .ant-checkbox + span {
      width: calc(100% - 20px);
      padding-right: 0;
      overflow: hidden;
      white-space: nowrap;
      text-overflow: ellipsis;
      font-size: 12px;
    }
  }
}
</style>
