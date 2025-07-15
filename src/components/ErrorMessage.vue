<script setup lang="ts">
import { watch } from "vue";

const props = defineProps<{
  message: string | null;
}>();

const emit = defineEmits(["clear"]);

const clearError = () => {
  emit("clear");
};

watch(
  () => props.message,
  () => {},
  { immediate: true }
);
</script>

<template>
  <transition name="fade">
    <div
      v-if="message"
      class="bg-red-500/20 text-red-50 border border-red-500 rounded-md p-3 flex items-center justify-between mb-4"
    >
      <div class="flex items-center">
        <span class="mr-2 text-red-600">⚠️</span>
        {{ message }}
      </div>
      <button
        @click="clearError"
        class="ml-4 text-red-400 hover:text-red-600 focus:outline-none transition-colors"
        aria-label="Dismiss error message"
      >
        <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
          <path
            fill-rule="evenodd"
            d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z"
            clip-rule="evenodd"
          ></path>
        </svg>
      </button>
    </div>
  </transition>
</template>

<style scoped>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s ease;
}
.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
