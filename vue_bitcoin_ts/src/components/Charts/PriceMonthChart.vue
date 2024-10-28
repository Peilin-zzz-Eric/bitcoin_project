<template>
  <div ref="PriceMonthChart" class="PriceMonthChart"></div>
</template>

<script lang="ts">
import { defineComponent, ref, onMounted } from 'vue';
import * as echarts from 'echarts';

// 定义接口以明确获取的数据结构
interface PriceData {
  price: number;
  timestamp: string;
}

export default defineComponent({
  name: 'PriceMonthChart',
  setup() {
    // chartData 类型为 PriceData 数组
    const chartData = ref<PriceData[]>([]);
    const chart = ref<echarts.ECharts | null>(null);
    const PriceMonthChart = ref<HTMLDivElement | null>(null);

    // 获取数据并渲染图表
    const fetchDataAndRenderChart = async () => {
      try {
        const response = await fetch('http://127.0.0.1:3030/price_month');
        const data: PriceData[] = await response.json();
        chartData.value = data;
        renderChart();
      } catch (error) {
        console.error('Error fetching data:', error);
      }
    };

    const renderChart = () => {
      if (PriceMonthChart.value && !chart.value) {
        chart.value = echarts.init(PriceMonthChart.value);
      }
      if (!chart.value) return;

      const prices = chartData.value.map(item => item.price);
      const minPrice = Math.min(...prices) * 0.95;
      const adjustedMinPrice = Math.round(minPrice / 1000) * 1000;

      const option = {
        title: {
          text: 'Bitcoin Price (Last Month)',
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
            formatter: '${value}', // 添加美元符号
          },
        },
        series: [
          {
            data: chartData.value.map(item => item.price),
            type: 'line',
            smooth: false,
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
      PriceMonthChart,
      chartData,
    };
  },
});
</script>

<style scoped>
.PriceMonthChart {
  width: 80%;
  height: 500px;
}
</style>
