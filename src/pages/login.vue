<script setup lang="ts">
import { getAuth, signInWithPopup, GoogleAuthProvider, GithubAuthProvider, sendSignInLinkToEmail, isSignInWithEmailLink, signInWithEmailLink } from "firebase/auth";
import { app } from '../firebase/firebase';
import GoogleSVG from "~/assets/img/Google.svg"
import GithubSVG from "~/assets/img/Github.svg"

definePageMeta({
  layout: 'custom'
})
const router = useRouter();
const auth = getAuth(app);

const googleProvider = new GoogleAuthProvider();
const githubProvider = new GithubAuthProvider();

// Sign in with Google
async function signInWithGoogle() {
  const auth = getAuth();

  try {
    const result = await signInWithPopup(auth, googleProvider);
    // Signed in user info
    const user = result.user;

    router.push('/')
    console.log(user)
  } catch (error: any) {
    const errorCode = error.code;
    const errorMessage = error.message;
    const email = error.email;

    const credential = GoogleAuthProvider.credentialFromError(error);

    alert(errorMessage)
    console.log(errorCode, errorMessage, email, credential)
  }
}

// Sign in with Github
async function signInWithGithub() {
  const auth = getAuth();
  try {
    const result = await signInWithPopup(auth, githubProvider);
    // Signed in user info.
    const user = result.user;

    router.push('/')
    console.log(user)
  } catch (error: any) {
    const errorCode = error.code;
    const errorMessage = error.message;
    const email = error.email;

    const credential = GithubAuthProvider.credentialFromError(error);

    alert(errorMessage)
    console.log(errorCode, errorMessage, email, credential)
  }
}

const email = ref('');

function clearInput() {
  email.value = ''
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
        <QInput v-model="email" label="E-mail" class="q-mt-sm" dense />
        <div class="row justify-end q-gutter-x-sm q-py-sm">
          <QBtn @click="clearInput" label="Cancel" size="sm" flat rounded />
          <QBtn label="Send code" size="sm" color="accent" text-color="white" rounded />
        </div>
      </div>

      <div class="row items-center justify-between q-mt-md">
        <div class="col-5" style="background-color: var(--q-accent); height: 1px; border-radius: 50%;"></div>
        <div>OR</div>
        <div class="col-5" style="background-color: var(--q-accent); height: 1px; border-radius: 50%;"></div>
      </div>

      <div>
        <div class="column justify-between q-mt-md">
          <QBtn @click="signInWithGoogle" class="q-pa-sm" :icon="`img:${GoogleSVG}`" label="Continue with Google"
            size="sm" color="accent" text-color="black" rounded no-caps outline />
          <QBtn @click="signInWithGithub" class="q-mt-sm q-pa-sm" :icon="`img:${GithubSVG}`" style="border-color: blue;"
            label="Continue with Github" size="sm" color="accent" text-color="black" rounded no-caps outline />
        </div>
      </div>
    </div>
  </div>

</template>

<style scoped lang="scss"></style>