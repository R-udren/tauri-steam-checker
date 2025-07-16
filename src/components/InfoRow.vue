<script setup lang="ts">
import { withDefaults } from "vue";

type Variant = "success" | "warning" | "error" | "info" | "default";

interface Props {
  label: string;
  value: string;
  description?: string;
  variant?: Variant;
  icon?: string;
}

withDefaults(defineProps<Props>(), {
  variant: "default",
});

const variantClasses: Record<Variant, string> = {
  default: "text-text",
  success: "text-green-400",
  warning: "text-yellow-400",
  error: "text-red-400",
  info: "text-blue-400",
};
</script>

<template>
  <div
    class="flex items-center justify-between p-3 bg-bg-tertiary rounded-lg border border-border"
    :title="description"
  >
    <div class="flex items-center gap-3">
      <span v-if="icon" class="text-lg">{{ icon }}</span>
      <div>
        <div class="text-xs text-text-muted uppercase tracking-wide">
          {{ label }}
        </div>
        <div class="text-sm font-medium" :class="variantClasses[variant]">
          {{ value }}
        </div>
      </div>
    </div>
    <slot name="actions"></slot>
  </div>
</template>
