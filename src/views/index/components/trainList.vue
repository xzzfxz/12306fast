<template>
  <div class="train-list-container">
    <div class="table-container">
      <a-table :columns="columnList" :data-source="tableData" bordered :pagination="false">
        <template #headerCell="{ column }">
          <template v-if="column.dataIndex === 'departureStation'">
            <div class="table-header column-container flex-center">
              <div class="top-line line flex-center">出发站</div>
              <div class="bottom-line line flex-center">到达站</div>
            </div>
          </template>
          <template v-else-if="column.dataIndex === 'departureTime'">
            <div class="table-header column-container flex-center">
              <div class="top-line line flex-center">出发时间</div>
              <div class="bottom-line line flex-center">到达时间</div>
            </div>
          </template>
          <template v-else-if="column.dataIndex === 'businessSeat'">
            <div class="table-header column-container flex-center">
              <div class="top-line line flex-center">商务座</div>
              <div class="bottom-line line flex-center">特等座</div>
            </div>
          </template>
          <template v-else-if="column.dataIndex === 'secondClassSeat'">
            <div class="table-header column-container flex-center">
              <div class="top-line line flex-center">二等座</div>
              <div class="bottom-line line flex-center">二等包座</div>
            </div>
          </template>
          <template v-else-if="column.dataIndex === 'softSleeper'">
            <div class="table-header column-container flex-center">
              <div class="top-line line flex-center">软卧</div>
              <div class="bottom-line line flex-center">一等卧</div>
            </div>
          </template>
          <template v-else-if="column.dataIndex === 'hardSleeper'">
            <div class="table-header column-container flex-center">
              <div class="top-line line flex-center">硬卧</div>
              <div class="bottom-line line flex-center">二等卧</div>
            </div>
          </template>
        </template>
        <template #bodyCell="{ column, text, record }">
          <template v-if="column.dataIndex === 'trainNumber'">
            <div class="column-container flex-center">
              <div class="top-line line flex-center">
                <a class="train">{{ text }}</a>
              </div>
              <div class="bottom-line line train-line flex-center">
                <div class="mark flex-center">智</div>
                <div class="mark flex-center warn">复</div>
                <div class="mark flex-center primary">静</div>
              </div>
            </div>
          </template>
          <template v-else-if="column.dataIndex === 'departureStation'">
            <div class="column-container flex-center">
              <div class="top-line line flex-center">北京</div>
              <div class="bottom-line line flex-center">上海</div>
            </div>
          </template>
          <template v-else-if="column.dataIndex === 'departureTime'">
            <div class="column-container flex-center">
              <div class="top-line line flex-center">8:00</div>
              <div class="bottom-line line flex-center">15:00</div>
            </div>
          </template>
          <template v-else-if="column.dataIndex === 'duration'">
            <div class="column-container flex-center">
              <div class="top-line line flex-center">8:00</div>
              <div class="bottom-line line flex-center">当日到达</div>
            </div>
          </template>
          <template
            v-else-if="
              [
                'businessSeat',
                'firstClassSeat',
                'secondClassSeat',
                'superSoftSleeper',
                'softSleeper',
                'movingSleeper',
                'hardSleeper',
                'softSeat',
                'hardSeat',
                'noSeat',
                'other'
              ].includes(column.dataIndex)
            "
          >
            <SeatTemplate :record="record" :field="column.dataIndex" />
          </template>
          <template v-else-if="column.dataIndex === 'order'">
            <div class="column-container flex-center">
              <!-- <div class="order-btn click-active">预定</div> -->
              <div class="order-btn cancel click-active">取消</div>
            </div>
          </template>
        </template>
      </a-table>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import SeatTemplate from './seatTemplate.vue';
import { COLUMN_LIST } from '../utils/index';

const columnList = ref(COLUMN_LIST);

const tableData = ref([
  {
    trainNumber: 'G1234',
    departureStation: '北京南 - 上海虹桥',
    departureTime: '08:00 - 15:30',
    duration: '7小时30分',
    businessSeat: '2',
    firstClassSeat: '有',
    secondClassSeat: '有',
    superSoftSleeper: '有',
    softSleeper: '有',
    movingSleeper: '有',
    hardSleeper: '有',
    softSeat: '有',
    hardSeat: '有',
    noSeat: '有',
    other: '有',
    order: '预定'
  }
]);
</script>

<style lang="scss" scoped>
.train-list-container {
  padding: $paddingWidth;
}
.table-container {
  border: 1px solid $borderColor;
  border-radius: $radiusWidth;
  overflow: hidden;
}
.column-container {
  flex-direction: column;
  &.table-header {
    .line {
      line-height: 16px;
    }
  }
  .line {
    line-height: 14px;
  }
  .train {
    font-size: 14px;
    font-weight: bold;
  }
  .bottom-line {
    margin-top: 2px;
  }
  .train-line {
    width: 100%;
    font-size: 12px;
    .mark {
      padding: 1px;
      margin-left: 4px;
      border: 1px solid $successColor;
      color: $successColor;
      font-size: 12px;
      line-height: 12px;
      &:first-child {
        margin-left: 0;
      }
      &.warn {
        border-color: $warningColor;
        color: $warningColor;
      }
      &.primary {
        border-color: $primaryColor;
        color: $primaryColor;
      }
    }
  }
}
.order-btn {
  color: $primaryColor;
  text-decoration: underline;
  &.cancel {
    color: $borderColor;
  }
}
</style>
