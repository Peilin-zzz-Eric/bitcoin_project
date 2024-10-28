<script setup lang="ts">

</script>

<template>
  <header class="header">
    <div class="header-container">
      <h1>Bitcoin Chain Metrics</h1>
    </div>
  </header>
</template>

<style scoped>

.header {
  font-family: Arial, Helvetica, sans-serif;
  background-color: lightslategray;
  color: #fff;
  padding: 30px 0;
  text-align: center;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
  position: sticky;
  //position: fixed;

  top: 0;
  z-index: 1000;
  width: 100%;
  margin-bottom: 50px;
}

.header h1 {
  margin: 0;
  font-size: 3rem;
  font-weight: 600;
  letter-spacing: 1px;
  color: #ffc107;
  transition: transform 0.3s ease;

}

.header h1:hover {
  transform: scale(1.05);
}

.header-container {
  max-width: 1200px;
  margin: 0 auto;
  padding: 0 20px;
}


</style>
