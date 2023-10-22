<script setup lang="ts">
import Header from "@/components/UI/Header.vue";
import Sidebar from "@/components/UI/Sidebar.vue";
import TeacherItem from "@/components/Teachers/TeacherItem.vue";
import StudentList from "@/components/Teachers/Students/StudentList.vue";

</script>

<template>
  <Header/>
  <main>
    <Sidebar :title="'Учителя'">
      <ul class="teacher-list">
        <TeacherItem />
      </ul>
    </Sidebar>
    <StudentList/>
  </main>
</template>




<style lang="scss" scoped>
.sidebar {
  flex: 1 1 15%
}
</style>
