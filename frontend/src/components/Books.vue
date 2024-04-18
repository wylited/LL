<template>
    <div>
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=Open+Sans:ital,wght@0,300..800;1,300..800&display=swap" rel="stylesheet"> 

    <div class="post">
        
      <div v-if="loading" class="loading"><h1>Loading...</h1></div>
  
      <div v-if="error" class="error">{{ error }} <p>TEST11231</p></div>
  
      <div v-if="post" class="content">
        
        <img class = "bookImg" :src=post.image_url alt="Paris"> 
        
        <div class = "mainBod">
            <h2 class = "title">{{ post['title'] }}</h2>
            <p class = "author">{{ post['authors'][0] }}, {{ post['isbn'] }}</p>
            

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
                <h3>{{ title }}</h3>
                <p>{{ desc }}</p>
                <p class = "bot">Submitted by {{ author }}</p>
                <img :src = selectedFile>
            </div>
        </div>
        <div class = "cont">
            <h2 class = "sub">View all Contributions</h2>
            <h4>Total number of resources: {{ post['resources'].length }}</h4>
            <input value = "0" placeholder = "to" class = "to" type="to" v-model="to" />
            <input value = "1000" placeholder = "from" class = "from" type="from" v-model="from" />
            <br>
            <li class = "here1" v-for="cont in post['resources']">
                <div class = "here" v-if="cont.page_number>=to && cont.page_number<=from">
                    <h3 class = "ititle">{{ cont.title }}</h3>
                    <p>{{ cont.description }}</p>
                    <p class = "cont">Contributed by {{ cont.author }}</p>
                    <p>current score is {{ cont.collab_score }}</p>
                    <button @click="AddScore(cont.book_isbn,cont.id,cont.collab_score)">Add 1</button>
                    <button @click="RemoveScore(cont.book_isbn,cont.id,cont.collab_score)">Remove 1</button>
                </div>
                <div v-else class = "gone">
                    <h2>bruh</h2>
                </div>
            </li>

        </div>

      </div>
    </div>
    </div>
  </template>


<style scoped>

.cont {
    margin-left:4vw;
}

.bot {
    font-style: italic;
    margin-top:40%;
}

.to {
    display:inline-block;
}

.from { 
    display:inline-block;
}

.gone {
    display:none;
}

.cont {
    font-style: italic;
}

.ititle {
    color:rgb(28, 31, 0);
}

.here {
    display: inline-block;
    background-color: grey;
    margin:1vw;
    min-height:5vw;
    min-width:10vw;
    padding:1vw;
    
}

.here1 {
    list-style: none;
    display:inline-block;
}


h1 {
    color:red;
    font-size: 40vw;
}



.preview {
    background-color: rgb(41, 41, 41);
    min-width:30vw;
    min-height:30vw;
    margin-left:7vw;
    border-radius: 10px;
    padding:1vw;
}

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
    font-size:3vw;
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
    width:65%;

}

.author {
    font-style: italic;
    margin-top: -2vw;
    font-size: 1.3vw;
}


</style>
  
<script lang="ts"> 
    import axios, {isCancel, AxiosError} from 'axios';
    import { getCurrentInstance } from 'vue';
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
        selectedFile : null,
        to: '',
        from:'',
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
    AddScore(ib,id,score) {
        this.post.resources.forEach((resc) => {
            if (resc.id == id) {
                resc.collab_score += 1;
            }
        });

        axios.post('http://188.166.250.75:3000/api/resources/' + this.$route.params.isbn + '/' + id + '/' + score + 1)
            .then(response => console.log(response))
    },
    RemoveScore(ib,id,score) {
        this.post.resources.forEach((resc) => {
            if (resc.id == id) {
                resc.collab_score -= 1;
            }
        });

        axios.post('http://188.166.250.75:3000/api/resources/' + this.$route.params.isbn + '/' + id + '/' + score - 1)
            .then(response => console.log(response))
    },
    async fetchData(id) {
        this.to = 0
        this.from = 1000
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

        
        console.log("filename:")
        if (this.selectedFile==null) {
            axios.post('http://188.166.250.75:3000/api/resources/' + this.$route.params.isbn, { id: x.toString(), book_isbn: parseInt(this.$route.params.isbn), title: this.title, author: this.author, description: this.desc, file_name: "nofileLMAO", page_number: 10, collab_score: 0})
            .then(response => console.log(response))
        }
        else {
            const mypostparameters= new FormData()
            mypostparameters.append('image', this.selectedFile, this.selectedFile.name);

            axios.post('http://188.166.250.75:3000/api/resources/' + this.$route.params.isbn, { id: x.toString(), book_isbn: parseInt(this.$route.params.isbn), title: this.title, author: this.author, description: this.desc, file_name: this.selectedFile.name, page_number: 10, collab_score: 0})
            .then(response => console.log(response))
            
            axios.post('http://188.166.250.75:3000/api/resources/' + this.$route.params.isbn + '/' + x.toString(), mypostparameters)
        }
        
        this.post.resources.push({
        "id": x,
        "book_isbn": this.post.isbn,
        "title": this.title,
        "author": this.author,
        "description": this.desc,
        "file_name": "test.txt",
        "page_number": "12",
        "collab_score": 0
      })



        console.log('posted fr (i hope)')
        },
    },
  }
</script>