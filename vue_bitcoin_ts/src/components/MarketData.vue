<template>
  <div class="market-info-container">
    <h2>Market Info</h2>
    <div class="info-grid">
      <div>
        <span>Market Cap</span>
        <p>{{ formatCurrency(data.market_cap) }}</p>
      </div>
      <div>
        <span>Diluted Market Cap</span>
        <p>{{ formatCurrency(data.diluted_market_cap) }}</p>
      </div>
      <div>
        <span>Vol 24h</span>
        <p>{{ formatCurrency(data.volume_24h) }}</p>
      </div>
      <div>
        <span>Vol / Market Cap</span>
        <p>{{ data.vol_market_cap_ratio.toFixed(2) }}</p>
      </div>
      <div>
        <span>24h Change</span>
        <p :class="{ red: data.price_change_24h < 0, green: data.price_change_24h >= 0 }">{{ data.price_change_24h.toFixed(2) }}%</p>
      </div>
      <div>
        <span>24h High</span>
        <p>{{ formatCurrency(data.high_24h) }}</p>
      </div>
      <div>
        <span>24h Low</span>
        <p>{{ formatCurrency(data.low_24h) }}</p>
      </div>
      <div>
        <span>Circulating Supply</span>
        <p>{{ formatNumber(data.circulating_supply) }}</p>
      </div>
      <div>
        <span>Max Supply</span>
        <p>{{ formatNumber(data.max_supply) }}</p>
      </div>
      <div>
        <span>Last Updated</span>
        <p>{{ data.last_updated }}</p>
      </div>
      <div>
        <span>Genesis Block Date</span>
        <p>{{ data.genesis_block_date }}</p>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

// 定义接口描述市场数据的结构
interface MarketData {
  market_cap: number;
  diluted_market_cap: number;
  volume_24h: number;
  vol_market_cap_ratio: number;
  high_24h: number;
  low_24h: number;
  price_change_24h: number;
  circulating_supply: number;
  max_supply: number;
  genesis_block_date: string;
  last_updated: string;
}

export default defineComponent({
  name: "MarketInfo",
  data() {
    return {
      data: {
        market_cap: 0,
        diluted_market_cap: 0,
        volume_24h: 0,
        vol_market_cap_ratio: 0,
        high_24h: 0,
        low_24h: 0,
        price_change_24h: 0,
        circulating_supply: 0,
        max_supply: 0,
        genesis_block_date: "",
        last_updated: "",
      } as MarketData, // 将数据对象类型声明为 MarketData
    };
  },
  mounted() {
    this.fetchData();
    setInterval(this.fetchData, 60000); // 每分钟刷新一次数据
  },
  methods: {
    // 获取市场数据
    async fetchData() {
      try {
        const response = await fetch("http://127.0.0.1:3030/market_data");
        const result: MarketData = await response.json(); // 使用 MarketData 类型推断

        // 使用响应中的数据更新页面显示
        this.data = result;
      } catch (error) {
        console.error("Error fetching market data:", error);
      }
    },
    // 格式化货币
    formatCurrency(value: number): string {
      return new Intl.NumberFormat("en-US", {
        style: "currency",
        currency: "USD",
      }).format(value);
    },
    // 格式化数字
    formatNumber(value: number): string {
      return new Intl.NumberFormat("en-US").format(value);
    },
  },
});
</script>

<style scoped>
.market-info-container {
  font-family: Arial, sans-serif;
  padding: 25px;
  background: white;
  border-radius: 12px;
  box-shadow: 0px 4px 12px rgba(0, 0, 0, 0.1);
  width: 80%;
  margin-bottom: 100px;
}

h2 {
  color: #333;
  font-size: 25px;
  margin-top: 10px;
  margin-bottom: 40px;
  display: flex;
  justify-content: center;
}

.info-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 30px;
  width: 100%;
}

.info-grid div {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  font-size: 14px;
}

.info-grid div p {
  font-size: 18px;
  font-weight: bold;
  color: #333;
}

.red {
  color: red !important;
}

.green {
  color: green !important;
}

.info-grid div span {
  font-size: 12px;
  color: #777;
}
</style>
