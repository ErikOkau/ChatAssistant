<script setup lang="ts">
import { invoke } from '@tauri-apps/api'

onMounted(async () => {
    const retval = await invoke('greet')
    console.log(retval)
})
</script>

<template>
    <h1>Hello Waifus!</h1>
</template>

<style scoped lang="scss">

</style>