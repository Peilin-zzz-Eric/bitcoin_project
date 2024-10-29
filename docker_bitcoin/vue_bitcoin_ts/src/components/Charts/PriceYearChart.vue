<template>
  <div ref="PriceYearChart" class="PriceYearChart"></div>
</template>

<script lang="ts">
import { defineComponent, ref, onMounted } from 'vue';
import * as echarts from 'echarts';

// 定义接口描述数据结构
interface PriceData {
  price: number;
  timestamp: string;
}

export default defineComponent({
  name: 'PriceYearChart',
  setup() {
    // chartData 类型为 PriceData 数组
    const chartData = ref<PriceData[]>([]);
    const chart = ref<echarts.ECharts | null>(null);
    const PriceYearChart = ref<HTMLDivElement | null>(null);
    const backendUrl = window.location.hostname === 'localhost' ? 'http://localhost:3030' : 'http://backend:3030';
    // 获取数据并渲染图表
    const fetchDataAndRenderChart = async () => {
      try {
        const response = await fetch(`${backendUrl}/price_year`);
        const data: PriceData[] = await response.json();
        chartData.value = data;
        renderChart();
      } catch (error) {
        console.error('Error fetching data:', error);
      }
    };

    const renderChart = () => {
      if (PriceYearChart.value && !chart.value) {
        chart.value = echarts.init(PriceYearChart.value);
      }
      if (!chart.value) return;

      const prices = chartData.value.map(item => item.price);
      const minPrice = Math.min(...prices) * 0.95;
      const adjustedMinPrice = Math.round(minPrice / 1000) * 1000;

      const option = {
        title: {
          text: 'Bitcoin Price (Last Year)',
        },
        tooltip: {
          trigger: 'item',
        },
        xAxis: {
          type: 'category',
          data: chartData.value.map(item => item.timestamp),
          /*axisLabel: {
            rotate: 45,  // 旋转 45 度
          },*/
        },
        yAxis: {
          type: 'value',
          min: adjustedMinPrice,
          axisLabel: {
            formatter: '${value}', // 添加美元符号
          },
        },
        series: [
          {
            data: chartData.value.map(item => item.price),
            type: 'line',
            smooth: false,  // 设置 smooth 为 false 或移除此行来确保线是直线
            lineStyle: {
              width: 2,
            },
            itemStyle: {
              color: '#5470C6',
            },
          },
        ],
      };

      chart.value.setOption(option);
    };

    // 组件挂载后获取数据并渲染图表
    onMounted(() => {
      fetchDataAndRenderChart();
      // 每 10 秒刷新一次数据
      setInterval(() => {
        fetchDataAndRenderChart();
      }, 10000);
    });

    return {
      PriceYearChart,
      chartData,
    };
  },
});
</script>

<style scoped>
.PriceYearChart {
  width: 80%;
  height: 500px;
}
</style>
