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
  <div class="border border-secondary/30 rounded-lg bg-sbg/30 mb-3">
    <button
      @click="toggleExpanded"
      class="w-full flex items-center justify-between p-3 hover:bg-secondary/20 transition-colors rounded-lg"
    >
      <div class="flex items-center gap-2">
        <span v-if="icon" class="text-sm">{{ icon }}</span>
        <span class="font-medium text-text text-sm">{{ title }}</span>
        <span
          v-if="count !== undefined"
          class="text-xs text-sub bg-secondary/30 px-1.5 py-0.5 rounded-full"
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
      enter-active-class="transition-all duration-200 ease-out"
      enter-from-class="opacity-0 max-h-0"
      enter-to-class="opacity-100"
      leave-active-class="transition-all duration-200 ease-in"
      leave-from-class="opacity-100"
      leave-to-class="opacity-0 max-h-0"
    >
      <div v-show="isExpanded" class="overflow-hidden">
        <div ref="contentRef" class="px-3 pb-3 border-t border-secondary/30">
          <slot />
        </div>
      </div>
    </Transition>
  </div>
</template>
