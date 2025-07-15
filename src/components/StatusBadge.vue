<script setup lang="ts">
import { computed, withDefaults } from "vue";

interface Props {
  variant?: "success" | "warning" | "error" | "info" | "default";
  text: string;
  icon?: string;
}

const props = withDefaults(defineProps<Props>(), {
  variant: "default",
});

const badgeStyles = computed(() => {
  const baseClasses =
    "inline-flex items-center gap-1 px-2 py-1 rounded-md text-xs font-medium transition-all";

  const variantStyles = {
    success: "bg-green-500/20 text-green-400 border border-green-500/30",
    warning: "bg-orange-500/20 text-orange-400 border border-orange-500/30",
    error: "bg-red-500/20 text-red-400 border border-red-500/30",
    info: "bg-blue-500/20 text-blue-400 border border-blue-500/30",
    default: "bg-gray-500/20 text-gray-400 border border-gray-500/30",
  };

  return `${baseClasses} ${variantStyles[props.variant]}`;
});
</script>

<template>
  <span :class="badgeStyles">
    <span v-if="icon" class="text-xs">{{ icon }}</span>
    {{ text }}
  </span>
</template>
