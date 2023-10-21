<script setup lang="ts">
defineProps({
  modelValue: {
    type: [String,Number]
  },
})
const emit = defineEmits(['update:modelValue']);

const updateValue = (event:Event):void =>{
  emit('update:modelValue', event.target.value);
}
</script>

<template>
  <div class="search">
    <input type="search" class="search-input" @input="updateValue">
  </div>
</template>

<style lang="scss" scoped>

</style>