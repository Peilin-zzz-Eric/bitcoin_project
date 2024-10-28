<template>
  <div ref="PriceWeekChart" class="PriceWeekChart"></div>
</template>

<script lang="ts">
import { defineComponent, ref, onMounted } from 'vue';
import * as echarts from 'echarts';

// 定义接口来描述获取的数据结构
interface PriceData {
  price: number;
  timestamp: string;
}

export default defineComponent({
  name: 'PriceWeekChart',
  setup() {
    // 定义 chartData 的类型为 PriceData 数组
    const chartData = ref<PriceData[]>([]);
    const chart = ref<echarts.ECharts | null>(null);
    const PriceWeekChart = ref<HTMLDivElement | null>(null);

    // 异步获取数据并渲染图表
    const fetchDataAndRenderChart = async () => {
      try {
        const response = await fetch('http://127.0.0.1:3030/price_week');
        const data: PriceData[] = await response.json();
        chartData.value = data;
        renderChart();
      } catch (error) {
        console.error('Error fetching data:', error);
      }
    };

    const renderChart = () => {
      if (PriceWeekChart.value && !chart.value) {
        chart.value = echarts.init(PriceWeekChart.value);
      }
      if (!chart.value) return;

      const prices = chartData.value.map(item => item.price);
      const minPrice = Math.min(...prices) * 0.95;
      const adjustedMinPrice = Math.round(minPrice / 1000) * 1000;

      const option = {
        title: {
          text: 'Bitcoin Price (Last Week)',
        },
        tooltip: {
          trigger: 'item',
        },
        xAxis: {
          type: 'category',
          data: chartData.value.map(item => item.timestamp),
        },
        yAxis: {
          type: 'value',
          min: adjustedMinPrice,
          axisLabel: {
            formatter: '${value}', // 加上美元符号
          },
        },
        series: [
          {
            data: chartData.value.map(item => item.price),
            type: 'line',
            smooth: false, // 线条直线
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

    // 组件挂载时调用数据获取与渲染函数
    onMounted(() => {
      fetchDataAndRenderChart();
      // 每 10 秒刷新一次数据
      setInterval(() => {
        fetchDataAndRenderChart();
      }, 10000);
    });

    return {
      PriceWeekChart,
      chartData,
    };
  },
});
</script>

<style scoped>
.PriceWeekChart {
  width: 80%;
  height: 500px;
}
</style>
