<template>
  <div ref="PriceDayChart" class="PriceDayChart"></div>
</template>

<script lang="ts">
import { defineComponent, ref, onMounted } from 'vue';
import * as echarts from 'echarts';

// 定义接口，明确数据结构
interface PriceData {
  price: number;
  timestamp: string;
}

export default defineComponent({
  name: 'PriceDayChart',
  setup() {
    // 定义chartData的类型为 PriceData 数组
    const chartData = ref<PriceData[]>([]);
    const chart = ref<echarts.ECharts | null>(null);
    const PriceDayChart = ref<HTMLDivElement | null>(null);

    // 数据获取与图表渲染函数
    const fetchDataAndRenderChart = async () => {
      try {
        const response = await fetch('http://127.0.0.1:3030/price_day');
        const data: PriceData[] = await response.json();
        chartData.value = data;
        renderChart();
      } catch (error) {
        console.error('Error fetching data:', error);
      }
    };

    const renderChart = () => {
      if (PriceDayChart.value && !chart.value) {
        chart.value = echarts.init(PriceDayChart.value);
      }
      if (!chart.value) return;

      const prices = chartData.value.map(item => item.price);
      const minPrice = Math.min(...prices) * 0.95;
      const adjustedMinPrice = Math.round(minPrice / 1000) * 1000;

      const option = {
        title: {
          text: 'Bitcoin Price (Last 24 Hours)',
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
            formatter: '${value}', // Add the dollar symbol to the y-axis labels
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

    // 生命周期钩子
    onMounted(() => {
      fetchDataAndRenderChart();
      // 每 10 秒刷新一次数据
      setInterval(() => {
        fetchDataAndRenderChart();
      }, 10000);
    });

    return {
      PriceDayChart,
      chartData,
    };
  },
});
</script>

<style scoped>
.PriceDayChart {
  width: 80%;
  height: 500px;
}
</style>
