<script setup lang="ts">
definePageMeta({
    layout: 'custom'
})

import { ref } from 'vue'
import { QInput } from 'quasar'

const email = ref('')

</script>

<template>

    <div class="row fit justify-center q-pt-lg">
        <div class="text-white col-5 column items-center justify-center q-pb-xl">
            <div class="text-h4">ChatAssistant</div>
            <div class="text-subtitle1">Log in or create an account</div>
        </div>

        <div class="bg-white col-4 rounded-borders q-ma-xl q-pa-xl" style="height: 471px;">
            <div class="">
                <div class="text-h5 text-weight-medium">Log in</div>
                <div class="text-subtitle2">Not a user? <NuxtLink class="text-grey">Create an account</NuxtLink></div>
            </div>

            <div>
                <QInput v-model="email" label="E-mail" class=""/>
                <!--One button for sending code to email to login, and one cancle button-->
                <div class="row justify-between">
                    <QBtn label="Cancel" class="q-mt-md text-black" flat :loading="false"/>
                    <QBtn label="Send code" color="" text-color="white"  rounded dense/>
                </div>
            </div>
        </div>
    </div>

</template>

<style scoped lang="scss"></style>