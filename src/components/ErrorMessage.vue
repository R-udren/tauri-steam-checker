<script setup lang="ts">
defineProps<{
  message: string | null;
}>();
</script>

<template>
  <div
    v-if="message"
    class="bg-red-50 text-red-700 border border-red-200 rounded-md p-3 flex items-center mb-4"
  >
    <span class="mr-2">⚠️</span>
    {{ message }}
  </div>
</template>
