<script setup lang="ts">
import { open } from '@tauri-apps/api/shell';
import {ref} from 'vue';
  defineProps({
    dropList: {
      type: Array<String>,


    }
  })
  // const vContains = {
  //   mounted: (el:Node) => {
  //
  //     if(!el.contains(dropdown)) {
  //       console.log(el)
  //     }
  //   }
  // }
  const openUrl = () => {
  }
  const close = (e:Event) => {

  }
</script>

<template>
  <div @click="close"  class="dropdown">
    <ul class="dropdown-items">
      <li v-for="(dropItem,index) in dropList" class="dropdown-item">
        <a @click="openUrl" >{{ dropItem }}</a>
      </li>
    </ul>
  </div>
</template>

<style scoped>

</style>