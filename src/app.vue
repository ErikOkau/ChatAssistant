<script setup lang="ts">

</script>

<template>
  <div class="bg-img">
    <NuxtLayout>
      <NuxtPage />
    </NuxtLayout>
  </div>
</template>

<style lang="scss">

@import url('https://fonts.googleapis.com/css2?family=Inter:wght@100..900&display=swap');

* {
  padding: none;
  margin: none;
  font-family: 'Inter', sans-serif;
}

body, html {
  width: 100%;
  height: 100%;
  margin: 0;
  padding: 0;
  overflow: hidden; 
}

.bg-img {
  background-image: url(~/assets/img/Bg_component.png);
  background-repeat: no-repeat;
  background-attachment: fixed;
  
  background-size: cover;
  width: 100%;
  height: 100%;
}
</style>