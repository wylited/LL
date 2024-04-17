
<script setup>

import { defineAsyncComponent } from 'vue'



const asyncModal = defineAsyncComponent(() => import('./LibraryBooks.vue'))




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

.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.5s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}

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
  margin: 0 4.5%;
  margin-top: 2%;
  background-color: rgb(62, 62, 62);
  border-radius: 2vw;
  min-height: 60vh;
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
