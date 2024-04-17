
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
    </div>
    <div id = "books" class = "library">
      <asyncModal></asyncModal>
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
  background-color: grey;
  border-radius: 2vw;
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


</style>
