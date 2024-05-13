<script setup lang="ts">
import GoogleSVG from "~/assets/img/Google.svg"
import GithubSVG from "~/assets/img/Github.svg"

definePageMeta({
    layout: 'custom'
})

const email = ref('')

// const { signIn } = useGoogleLogin(process.env.GOOGLE_CLIENT_ID)

const loginWithGoogle = async () => {
  try {
    // const user = await signIn()
    // handle successful login
  } catch (error) {
    // handle error
  }
}

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
                <QInput v-model="email" label="E-mail" class="q-mt-sm" dense/>
                <!--One button for sending code to email to login, and one cancle button-->
                <div class="row justify-end q-gutter-x-sm q-py-sm">
                    <QBtn label="Cancel" size="sm" flat rounded/>
                    <QBtn label="Send code" size="sm" color="accent" text-color="white" rounded />
                </div>
            </div>

            <div class="row items-center justify-between q-mt-md">
                <div class="col-5" style="background-color: var(--q-accent); height: 1px; border-radius: 50%;"></div>
                <div>OR</div>
                <div class="col-5" style="background-color: var(--q-accent); height: 1px; border-radius: 50%;"></div>
            </div>

            <div>
                <!--Continue with google and continue with github buttons-->
                <div class="column justify-between q-mt-md">
                    <QBtn @click="loginWithGoogle" class="q-pa-sm" :icon="`img:${GoogleSVG}`" label="Continue with Google" size="sm" color="accent" text-color="black" rounded no-caps outline/>
                    <QBtn class="q-mt-sm q-pa-sm" :icon="`img:${GithubSVG}`" style="border-color: blue;" label="Continue with Github" size="sm" color="accent" text-color="black" rounded no-caps outline/>
                </div>
            </div>
        </div>
    </div>

</template>

<style scoped lang="scss"></style>