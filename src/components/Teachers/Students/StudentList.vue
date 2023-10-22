<script setup lang="ts">

import {ref, watch, Ref} from "vue";
import TheSearch from "@/components/UI/TheSearch.vue";
import StudentItem from "@/components/Teachers/Students/StudentItem.vue";
const searchStudent = ref<string>("");
watch(() => searchStudent, (searchStudent:Ref<string>, student:Ref<string>) => {
  console.log(student.value);
},{deep:true})
</script>

<template>
<div class="students">
  <div class="students-header">
    <TheSearch v-model="searchStudent"/>
    <button>Добавить ученика</button>
  </div>
  <div class="students-wrapper">
    <StudentItem v-for="item in 16" />
  </div>
</div>
</template>

<style scoped lang="scss">
.students {
  height: calc(100vh - 8rem);
  overflow-y: scroll;
  flex: 0 1 100%;
  padding: 1rem;
  &-wrapper {
    margin-top: 2rem;
  }
  &-header {
    display: flex;
    justify-content: space-between;
  }
}
</style>