<script setup lang="ts">
// 这里暂时不需要添加任何 TypeScript 逻辑
</script>

<template>
  <footer class="footer">
    <div class="footer-container">
      <nav class="footer-nav">
        <a href="#">Privacy Policy</a>
        <a href="#">Terms of Service</a>
        <a href="#">Contact us</a>
      </nav>
    </div>
  </footer>
</template>

<style scoped>
.footer {
  background-color: lightgray;
  width: 100%;
  padding: 30px 0;
  text-align: center;
  font-family: Arial, Helvetica, sans-serif;
  box-shadow: 0 -2px 5px rgba(0, 0, 0, 0.2);
}

.footer-container {
  max-width: 1200px;
  margin: 0 auto;
  padding: 0 20px;
  display: flex;
  justify-content: center;
  align-items: center;
}

.footer-nav {
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
}

.footer-nav a {
  color: black;
  font-size: 16px;
  transition: color 0.3s ease;
  padding: 8px 12px;
  border-radius: 4px;
  background-color: transparent;
  text-decoration: none;
}

.footer-nav a:hover {
  color: lightcoral;
  background-color: rgba(255, 255, 255, 0.1);
  text-decoration: none;
}


</style>
