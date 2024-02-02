<template>
  <div class="search-container flex">
    <div class="search-item flex">
      <div class="filter-label">出发地：</div>
      <div class="input-container">
        <a-select
          v-model:value="startPlace"
          show-search
          placeholder="出发地"
          style="width: 140px"
          size="small"
          :show-arrow="false"
          :filter-option="false"
          :not-found-content="null"
          :options="startPlaceList"
          @search="handlePlaceSearch($event, true)"
        ></a-select>
      </div>
    </div>
    <div class="search-item flex">
      <div class="input-container" title="交换起始点">
        <SwapOutlined class="exchange click-active" />
      </div>
    </div>
    <div class="search-item flex">
      <div class="filter-label">目的地：</div>
      <div class="input-container">
        <a-select
          v-model:value="endPlace"
          show-search
          placeholder="出发地"
          style="width: 140px"
          size="small"
          :show-arrow="false"
          :filter-option="false"
          :not-found-content="null"
          :options="endPlaceList"
          @search="handlePlaceSearch($event, false)"
        ></a-select>
      </div>
    </div>
    <div class="search-item flex">
      <div class="filter-label">日期：</div>
      <div class="input-container">
        <a-date-picker
          v-model:value="date"
          size="small"
          valueFormat="YYYY-MM-DD"
          :allowClear="false"
          :disabledDate="getDisabledDate"
        />
      </div>
    </div>
    <div class="search-item flex">
      <div class="input-container">
        <a-button type="primary" size="small">查询</a-button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { SwapOutlined } from '@ant-design/icons-vue';
import dayjs from 'dayjs';

const startPlace = ref('');
const startPlaceList = ref([]);

const endPlace = ref('');
const endPlaceList = ref([]);

const date = ref(dayjs().format('YYYY-MM-DD'));

// 搜索地点
const handlePlaceSearch = (value: string, isStart: boolean = false) => {
  console.log(value, isStart);
};

// 今日之前的日期禁用
const getDisabledDate = (current: Date) => {
  const today = dayjs(dayjs().format('YYYYMMDD'));
  const currentDate = dayjs(dayjs(current).format('YYYYMMDD'));
  if (currentDate.isSame(today)) {
    return false;
  }
  return currentDate.isBefore(dayjs());
};
</script>

<style lang="scss" scoped></style>
