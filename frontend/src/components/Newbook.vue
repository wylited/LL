
<template>
    <div class = "menu">
        <h2 class = "tit">Create a new book repository!</h2>
        <p class = "sub">If your textbook isn't already on the website, don't worry - you can put it on.</p>
        <input placeholder = "Book Title" class = "inp" type="bookTitle" v-model="bookTitle" />
        <input placeholder = "ISBN Number" class = "inp" type="bookISBN" v-model="bookISBN" />
        <input placeholder = "Front Cover Image URL" class = "inp" type="bookimgURL" v-model="bookimgURL" />
        <input placeholder = "Author" class = "inp" type="bookAuthor" v-model="bookAuthor" />
        <p>Book Title is: {{ bookTitle }}</p>
        <p>ISBN is: {{ bookISBN }}</p>
        <p>img url is: {{ bookimgURL }}</p>
        <img :src = bookimgURL alt = "Image not loaded">
        <p>author is: {{ bookAuthor }}</p>
        
    
        <button @click="submit">Submit</button>
    </div>
  </template>
  
<style>



  .sub {
    font-style: italic;
  }

  .menu {
    display: block;
    margin-left:4vw;
  }

  .inp {
    display: block;
  }

</style>

  <script>
    import axios, {isCancel, AxiosError} from 'axios';
    import { ref } from 'vue'

    const message = ref('')

    export default {
        data() {
        return {
            bookISBN: '',
            bookTitle: '',
            bookimgURL:'',
            bookAuthor:''
        }
        },
        methods: {
        submit() {
            console.log(this.bookISBN)
            axios.post('http://188.166.250.75:3000/api/books',{ params: { isbn: parseInt(this.bookISBN), title: this.bookTitle, authors: [this.bookAuthor], image_url: this.bookimgURL }})
            .then(response => console.log('actually posted'))
            console.log('posted fr (i hope)')
        }
        }
    }
  </script>
  