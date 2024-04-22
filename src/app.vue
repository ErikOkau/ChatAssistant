<script setup lang="ts">
const bgImage = ref("https://i.imgur.com/bhVbCL6.jpeg")
import image from "https://i.imgur.com/bhVbCL6.jpeg"
</script>

<template>
  <div class="body">
    test
    <NuxtLayout>
      <NuxtPage />
    </NuxtLayout>
  </div>
    
</template>

<style lang="scss">
body {
  background-image: v-bind(image); 
  
}
</style>