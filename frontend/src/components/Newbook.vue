
<template>
    <div class = "menu">
        <div class = "left">
            <h2 class = "tit">Create a new book repository!</h2>
            <p class = "sub">If your textbook isn't already on the website, don't worry - you can put it on.</p>
            <div class = "inpSchema">
                <p>Add the title of your textbook</p>
                <input placeholder = "Book Title" class = "inp" type="bookTitle" v-model="bookTitle" />
                <p>Add the ISBN of your textbook</p>
                <input placeholder = "ISBN Number" class = "inp" type="bookISBN" v-model="bookISBN" />
                <p>Add a working standalone URL to an image of the textbook's cover</p>
                <input placeholder = "Front Cover Image URL" class = "inp" type="bookimgURL" v-model="bookimgURL" />
                <p>Add the primary author of your textbook</p>
                <input placeholder = "Author" class = "inp" type="bookAuthor" v-model="bookAuthor" />
                <br>
                <button @click="submit">Submit</button>
              </div>
        </div>
        <div class = "preview">
            <h2 class = "prevtxt">Preview</h2>
            <p>Title: {{ bookTitle }}</p>
            <p>ISBN: {{ bookISBN }}</p>
            <img class = "img2":src = bookimgURL alt = "Image not loaded">
            <p>Written by: {{ bookAuthor }}</p>
        </div>
    
        
    </div>
  </template>
  
<style>

  .img2 {
    max-width: 30vw;
  }

  .prevtxt {
    font-style: italic;
    color:rgb(111, 111, 111);
  }

  .preview {
    background-color: rgb(41, 41, 41);
    border-radius: 10px;
    margin-left: 4vw;
    padding:2vw;
    min-width:30vw;
    display:inline-block;

  }

  .inpSchema {
    background-color: rgb(41, 41, 41);
    border-radius: 10px;
    padding:1vw;
    display:inline-block;
  }
  
  .left {
    display:inline-block;
  }



  .sub {
    font-style: italic;
  }

  .menu {
    display: block;
    margin-left:4vw;
  }

  .inp {
    display: block;
    margin:1vw;
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
            axios.post('http://188.166.250.75:3000/api/books', { isbn: parseInt(this.bookISBN), title: this.bookTitle, authors: [this.bookAuthor], image_url: this.bookimgURL })
            .then(response => {
            alert("Book successfully added!")
            router.push('/')
            }
          )
            .except(error => alert("There was an error in creating your repository. Please try again later."))
            console.log('posted fr (i hope)')

        }
        }
    }
  </script>
  