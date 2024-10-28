<template>
  <div class="scroll-container">
    <!-- Scroll Container -->
    <div ref="scrollBlockContainer" class="scroll-block-container" @mousedown="startDrag" @mouseup="stopDrag" @mouseleave="stopDrag" @mousemove="onDrag">
      <div class="scroll-content_block">
        <div v-for="(box, index) in boxes" :key="index" class="box" @click="showInfo(box)">
          {{ box.block_height }}
        </div>
      </div>
    </div>

    <!-- 显示点击的块的信息 -->
    <div v-if="selectedBox" class="info-container">
      <p><span class="label">Block Height:</span> <span class="value">{{ selectedBox.block_height }}</span></p>
      <p><span class="label">Timestamp:</span> <span class="value">{{ selectedBox.timestamp }}</span></p>
      <p><span class="label">Block Version:</span> <span class="value">0x{{ selectedBox.block_version.toString(16).padStart(8, '0')}}</span></p>
      <p><span class="label">Confirmations:</span> <span class="value">{{ selectedBox.confirmations }}</span></p>
      <p><span class="label">Block Hash:</span> <span class="value">{{ selectedBox.block_hash }}</span></p>
      <p><span class="label">Previous Block Hash:</span> <span class="value">{{ selectedBox.previous_block_hash }}</span></p>
      <p><span class="label">Block Size:</span> <span class="value">{{ selectedBox.block_size }} B</span></p>
      <p><span class="label">Block Weight:</span> <span class="value">{{ selectedBox.block_weight }}</span></p>
      <p><span class="label">Difficulty:</span> <span class="value">{{ selectedBox.difficulty }}</span></p>
      <p><span class="label">Transaction Count:</span> <span class="value">{{ selectedBox.transaction_count }}</span></p>
      <p><span class="label">Average Transaction Size:</span> <span class="value">{{ selectedBox.avg_tx_size.toFixed(2) }} B</span></p>
      <p><span class="label">Total Supply:</span> <span class="value">{{ selectedBox.total_supply }}</span></p>
      <p><span class="label">Block Time:</span> <span class="value">{{ selectedBox.block_time.toFixed(2) }} min</span></p>
      <p><span class="label">Block Interval:</span> <span class="value">{{ selectedBox.block_interval.toFixed(2) }} min</span></p>
      <p><span class="label">Hash Rate:</span> <span class="value">{{ formatHashRate(selectedBox.hash_rate) }}</span></p>
      <p><span class="label">Nonce:</span> <span class="value">{{ selectedBox.nonce }}</span></p>
      <p><span class="label">Merkle Root:</span> <span class="value">{{ selectedBox.merkle_root }}</span></p>


    </div>
  </div>
</template>


<script>
export default {
  data() {
    return {
      boxes: [], // 通过 API 获取的区块数据
      isDown: false,
      startX: 0,
      scrollLeft: 0,
      selectedBox: null, // 当前点击的区块信息
      intervalId: null,  // 定时器 ID
    };
  },
  methods: {
    startDrag(e) {
      this.isDown = true;
      this.startX = e.pageX - this.$refs.scrollBlockContainer.offsetLeft;
      this.scrollLeft = this.$refs.scrollBlockContainer.scrollLeft;
    },
    stopDrag() {
      this.isDown = false;
    },
    onDrag(e) {
      if (!this.isDown) return;
      e.preventDefault();
      const x = e.pageX - this.$refs.scrollBlockContainer.offsetLeft;
      const walk = (x - this.startX) * 2;
      this.$refs.scrollBlockContainer.scrollLeft = this.scrollLeft - walk;
    },
    showInfo(box) {
      // 如果点击的是当前选择的块，则清空 selectedBox
      /*if (this.selectedBox === box) {
        this.selectedBox = null;
      } else {
        this.selectedBox = box; // 否则更新 selectedBox 为点击的块
      }*/
      this.selectedBox = box;
    },
    async fetchBlockData() {
      try {
        const response = await fetch('http://127.0.0.1:3030/block_data');
        const data = await response.json();

        // 假设 data 是一个包含多个区块信息的数组，我们只取前 30 个块的信息
        this.boxes = data.slice(0, 30); // 假设数组每个对象包含完整区块信息
        this.selectedBox = this.boxes.reduce((max, box) => max.block_height > box.block_height ? max : box);
      } catch (error) {
        console.error('Failed to fetch block data:', error);
      }
    },
    formatHashRate(hashRate) {
      const units = ['H/s', 'kH/s', 'MH/s', 'GH/s', 'TH/s', 'PH/s', 'EH/s'];
      let unitIndex = 0;

      // 持续除以1000，直到合适的单位
      while (hashRate >= 1000 && unitIndex < units.length - 1) {
        hashRate /= 1000;
        unitIndex++;
      }

      // 返回保留两位小数的格式化值
      return `${hashRate.toFixed(2)} ${units[unitIndex]}`;
    },
  },
  mounted() {
    // 初始化时立即获取数据
    this.fetchBlockData();

    // 每 1 分钟刷新一次数据
    this.intervalId = setInterval(() => {
      this.fetchBlockData();
    }, 60000); // 60000 毫秒 = 1 分钟
  },
  beforeDestroy() {
    // 清除定时器
    if (this.intervalId) {
      clearInterval(this.intervalId);
    }
  },
};
</script>


<style scoped>
.scroll-container {
  //height: 100rem;
  margin-bottom: 100px;
  width: 83%;
}
.scroll-block-container {
  width: 100%;
  max-width: 2000px;
  overflow-x: scroll;
  white-space: nowrap;
  display: flex;
  scroll-behavior: smooth;
  box-sizing: border-box;
  cursor: grab;
  scrollbar-width: none; /* 隐藏滚动条 */
}

.scroll-block-container::-webkit-scrollbar {
  display: none; /* 隐藏 Chrome, Safari 和 Edge 浏览器中的滚动条 */
}

.scroll-content_block {
  display: inline-flex;
  flex-shrink: 0;
}

.box {
  width: 80px;
  height: 80px;
  background-color: #f0a5a5;
  margin-right: 10px;
  text-align: center;
  line-height: 80px;
  border-radius: 12px;
  background: linear-gradient(145deg, #f5b5b5, #f9a2a2);
  box-shadow: 5px 5px 10px #d0d0d0, -5px -5px 10px #ffffff;
  font-size: 12px;
  user-select: none;
  cursor: pointer; /* 鼠标样式 */
}



.info-container {
  margin-top: 20px;
  padding: 20px;
  background-color: #ffffff;
  border-radius: 10px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.05); /* Light shadow for depth */
  text-align: left;
  display: grid;
  grid-template-columns: 1fr 1fr; /* Two columns layout */
  gap: 15px; /* Space between items */
  line-height: 1.6; /* Increase line height for better readability */
}

.info-container p {
  margin: 0;
  font-size: 14px;
  color: #333; /* Dark color for contrast */
}

.info-container p span.label {
  font-weight: 600; /* Semi-bold for labels */
   /* Slightly lighter color for labels */
  color: #555;
}

.info-container p span.value {
  font-weight: 400; /* Regular weight for values */
  color: #000; /* Darker color for values */

}

.info-container p:last-child {
  border-bottom: none; /* Remove border from the last item */
}


</style>
