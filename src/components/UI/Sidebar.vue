<script setup lang="ts">
defineProps({
  title: String
})
</script>

<template>
  <div class="sidebar">
    <h3 class="sidebar-title">{{title}}</h3>
    <div class="sidebar-block">
      <slot />
    </div>
  </div>
</template>

<style lang="scss" scoped>
  .sidebar {
    min-height: calc(100vh - 8rem);
    height:100%;
    min-width: 10rem;
    width: fit-content;
    background-color: $background-accent;
    padding: 1rem;
    border-radius: 1rem;
    font-size: 2rem;
  }
</style>