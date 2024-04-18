<template>
    <div class="post">
      <div v-if="loading" class="loading">Lasdasdoading...</div>
  
      <div v-if="error" class="error">{{ error }} <p>TEST11231</p></div>
  
      <div v-if="post" class="content"> 
        <li v-for="book in post">
          <p class = "vardec">{{ x = "/books/" + book['isbn'] }}</p>
          <RouterLink class = "linky" :to = x >
            <div class = "bookDisplay">
              <p class = "bkTitle">{{ book.title }}</p>
              <img class = "bookImg" :src=book.image_url alt="Paris"> 
            </div>
          </RouterLink>
        </li>   
        
        
        </div>
        
        
    </div>
    <RouterLink class = "new linky" to = "/addbook">
          <p class = "addtxt">Add a Book</p>
          <img class ="plus" src = "https://cdn-icons-png.flaticon.com/512/7598/7598663.png" alt ="plus" >
        </RouterLink>
  </template>

  <style scoped>

  li {
    list-style-type: none;
    display: inline-block;
    padding:0 1vw;
  }

  .addtxt {
    text-align: center;
    color:white;
  }

  .plus {
    margin-left: auto;
    margin-right: auto;
    width: 80px;
    margin:2vw;
    margin-top:40%;
  }

  .new {
    margin-left:1vw;
    display: inline-block;
    justify-content: center;
    text-align: center;
    border-radius: 10px;
    border-style: solid;
    border-width: 2px;
  }

  .linky {
    height:310px;
    width:200px;
    display: inline-block;
    transition-duration: .4s;
    border-radius: 10px;
    border-style: solid;
    border-width: 2px;
  }

  .linky:hover {
    background-color: rgb(82, 82, 82);
    

  }

  .vardec {
    display:none;
  }

  .bookImg {
    width:200px;
    height:260px;
    border-radius: 10px;
  }

  .bkTitle {
    text-decoration: none;
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
    text-decoration: none; 
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
                this.post = data
                
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

