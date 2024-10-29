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

<script lang="ts">
import { defineComponent, ref, onMounted, onBeforeUnmount } from 'vue';

interface BlockData {
  block_height: number;
  timestamp: string;
  block_version: number;
  confirmations: number;
  block_hash: string;
  previous_block_hash: string;
  block_size: number;
  block_weight: number;
  difficulty: number;
  transaction_count: number;
  avg_tx_size: number;
  total_supply: number;
  block_time: number;
  block_interval: number;
  hash_rate: number;
  nonce: number;
  merkle_root: string;
}

export default defineComponent({
  name: 'ScrollBar',
  setup() {
    const boxes = ref<BlockData[]>([]); // 区块数据的类型为 BlockData 数组
    const isDown = ref(false);
    const startX = ref(0);
    const scrollLeft = ref(0);
    const selectedBox = ref<BlockData | null>(null); // 当前选择的区块信息
    const scrollBlockContainer = ref<HTMLElement | null>(null); // 参考元素的类型

    const startDrag = (e: MouseEvent) => {
      if (!scrollBlockContainer.value) return;
      isDown.value = true;
      startX.value = e.pageX - scrollBlockContainer.value.offsetLeft;
      scrollLeft.value = scrollBlockContainer.value.scrollLeft;
      scrollBlockContainer.value.style.cursor = 'grabbing';

    };

    const stopDrag = () => {
      isDown.value = false;
      scrollBlockContainer.value.style.cursor = 'grab';
    };

    const onDrag = (e: MouseEvent) => {
      if (!isDown.value || !scrollBlockContainer.value) return;
      e.preventDefault();
      const x = e.pageX - scrollBlockContainer.value.offsetLeft;
      const walk = (x - startX.value) * 2;
      scrollBlockContainer.value.scrollLeft = scrollLeft.value - walk;
    };

    const showInfo = (box: BlockData) => {
      selectedBox.value = box;
    };

    const fetchBlockData = async () => {
      const backendUrl = window.location.hostname === 'localhost' ? 'http://localhost:3030' : 'http://backend:3030';
      try {
        const response = await fetch(`${backendUrl}/block_data`);
        const data: BlockData[] = await response.json();
        boxes.value = data.slice(0, 30); // 假设返回的数组包含完整区块信息
        selectedBox.value = boxes.value.reduce((max, box) =>
            max.block_height > box.block_height ? max : box
        );
      } catch (error) {
        console.error('Failed to fetch block data:', error);
      }
    };

    const formatHashRate = (hashRate: number): string => {
      const units = ['H/s', 'kH/s', 'MH/s', 'GH/s', 'TH/s', 'PH/s', 'EH/s'];
      let unitIndex = 0;
      while (hashRate >= 1000 && unitIndex < units.length - 1) {
        hashRate /= 1000;
        unitIndex++;
      }
      return `${hashRate.toFixed(2)} ${units[unitIndex]}`;
    };

    let intervalId: number | undefined;

    onMounted(() => {
      fetchBlockData();
      intervalId = setInterval(fetchBlockData, 60000);
    });

    onBeforeUnmount(() => {
      if (intervalId) {
        clearInterval(intervalId);
      }
    });

    return {
      boxes,
      isDown,
      startDrag,
      stopDrag,
      onDrag,
      showInfo,
      selectedBox,
      scrollBlockContainer,
      formatHashRate,
    };
  },
});
</script>

<style scoped>
.scroll-container {
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
  scrollbar-width: none;
}

.scroll-block-container::-webkit-scrollbar {
  display: none;
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
  cursor: pointer;
}

.info-container {
  margin-top: 20px;
  padding: 20px;
  background-color: #ffffff;
  border-radius: 10px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.05);
  text-align: left;
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 15px;
  line-height: 1.6;
}

.info-container p {
  margin: 0;
  font-size: 14px;
  color: #333;
}

.info-container p span.label {
  font-weight: 600;
  color: #555;
}

.info-container p span.value {
  font-weight: 400;
  color: #000;
}
</style>
