<template>
    <div class="post">
      <div v-if="loading" class="loading">Loading...</div>
  
      <div v-if="error" class="error">{{ error }} <p>TEST11231</p></div>
  
      <div v-if="post" class="content">        
        {{ post.title }}
        <p class = "vardec">{{ x = "/books/" + post.isbn }}</p>
        <p>TEST</p>
        <RouterLink :to = x >Go to Home</RouterLink>
      </div>
    </div>
  </template>
  
  <script>  
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
            const apiUrl = 'http://188.166.250.75:3000/api/books/' + this.$route.params.isbn;
            console.log(apiUrl)

            fetch(apiUrl)
            .then(response => {
                if (!response.ok) {
                    throw new Error('Network response was not ok');
                }
                return response.json();
            })
            .then(data => {
                this.post = data
                console.log(data)
            })
            .catch(error => {
                console.error('Error:', error);
            });
          this.post = await fetch("http://188.166.250.75:3000/api/books")
        } catch (err) {
          this.error = err.toString()
        } finally {
          this.loading = false
        }
      },
    },
  }
  </script>