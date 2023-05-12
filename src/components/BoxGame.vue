<script setup lang="ts">
defineProps<{value: string}>();
</script>

<template>
  <div
    class="box-container"
    :style="{backgroundColor: value == '2' ? '#545454': 'green'}"
  >
    2
  </div>
</template>

<style scoped>
.box-container {
  height: 98px;
  width: 98px;
  color: #FFFFFF;
  border-radius: 3px;
  text-align: center;
  line-height: 98px;
  font-size: 53px;
  font-weight: 700;
}
</style>