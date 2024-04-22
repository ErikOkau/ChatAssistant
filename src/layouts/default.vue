<script setup lang="ts">
import { appWindow } from '@tauri-apps/api/window'

const drawer = ref(false)

const minimize = () => appWindow.minimize()
const maximize = () => appWindow.toggleMaximize()
const close = () => appWindow.close()

</script>

<template>
    <QLayout view="lHh lpR lFf">
        <QHeader elevated class="bg-primary text-white">
            <div class="titlebar">
                <QSpace data-tauri-drag-region/>
                <QBtn flat dense round icon="remove" color="primary" @click="minimize" />
                <QBtn flat dense round icon="crop_square" color="primary" @click="maximize" />
                <QBtn flat dense round icon="close" color="primary" @click="close" />
            </div>         
        </QHeader>

        <QDrawer v-model="drawer" side="left" behavior="desktop" bordered>
            
        </QDrawer>

        <QPageContainer>
            <NuxtPage />
        </QPageContainer>

    </QLayout>
</template>

<style scoped lang="scss">
.test {
    user-select: none;
    display: flex;
    justify-content: flex-end;
}

.titlebar {
    height: 30px;
    user-select: none;
    display: flex;
    justify-content: flex-end;
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
}
</style>