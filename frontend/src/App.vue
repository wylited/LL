<template>
  <nav>
    <h1 class="title inl">Untitled Project</h1>
    <RouterLink class = "inl" to="/">Go to Home</RouterLink>
    <div class = "inl search">
      <p> Search Bar Goes Here </p>
    </div>
  </nav>
  <main>
    <router-view :key="$route.path"></router-view>
  </main>
</template>

<style scoped>

.title {
  text-decoration: none;
  font-size:1.5vw;
  color: hsla(160, 100%, 37%, 1);
  transition: 0.4s;
  display: inline-block;
}

.search {
}
nav {
  display: inline-block;

}

.inl {
  display: inline-block;
  margin:0vw 3vw;
  margin-top:2vw;
}

</style>