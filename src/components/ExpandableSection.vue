<script setup lang="ts">
import { ref, withDefaults } from "vue";

interface Props {
  title: string;
  defaultExpanded?: boolean;
  hideEmpty?: boolean;
  icon?: string;
  count?: number;
}

const props = withDefaults(defineProps<Props>(), {
  defaultExpanded: false,
  hideEmpty: false,
});

const isExpanded = ref(props.defaultExpanded);
const contentRef = ref<HTMLElement>();

function toggleExpanded() {
  isExpanded.value = !isExpanded.value;
}
</script>

<template>
  <div class="border border-secondary/50 rounded-lg bg-sbg/50">
    <button
      @click="toggleExpanded"
      class="w-full flex items-center justify-between p-3 hover:bg-secondary/20 transition-colors rounded-lg"
    >
      <div class="flex items-center gap-2">
        <span v-if="icon" class="text-sm">{{ icon }}</span>
        <span class="font-medium text-text">{{ title }}</span>
        <span
          v-if="count !== undefined"
          class="text-xs text-sub bg-secondary/30 px-2 py-0.5 rounded-full"
        >
          {{ count }}
        </span>
      </div>
      <svg
        :class="{ 'rotate-180': isExpanded }"
        class="w-4 h-4 text-sub transition-transform duration-200"
        fill="none"
        stroke="currentColor"
        viewBox="0 0 24 24"
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="M19 9l-7 7-7-7"
        />
      </svg>
    </button>

    <Transition
      enter-active-class="transition-all duration-300 ease-out"
      enter-from-class="opacity-0 max-h-0"
      enter-to-class="opacity-100 max-h-96"
      leave-active-class="transition-all duration-300 ease-in"
      leave-from-class="opacity-100 max-h-96"
      leave-to-class="opacity-0 max-h-0"
    >
      <div v-show="isExpanded" class="overflow-hidden">
        <div ref="contentRef" class="p-3 pt-0 border-t border-secondary/30">
          <slot />
        </div>
      </div>
    </Transition>
  </div>
</template>
