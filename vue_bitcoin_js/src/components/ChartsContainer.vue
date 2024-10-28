<template>
  <div class = "linechart-container">
    <!-- 切换控件 -->
    <div class="chart-switch">
      <button
          v-for="(label, index) in chartLabels"
          :key="index"
          :class="{ active: activeChart === index }"
          @click="activeChart = index"
      >
        {{ label }}
      </button>
    </div>

    <!-- 根据activeChart的值显示不同的图表 -->
    <div class="chart-container">
      <PriceDayChart v-if="activeChart === 0" />
      <PriceWeekChart v-if="activeChart === 1" />
      <PriceMonthChart v-if="activeChart === 2" />
      <PriceYearChart v-if="activeChart === 3" />
    </div>
  </div>
</template>

<script>
import PriceDayChart from "@/components/Charts/PriceDayChart.vue";
import PriceWeekChart from "@/components/Charts/PriceWeekChart.vue";
import PriceMonthChart from "@/components/Charts/PriceMonthChart.vue";
import PriceYearChart from "@/components/Charts/PriceYearChart.vue";
export default {
  name: "ChartContainer",
  components: {

    PriceDayChart,
    PriceWeekChart,
    PriceMonthChart,
    PriceYearChart,
  },
  data() {
    return {
      activeChart: 0, // 默认显示 chart1
      chartLabels: ["1D", "1W", "1M", "1Y"], // 标签
    };
  },
};
</script>

<style scoped>
/* 按钮组的样式 */
.linechart-container{
  margin-bottom: 50px;
  width: 100%;
}
.chart-switch {
  display: flex;
  justify-content: center;
  margin-bottom: 20px;
}

.chart-switch button {
  background-color: transparent;
  border: none;
  padding: 10px;
  margin: 0 10px;
  font-size: 16px;
  color: gray;
  cursor: pointer;
}

.chart-switch button.active {
  color: black;
  font-weight: bold;
  border-bottom: 2px solid black;
}

/* 图表容器 */
.chart-container {
  width: 100%;
  display: flex;
  justify-content: center;
}


</style>
