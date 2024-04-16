<template>
  <h1>Hello App!</h1>
  <p>
    <strong>Current route path:</strong> {{ $route.fullPath }}
  </p>
  <nav>
    <RouterLink to="/">Go to Home</RouterLink>
    <RouterLink to="/books/3">Go to About</RouterLink>
  </nav>
  <main>
    <router-view :key="$route.path"></router-view>
  </main>
</template>