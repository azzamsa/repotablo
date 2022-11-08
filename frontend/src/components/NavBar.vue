<script setup lang="ts">
// Menu
const showMenu = ref(false)
</script>

<template>
  <header>
    <div class="mb-2 shadow-lg navbar rounded-box bg-primary">
      <div class="px-2 mx-2 navbar-start">
        <span class="text-lg font-bold">
          <RouterLink class="nav-btn" to="/"> Vue </RouterLink>
        </span>
      </div>

      <div class="px-2 mx-2 navbar-center">
        <div class="hidden items-stretch md:flex">
          <RouterLink class="nav-btn" to="/">
            Home
          </RouterLink>
          <RouterLink class="nav-btn" to="/about">
            About
          </RouterLink>
        </div>
      </div>
    </div>

    <ul
      v-if="showMenu"
      class="py-3 font-bold md:hidden menu rounded-box bg-primary"
    >
      <li>
        <RouterLink class="nav-btn" to="/" @click="showMenu = !showMenu">
          Home
        </RouterLink>
      </li>
      <div class="mt-0 mb-0 divider" />
      <li>
        <RouterLink class="nav-btn" to="/about" @click="showMenu = !showMenu">
          About
        </RouterLink>
      </li>
    </ul>
  </header>
</template>

<style scoped>
.nav-btn {
  @apply btn btn-ghost btn-sm rounded-btn;
}
</style>
