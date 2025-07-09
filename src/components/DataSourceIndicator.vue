<script setup lang="ts">
import { computed, withDefaults } from "vue";

interface Props {
  source: "local" | "remote";
  lastUpdated?: number;
  reliability?: "high" | "medium" | "low";
  compact?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  reliability: "high",
  compact: false,
});

const sourceInfo = computed(() => {
  if (props.source === "local") {
    return {
      label: "Local",
      icon: "💻",
      color: "text-green-400",
      bgColor: "bg-green-500/10",
    };
  } else {
    return {
      label: "Remote",
      icon: "🌐",
      color: "text-blue-400",
      bgColor: "bg-blue-500/10",
    };
  }
});

const reliabilityInfo = computed(() => {
  const styles = {
    high: { color: "text-green-400", icon: "●" },
    medium: { color: "text-yellow-400", icon: "●" },
    low: { color: "text-red-400", icon: "●" },
  };
  return styles[props.reliability];
});

function getTimeAgo(timestamp: number) {
  const date = new Date(timestamp * 1000);
  const now = new Date();
  const seconds = Math.floor((now.getTime() - date.getTime()) / 1000);

  if (seconds < 60) return "just now";
  if (seconds < 3600) return `${Math.floor(seconds / 60)}m ago`;
  if (seconds < 86400) return `${Math.floor(seconds / 3600)}h ago`;
  return `${Math.floor(seconds / 86400)}d ago`;
}
</script>

<template>
  <div
    v-if="compact"
    class="inline-flex items-center gap-1 text-xs text-sub"
    :title="`${sourceInfo.label} data ${
      lastUpdated ? `(${getTimeAgo(lastUpdated)})` : ''
    }`"
  >
    <span class="text-xs">{{ sourceInfo.icon }}</span>
    <span :class="reliabilityInfo.color">{{ reliabilityInfo.icon }}</span>
  </div>

  <div
    v-else
    class="inline-flex items-center gap-2 px-2 py-1 rounded text-xs"
    :class="sourceInfo.bgColor"
  >
    <span class="text-xs">{{ sourceInfo.icon }}</span>
    <span :class="sourceInfo.color" class="font-medium">{{
      sourceInfo.label
    }}</span>
    <span :class="reliabilityInfo.color" class="font-medium">{{
      reliabilityInfo.icon
    }}</span>
    <span v-if="lastUpdated" class="text-sub">{{
      getTimeAgo(lastUpdated)
    }}</span>
  </div>
</template>
