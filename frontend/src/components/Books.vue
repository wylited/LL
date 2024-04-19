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
                    <input placeholder = "Title" class = "inp title1" type="title" v-model="title" />
                    <p>Add a description to your resource</p>
                    <textarea rows="2" cols="25" placeholder = "Description" class = "inp descr" type="desc" v-model="desc"></textarea>
                    <p>Upload your file</p>
                    <input
                        type="file"
                        @change="Images_onFileChanged($event)"
                        accept="image/jpeg"
                        capture
                    />
                    <p>What is your name?</p>
                    <input placeholder = "Author" class = "inp" type="author" v-model="author" />
                    <p>What is the page reference?</p>
                    <input placeholder = "Page Reference" class = "inp" type="pref" v-model="pref" />
                    <button @click="submit">Submit</button>
                </div>
            </div>
            <div class = "preview">
                <h2 class = "prev">Preview</h2>
                <h3>⠀{{ title }}</h3>
                <p>⠀{{ desc }}</p>
                <p class = "bot">Submitted by {{ author }}</p>
            </div>
        </div>
        <div class = "cont">
            <h2 class = "sub">View all Contributions</h2>
            <h4>Total number of resources: {{ post['resources'].length }}</h4>
            <p class = "d">From page   </p>
            <input value = "0" placeholder = "to" class = "to pg" type="to" v-model="to" />
            <p class = "d">to   </p>
            <input value = "1000" placeholder = "from" class = "from pg" type="from" v-model="from" />
            <br>
            <li class = "here1" v-for="cont in post['resources']">
                <div class = "here" v-if="cont.page_number>=to && cont.page_number<=from">
                    <p class = "page">Page {{ cont.page_number }}</p>
                    <h3 class = "ititle">{{ cont.title }}</h3>
                    
                    <p>{{ cont.description }}</p>
                    <a :href = "'http://188.166.250.75:3000/api/resources/' + cont.book_isbn + '/' + cont.id">
                        <img class = "contImg" :src = "'http://188.166.250.75:3000/api/resources/' + cont.book_isbn + '/' + cont.id" alt = "No Image">
                    </a>
                    <p class = "cont2">Contributed by {{ cont.author }}</p>
                    
                    <p class = "inlsc">{{ cont.collab_score }}</p>
                    <button class = "adds" @click="AddScore(cont.book_isbn,cont.id,cont.collab_score)">Add 1</button>
                    
                    <button class = "minus" @click="RemoveScore(cont.book_isbn,cont.id,cont.collab_score)">Remove 1</button>
                    
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

.page {
    float:right;
    font-style: italic;
}

.contImg {
    width:300px;
}

.AddCont {
    max-width:10vw;
    background-color: rgb(41, 41, 41);
    border-radius: 10px;
}

.inlsc {
    text-align: center;
    color: rgb(54, 54, 54);
    margin-left:auto;
    margin-right:auto;
}

.pg {
    border:none;
    border-radius: 3px;
    transition-duration: .2s;

}

.pg:focus {
    border:none;
    
    
    background-color: rgb(185, 185, 185);
    outline:none;
}

.desc {
    
}

.prev {
    color: rgb(145, 145, 145);
    font-style: italic;
}

.adds {
    background-color: rgb(133, 164, 134);
    border:none;
    padding:.4vw;
    border-radius:8px;
    transition-duration: .3s;
}

.adds:hover {
    background-color: rgb(63, 177, 67);
}

.minus {
    background-color: rgb(182, 134, 134);
    border:none;
    padding:.4vw;
    float: right;
    border-radius:8px;
    transition-duration: .3s;
}

.minus:hover {
    background-color: rgb(185, 74, 74);
}

.left {
    background-color: rgb(41, 41, 41);
    border-radius: 10px;
    padding: 1vw;
}

.d {
    display: inline-block; 
    margin-left:1vw;
    margin-right: 1vw;
}

.cont {
    margin-left:4vw;
    min-height: 40vw;
}

.bot {
    font-style: italic;
    margin-top:40%;
}

.to {
    display:inline-block;
    width: 3vw;
}

.from { 
    display:inline-block;
    width: 3vw;
}

.gone {
    display:none;
}

.cont2 {
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
    border-radius: 10px;
    width:20vw;
    margin-left:7vw;
    
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
    width:400px;
    margin-right:-5vw;
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
        previewImage:null,
        dat:null,
        filename:null,
        prevFile:null,
        pref:'',
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
    Images_onFileChanged (event) {
        let data = new FormData();
        data.append('name', 'my-picture');
        data.append('file', event.target.files[0]); 
        this.filename = event.target.files[0].name;
        this.dat = data;
        this.prevFile = event.target.files[0]
        
    },
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
            // kill mer
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
        if (this.dat==null) {
            axios.post('http://188.166.250.75:3000/api/resources/' + this.$route.params.isbn, { id: x.toString(), book_isbn: parseInt(this.$route.params.isbn), title: this.title, author: this.author, description: this.desc, file_name: "nofileLMAO", page_number: parseInt(this.pref), collab_score: 0})
            .then(response => console.log(response))
            console.log("Null File")
        }
        else {
            console.log("Not Null File")
            axios.post('http://188.166.250.75:3000/api/resources/' + this.$route.params.isbn, { id: x.toString(), book_isbn: parseInt(this.$route.params.isbn), title: this.title, author: this.author, description: this.desc, file_name: this.filename, page_number: parseInt(this.pref), collab_score: 0})
            .then(response => console.log(response))
            
            let config = {
            header : {
                'Content-Type' : 'image/png'
            }
        }

        axios.post(
            'http://188.166.250.75:3000/api/resources/' + this.$route.params.isbn + '/' + x, 
            this.dat,
            config
        ).then(
        response => {
            console.log('image upload response > ', response)
        }
        )
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