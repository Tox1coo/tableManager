<script setup lang="ts">
import {ref, watch, watchEffect} from 'vue'
import TheSearch from "@/components/UI/TheSearch.vue";


const count = ref(0)
const search = ref<string>("");
watch(() => search, (search, newSearch) => {
  console.log(newSearch.value);
},{deep:true})
</script>

<template>
 <header class="header">
   <TheSearch v-model="search"/>
   {{search}}
 </header>
</template>

<style scoped>
.read-the-docs {
  color: #888;
}
</style>
