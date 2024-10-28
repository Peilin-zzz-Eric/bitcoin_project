<template>

  <div ref="PriceMonthChart" class="PriceMonthChart"></div>

</template>

<script>
import * as echarts from 'echarts';

export default {
  name: 'PriceMonthChart',
  data() {
    return {
      chartData: [],
      chart: null,
    };
  },
  mounted() {
    this.fetchDataAndRenderChart();
    // 每 1 分钟刷新一次数据
    setInterval(() => {
      this.fetchDataAndRenderChart();
    }, 10000); //
  },
  methods: {
    async fetchDataAndRenderChart() {
      try {
        const response = await fetch('http://127.0.0.1:3030/price_month');
        const data = await response.json();
        this.chartData = data;
        this.renderChart();
      } catch (error) {
        console.error('Error fetching data:', error);
      }
    },
    renderChart() {
      if (!this.chart) {
        this.chart = echarts.init(this.$refs.PriceMonthChart);
      }
      const prices = this.chartData.map(item => item.price);
      const minPrice = Math.min(...prices) * 0.95;
      const adjustedMinPrice = Math.round(minPrice / 1000) * 1000;

      const option = {
        title: {
          text: 'Bitcoin Price (Last month)',
        },
        tooltip: {
          trigger: 'item',
        },
        xAxis: {
          type: 'category',
          data: this.chartData.map(item => item.timestamp),
        },
        yAxis: {
          type: 'value',
          min: adjustedMinPrice, // Minimum value for the y-axis
          axisLabel: {
            formatter: '${value}', // Add the dollar symbol to the y-axis labels
          },
        },
        series: [
          {
            data: this.chartData.map(item => item.price),
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

      this.chart.setOption(option);
    },
  },
};
</script>

<style scoped>
.PriceMonthChart {
  width: 80%;
  height: 500px;
}
</style>
