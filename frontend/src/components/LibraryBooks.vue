<template>
    <div class="post">
      <div v-if="loading" class="loading">Lasdasdoading...</div>
  
      <div v-if="error" class="error">{{ error }} <p>TEST11231</p></div>
  
      <div v-if="post" class="content"> 
        {{ console.log("POST") }}
        {{ console.log(post) }}  
        {{ console.log(post['isbn']) }}     
        <p class = "vardec">{{ x = "/books/" + post['isbn'] }}</p>
        <RouterLink class = "linky" :to = x >
          <div class = "bookDisplay">
            <p class = "bkTitle">{{ post['title'] }}</p>
            <img class = "bookImg" :src=post.image_url alt="Paris"> 
          </div>
        </RouterLink>
        
        </div>
        
      
    </div>
    <RouterLink class = "new" to = "/addbook">
          <p>add book!</p>
        </RouterLink>
  </template>

  <style scoped>

  .new {
    display: inline-block;
    margin-left:3vw;
  }

  .linky {
    display: inline-block;
  }

  .linky:hover {
    border-radius: 10px;
    transition-duration: .4s;
    background-color: rgb(82, 82, 82);

  }

  .vardec {
    display:none;
  }

  .bookImg {
    width:200px;
    border-radius: 10px;
  }

  .bkTitle {
    text-align: center;
    color:white;
    font-family: "Spectral", serif;
    font-weight: 400;
    font-style: normal;
  }

  .bookDisplay {
    text-align: center;
  }

  RouterLink {
    background-color: red;
  }
  
  </style>


  <script lang="ts">  
  export default {
    data() {
      return {
        loading: false,
        post: null,
        error: null,
      }
    },
    created() {
      // watch the params of the route to fetch the data again
      this.$watch(
        () => this.$route.params.id,
        this.fetchData,
        // fetch the data when the view is created and the data is
        // already being observed
        { immediate: true }
      )
    },
    methods: {
      async fetchData(id) {
        this.error = this.post = null
        this.loading = true
  
        try {
          // replace `getPost` with your data fetching util / API wrapper
          const apiUrl = 'http://188.166.250.75:3000/api/books';
              fetch(apiUrl)
                .then(response => {
                if (!response.ok) {
                    throw new Error('Network response was not ok');
                }
                console.log("Begin Fetch")
                console.log(response)
                return response.json();
                })
                .then(data => {
                this.post = data[0]
                
                console.log(data)
                console.log("End Fetch")
                })
                .catch(error => {
                console.error('Error:', error);
                });
        } catch (err) {
          this.error = err.toString()
        } finally {
          this.loading = false
        }
      },
    },
  }
  </script>

