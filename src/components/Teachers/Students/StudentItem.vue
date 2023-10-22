<script setup lang="ts">

</script>

<template>
  <div class="student">
    <div class="student-name">
      Алиса Иванченко
    </div>
    <div class="student-days">
      Пн,Вт,Ср
    </div>
    <div class="student-phone">
      +8-913-372-76-06
    </div>
    <div class="student-email">
      www.daniil_.ru@mail.ru
    </div>
    <div class="student-buttons">
      <button>Change</button>
      <button>Delete</button>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.student {
  padding: 1rem;
  border-radius: 1rem;
  border: 1px solid #B2B2B2;
  display: flex;
  justify-content: space-between;
  align-items: center;
  &:not(:last-child){
    margin-bottom: 1rem;
  }
  &-buttons {
    button:not(:last-child) {
      margin-right: 1rem;
    }
  }
}
</style>