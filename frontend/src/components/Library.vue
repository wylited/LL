
<script setup lang="ts">

import { defineAsyncComponent } from 'vue'



const asyncModal = defineAsyncComponent(() => import('./Test.vue'))



window.onload = function() {
  
  const booksEl = document.getElementById("books")
  const apiUrl = 'http://188.166.250.75:3000/api/books';
  

  // Make a GET request
  fetch(apiUrl)
    .then(response => {
      if (!response.ok) {
        throw new Error('Network response was not ok');
      }
      return response.json();
    })
    .then(data => {
      data.forEach(book => {
        console.log(book)
        const loading = document.getElementById("loading")
        loading.innerText = ""
        const bookEl = document.createElement('p')
        bookEl.innerHTML = `<a class="routerlink" href="${"/books/"+book.isbn}">${book.isbn}</a>`
        booksEl?.append(bookEl)
      });
    })
    .catch(error => {
      console.error('Error:', error);
    });
}
</script>


<template>
  <div class = "main">
    <div class = "topBar">
      <h1 class="title">Untitled Project</h1>
      <div class = "searchBar">
        <asyncModal></asyncModal>
        <p>search bar goes here :skull: (feels weird here but idk what else to put in this space)</p>
      </div>
    </div>
    <div id = "books" class = "library">
      <p id = "loading">Loading...</p>
    </div>
  </div>
</template>



<style scoped>

#loading {
  font-size: larger;
  color:blueviolet;

}

.searchBar {
  display:inline-block;
  text-align:center;
  margin:0 10rem;
}

.library {
  display: flex;
  margin: 2% 4.5%;
  background-color: red;
  min-height: 80vh;
  min-width: 100%;
  padding:3vw;

}

.topBar {
  padding:1% 2%;
}

.main {
  width:100%;
}

.title {
  text-decoration: none;
  color: hsla(160, 100%, 37%, 1);
  transition: 0.4s;
  display: inline-block;
}

</style>
