<script setup lang="ts">
import { computed, ref, withDefaults } from "vue";

interface Props {
  value: string;
  label?: string;
  size?: "sm" | "md";
}

const props = withDefaults(defineProps<Props>(), {
  size: "sm",
});

const copied = ref(false);

async function copyToClipboard() {
  try {
    await navigator.clipboard.writeText(props.value);
    copied.value = true;
    setTimeout(() => {
      copied.value = false;
    }, 2000);
  } catch (err) {
    console.error("Failed to copy:", err);
  }
}

const buttonClasses = computed(() => {
  const baseClasses =
    "inline-flex items-center gap-1 rounded transition-all hover:bg-secondary/20";
  const sizeClasses = {
    sm: "px-1.5 py-0.5 text-xs",
    md: "px-2 py-1 text-sm",
  };
  return `${baseClasses} ${sizeClasses[props.size]}`;
});
</script>

<template>
  <button
    @click="copyToClipboard"
    :class="buttonClasses"
    class="text-sub hover:text-text"
    :title="`Copy ${label || 'value'}`"
  >
    <svg
      v-if="!copied"
      class="w-3 h-3"
      fill="none"
      stroke="currentColor"
      viewBox="0 0 24 24"
    >
      <path
        stroke-linecap="round"
        stroke-linejoin="round"
        stroke-width="2"
        d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"
      />
    </svg>
    <svg
      v-else
      class="w-3 h-3 text-green-400"
      fill="none"
      stroke="currentColor"
      viewBox="0 0 24 24"
    >
      <path
        stroke-linecap="round"
        stroke-linejoin="round"
        stroke-width="2"
        d="M5 13l4 4L19 7"
      />
    </svg>
    <span v-if="label">{{ copied ? "Copied!" : label }}</span>
  </button>
</template>
