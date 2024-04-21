<template>
<link rel="preconnect" href="https://fonts.googleapis.com">
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
<link href="https://fonts.googleapis.com/css2?family=Nunito+Sans:ital,opsz,wght@0,6..12,200..1000;1,6..12,200..1000&display=swap" rel="stylesheet">
  <nav>
    <h1 class="title inl">Learner's Library</h1>
    <RouterLink class = "inl link" to="/">Your Library</RouterLink>
    <div class = "inl search">
      <input placeholder = "Search for a book or resource!" class = "inp" type="searchq" v-model="searchq" />
      <button class = "tst" @click="submit">Submit</button>
    </div>
  </nav>
  <main>
    <router-view v-slot="{ Component }":key="$route.path">

    </router-view>
  </main>
</template>


<script>
    import axios, {isCancel, AxiosError} from 'axios';
    import { ref } from 'vue'

    const message = ref('')

    export default {
        data() {
        return {
            bookISBN: '',
        }
        },
        methods: {
        submit() {
          console.log(this.bookISBN)
          
          }
        }
    }
  </script>


<style scoped>

nav {
  width:100%;
}

.search {
  width:45%;
}

.tst {
  display:inline-block;
}

.inp {

  width:50%;
  display:inline-block;
}

.title {
  text-decoration: none;
  font-size:2vw;
  color: hsla(160, 100%, 37%, 1);
  transition: 0.4s;
  display: inline-block;

}

.link {
  font-family: "Spectral", serif;
  font-size: medium;
  color:rgb(27, 169, 110);
  padding:.5vw 2vw;
  text-decoration: none;
}


nav {
  display: inline-block;

}

.inl {
  transition-duration: 0.2s;
  display: inline-block;
  margin:0vw 3vw;
  margin-top:2vw;
  font-weight: 400;
  font-style: normal;
}

</style>