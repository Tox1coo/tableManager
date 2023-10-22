<script setup lang="ts">
import {ref, watch} from 'vue'
import TheSearch from "@/components/UI/TheSearch.vue";
import DropDown from "@/components/UI/DropDown.vue";

const search = ref<string>("");
watch(() => search, (search, newSearch) => {
  console.log(newSearch.value);
},{deep:true})
</script>

<template>
 <header class="header">
   <div class="burger">
     <div class="burger-wrapper">
       <span></span>
     </div>
   </div>
   <DropDown :drop-list="['Рабочие таблицы', 'Новенькие', 'расходы']" />
   <TheSearch v-model="search"/>
 </header>
</template>

<style lang="scss" scoped>
%span-burger {
    content: '';
    position: absolute;
    display: block;
    width: inherit;
    height: inherit;
    background-color: inherit;
}
.header {
  display: flex;
  align-items: center;
  padding: 0 2rem;
  justify-content: space-between;
}
.burger {
  width: 4rem;
  height:4rem;
  cursor: pointer;
  &-wrapper {
    display: flex;
    justify-content: center;
    align-items: center;
    height: 100%;
    span {
      display: block;
      position: relative;
      background-color: $background-accent;
      height: 0.5rem;
      width: 100%;
      &::before {
        @extend %span-burger;
        top: -1rem;
      }
      &::after {
        @extend %span-burger;
        bottom: -1rem;
      }
    }
  }
}
</style>
