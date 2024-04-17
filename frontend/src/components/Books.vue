<template>
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=Open+Sans:ital,wght@0,300..800;1,300..800&display=swap" rel="stylesheet"> 

    <div class="post">
      <div v-if="loading" class="loading">Loading...</div>
  
      <div v-if="error" class="error">{{ error }} <p>TEST11231</p></div>
  
      <div v-if="post" class="content">
        
        <img class = "bookImg" :src=post.image_url alt="Paris"> 
        
        <div class = "mainBod">
            <h2 class = "title">{{ post['title'] }}</h2>
            <p class = "author">{{ post['authors'][0] }}, {{ post['isbn'] }}</p>

            <h2 class = "sub">Add a Contribution</h2>
        </div>

      </div>
    </div>
  </template>


<style scoped>

.sub {
    margin-top:5vw;
}

.isbn {
    float:right;
    display:block;
}

.bookImg {
    display:block;
    width:300px;
    border-radius:10px;
    display: inline-block;
    float:right;
    margin-top:3vw;
}

.title {
    font-size:2.3vw;
    font-family: "Open Sans", sans-serif;
    font-optical-sizing: auto;
    font-weight: <weight>;
    font-style: normal;
    font-variation-settings:
     "wdth" 10;
    
}

.mainBod {
    display: inline-block;
    margin:2vw 4vw;

}

.author {
    font-style: italic;
    margin-top: -1vw;
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
        () => this.$route.params.imdb,
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
            const apiUrl = 'http://188.166.250.75:3000/api/fullbooks';
            console.log(apiUrl)
            fetch(apiUrl)
            .then(response => {
                if (!response.ok) {
                    throw new Error('Network response was not ok');
                }
                return response.json();
            })
            .then(data => {
                console.log("BRUH!!!")
                console.log(data)
                data.forEach((book) => {
                    if (book.isbn==this.$route.params.isbn) {
                        this.post = book
                        console.log("SUCCESS")
                        console.log(book)
                    }
                    else {
                        console.log("FAIL")
                        console.log(book.isbn)
                        console.log( this.$route.params.isbn)
                    }
                });
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