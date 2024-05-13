// Import the functions you need from the SDKs you need
import { initializeApp } from "firebase/app";
import { getAnalytics } from "firebase/analytics";
// TODO: Add SDKs for Firebase products that you want to use
// https://firebase.google.com/docs/web/setup#available-libraries

// Your web app's Firebase configuration
// For Firebase JS SDK v7.20.0 and later, measurementId is optional
const firebaseConfig = {
  apiKey: "AIzaSyCaHTas9iSYqIBpg7rdsV3_nOOnTOjTB6I",
  authDomain: "chatassistant-real.firebaseapp.com",
  projectId: "chatassistant-real",
  storageBucket: "chatassistant-real.appspot.com",
  messagingSenderId: "252365064733",
  appId: "1:252365064733:web:8373d7b55c80edd0eb14f4",
  measurementId: "G-GRLLKSL6H9"
};

// Initialize Firebase
const app = initializeApp(firebaseConfig);
const analytics = getAnalytics(app);