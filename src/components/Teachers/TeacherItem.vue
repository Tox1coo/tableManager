<script setup lang="ts">
defineProps({
  name: {
    type: String,
    default: ''
  }
})
</script>

<template>
  <li class="teacher-item">
    <div class="teacher-name">
      Алеся
    </div>
  </li>
</template>

<style lang="scss" scoped>
.teacher-item {
  padding: 0.5rem;
  color: $color;
  cursor: pointer;
}
</style>