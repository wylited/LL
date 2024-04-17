
<template>
    <div class = "menu">
        <p>NEW</p>
        <p>Book Title is: {{ bookTitle }}</p>
        <p>Message is: {{ email }}</p>
        <p>Message is: {{ email }}</p>
        <input class = "inp" type="bookTitle" v-model="bookTitle" />
        <input class = "inp" type="email" v-model="email" />
    
        <button @click="submit">Submit</button>
    </div>
  </template>
  
<style>

  .menu {
    display: block;
  }

  .inp {
    display: block;
  }

</style>

  <script>
    
import { ref } from 'vue'

const message = ref('')

  export default {
    data() {
      return {
        email: '',
        bookTitle: ''
      }
    },
    methods: {
      submit() {
        console.log(this.email)
      }
    }
  }
  </script>
  