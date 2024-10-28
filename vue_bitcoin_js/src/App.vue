<script>
import HeaderContent from "@/components/HeaderContent.vue";
import FooterContent from "@/components/FooterContent.vue";
import ChartsContainer from "@/components/ChartsContainer.vue";
import MarketData from "@/components/MarketData.vue";
import ScrollBar  from "@/components/ScrollBar.vue";
export default {
  name: 'App',
  components: {
    HeaderContent,
    FooterContent,
    ChartsContainer,
    MarketData,
    ScrollBar,
  },
};
</script>

<template>
  <div id="app">
    <HeaderContent  />
    <ChartsContainer  />
    <MarketData  />
    <ScrollBar  />
    <FooterContent  />
  </div>
</template>

<style scoped>
/* 根容器 */


#app {
  box-sizing: border-box;
  margin: 0;
  padding: 0;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;

}

</style>
