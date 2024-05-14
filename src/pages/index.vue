<script setup lang="ts">
import { getAuth, signOut } from "firebase/auth";
import { app } from '../firebase/firebase';
import { routerKey } from "vue-router";


const auth = getAuth(app);
const router = useRouter();

function logout() {
  signOut(auth).then(() => {
    // Sign-out successful.
    router.push('/login')
  }).catch((error) => {
    // An error happened.
  });
  console.log('logout')
  console.log(auth.currentUser)
}
</script>

<template>
  <button @click="logout">Logout</button>
</template>

<style scoped lang="scss"></style>