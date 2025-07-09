<script setup lang="ts">
import { computed, withDefaults } from "vue";

interface Props {
  type: "vac" | "trade" | "limited" | "online" | "privacy" | "offline";
  status: string;
  severity?: "info" | "warning" | "error";
  icon?: string;
}

const props = withDefaults(defineProps<Props>(), {
  severity: "info",
});

const badgeStyles = computed(() => {
  const baseClasses =
    "inline-flex items-center gap-1 px-2 py-1 rounded-md text-xs font-medium transition-all";

  const typeStyles = {
    vac: "bg-red-500/20 text-red-400 border border-red-500/30",
    trade: "bg-orange-500/20 text-orange-400 border border-orange-500/30",
    limited: "bg-yellow-500/20 text-yellow-400 border border-yellow-500/30",
    online: "bg-green-500/20 text-green-400 border border-green-500/30",
    offline: "bg-gray-500/20 text-gray-400 border border-gray-500/30",
    privacy: "bg-blue-500/20 text-blue-400 border border-blue-500/30",
  };

  return `${baseClasses} ${typeStyles[props.type]}`;
});
</script>

<template>
  <span :class="badgeStyles">
    <span v-if="icon" class="text-xs">{{ icon }}</span>
    {{ status }}
  </span>
</template>
