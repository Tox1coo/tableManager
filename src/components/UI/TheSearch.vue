<script setup lang="ts">
import search from "@/assets/img/search.png";
defineProps({
  modelValue: {
    type: [String,Number]
  },
  placeholder: {
    type: String,
    default: "Type anything"
  }
})
const emit = defineEmits(['update:modelValue']);

const updateValue = (event:Event):void =>{
  emit('update:modelValue', event.target.value);
}
</script>

<template>
  <div class="search">
    <input type="search" class="search-input" @input="updateValue" :placeholder="placeholder">
    <img :src="search" alt="search">
  </div>
</template>

<style lang="scss" scoped>
  .search {
    flex: 0 1 40%;
    height: 5rem;
    position: relative;
    &-input {
      width: 100%;
      height: 100%;
      border-radius:1rem;
      padding-left: 4rem;
      font-size: 1.6rem;
    }
    img {
      position: absolute;
      top: 50%;
      left: 5px;
      transform: translate(10px, -50%);
    }
  }
</style>