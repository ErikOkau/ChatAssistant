<script setup lang="ts">
import TitleBar from '../components/TitleBar.vue'

</script>

<template>
    <div class="titlebar">
        <TitleBar />
    </div>

    <div class="content">
        <NuxtPage></NuxtPage>
    </div>
</template>

<style scoped lang="scss">
.titlebar {
    margin-bottom: 2rem;
}
</style>