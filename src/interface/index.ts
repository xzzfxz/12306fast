/**
 * @description: 车次过滤选项
 * @return {*}
 */
export interface FilterItem {
  id: string;
  name: string;
}

// 车站信息
export interface Station {
  jianPin: string;
  name: string;
  id: string;
  areaCode: number;
  quanPin: string;
  jianName: string;
}
