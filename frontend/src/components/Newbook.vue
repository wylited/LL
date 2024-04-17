
<template>
    <div>
        <p>NEW</p>
        <p>Message is: {{ message }}</p>
        <input v-model="message" placeholder="edit me" />
      <input type="email" v-model="email" />
  
      <button @click="submit">Submit</button>
    </div>
  </template>
  
  <script>
    
import { ref } from 'vue'

const message = ref('')

  export default {
    data() {
      return {
        email: ''
      }
    },
    methods: {
      submit() {
        console.log(this.email)
      }
    }
  }
  </script>