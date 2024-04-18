<template>
    <div>
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
            <h4>Total number of resources: {{ post['resources'].length }}</h4>

            <h2 class = "sub">Add a Contribution</h2>

            <div class = "left">
                <div class = "inpSchema">
                    <p>Add the title of your resource</p>
                    <input placeholder = "Title" class = "inp" type="title" v-model="title" />
                    <p>Add a description to your resource</p>
                    <input placeholder = "Description" class = "inp" type="desc" v-model="desc" />
                    <p>Upload your file</p>
                    <input
                        type="file"
                        @change="Images_onFileChanged($event)"
                        accept="image/*"
                        capture
                    />
                    <p>What is your name?</p>
                    <input placeholder = "Author" class = "inp" type="author" v-model="author" />
                    <button @click="submit">Submit</button>
                </div>
            </div>
            <div class = "preview">
                <h2>Preview</h2>
                <h3>: {{ title }}</h3>
                <p>Description: {{ desc }}</p>
                <p>Submitted by {{ author }}</p>
                <img :src = selectedFile>
            </div>

            <h2 class = "sub">View all Contributions</h2>

            <li v-for="cont in post['resources']">
            
                <h3>{{ cont.title }}</h3>
            </li>
        </div>

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
    import axios, {isCancel, AxiosError} from 'axios';
    import { ref } from 'vue' 
  export default {
    data() {
      return {
        loading: false,
        post: null,
        error: null,
        desc: '',
        title: '',
        author:'',
        selectedFile : null
      }
    },
    created() {
      // watch the params of the route to fetch the data again
      this.$watch(
        () => this.$route.params.isbn,
        this.fetchData,
        // fetch the data when the view is created and the data is
        // already being observed
        { immediate: true }
      )
    },
    methods: {
    Images_onFileChanged (event) {this.selectedFile = event.target.files[0];console.log("wow");},
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
    submit() {
        function getRandomInt(min, max) {
            const minCeiled = Math.ceil(min);
            const maxFloored = Math.floor(max);
            return Math.floor(Math.random() * (maxFloored - minCeiled) + minCeiled); // The maximum is exclusive and the minimum is inclusive
        }

        const x = getRandomInt(0,100000000)
        console.log(x,this.$route.params.isbn)

        const y = this.$route.params.isbn
        const mypostparameters= new FormData()
        mypostparameters.append('image', this.selectedFile, this.selectedFile.name);

        axios.post('http://188.166.250.75:3000/api/resources/' + this.$route.params.isbn, { id: x.toString(), book_isbn: parseInt(this.$route.params.isbn), title: this.title, author: this.author, description: this.desc, file_name: this.selectedFile.name, page_number: 10, collab_score: 0})
        .then(response => console.log(response))
        
        axios.post('http://188.166.250.75:3000/api/resources/' + this.$route.params.isbn + '/' + x.toString(), mypostparameters)
        
        console.log('posted fr (i hope)')
        },
    },
  }
</script>