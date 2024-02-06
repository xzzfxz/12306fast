<template>
  <div class="search-container flex">
    <div class="search-item flex">
      <div class="filter-label">出发地：</div>
      <div class="input-container">
        <StationSelect
          v-model:value="startPlace"
          :options="startPlaceList"
          :filterOption="filterStation"
        />
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
          allowClear
          placeholder="出发地"
          style="width: 140px"
          size="small"
          :fieldNames="{ label: 'name', value: 'id' }"
          :show-arrow="false"
          :filter-option="false"
          :not-found-content="null"
          :options="endPlaceList"
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
import { EventName } from '@/const/eventName';
import { invoke } from '@tauri-apps/api';
import { Station } from '@/interface';
import StationSelect from '@/components/StationSelect/index.vue';

const commonStationList = ref<Station[]>([]);
// const allStationList = ref<Station[]>([]);

const startPlace = ref('');
const startPlaceList = ref<Station[]>([]);

const endPlace = ref('');
const endPlaceList = ref<Station[]>([]);

const date = ref(dayjs().format('YYYY-MM-DD'));

// 过滤站点
const filterStation = (value: string, current: Station) => {
  if (!value) {
    return true;
  }
  if (
    current.name?.includes(value.toLowerCase()) ||
    current.jianName?.includes(value.toLowerCase()) ||
    current.jianPin?.includes(value.toLowerCase()) ||
    current.quanPin?.includes(value.toLowerCase())
  ) {
    return true;
  }
  return false;
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

// 获取常用站点
const getCommonStations = async () => {
  const list: Station[] = await invoke(EventName.GET_COMMON_STATIONS);
  commonStationList.value = list;
  startPlaceList.value = list;
  endPlaceList.value = list;
};

// 初始化事件
const initData = () => {
  getCommonStations();
};
initData();
</script>

<style lang="scss" scoped></style>
