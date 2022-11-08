<script setup lang="ts">
const linksStore = useLinksStore()

const links = ref('')

watch(links, () => {
  linksStore.setLinks(links.value)
})
</script>

<template>
  <div class="mt-20 hero">
    <div class="text-center hero-content">
      <textarea v-model="links" class="textarea textarea-bordered" placeholder="[link](https://foo.com)" />
    </div>
  </div>
</template>
