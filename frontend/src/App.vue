<template>
  <link rel="preconnect" href="https://fonts.googleapis.com">
  <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="">
  <link href="https://fonts.googleapis.com/css2?family=Spectral:wght@400;500;600;700;800&display=swap" rel="stylesheet">
  <nav>
    <h1 class="title inl">Untitled Project</h1>
    <RouterLink class = "inl link" to="/">Your Library</RouterLink>
    <div class = "inl search">
      <p> Search Bar Goes Here </p>
    </div>
  </nav>
  <main>
    <router-view v-slot="{ Component }":key="$route.path">

    </router-view>
  </main>
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
.title {
  text-decoration: none;
  font-size:2vw;
  color: hsla(160, 100%, 37%, 1);
  transition: 0.4s;
  display: inline-block;
  font-family: "Spectral", serif;
  font-weight: 600;
  font-style: normal;
}

.link {
  font-size: medium;
  color:rgb(79, 214, 158);
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
  font-family: "Spectral", serif;
  font-weight: 400;
  font-style: normal;
}

</style>