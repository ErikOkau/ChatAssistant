<script setup lang="ts">
import { invoke } from '@tauri-apps/api'

const message = ref('')

onMounted(async () => {
    message.value = await invoke('greet')
})
</script>

<template>
    <h1>Hello World!</h1>
    <p>{{ message }}</p>
</template>

<style scoped lang="scss"></style>