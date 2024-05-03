<script setup lang="ts">
import { appWindow } from '@tauri-apps/api/window'

const drawer = ref(false)

const minimize = () => appWindow.minimize()
const maximize = () => appWindow.toggleMaximize()
const close = () => appWindow.close()

</script>

<template>
    <QLayout view="lHh Lpr lFf">
        <QHeader class="text-white "  style="background: none;">
            <div class="titlebar" data-tauri-drag-region>
                <QSpace data-tauri-drag-region/>
                <QBtn flat dense round icon="remove" color="secondary" @click="minimize" />
                <QBtn flat dense round icon="crop_square" color="secondary" @click="maximize" />
                <QBtn flat dense round icon="close" color="secondary" @click="close" />
                <NuxtLink to="/login" >aeae</NuxtLink>
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
.titlebar {
    height: 2.2rem;
    user-select: none;
    display: flex;
    justify-content: flex-end;
}
</style>