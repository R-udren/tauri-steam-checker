<script setup lang="ts">
import { computed, withDefaults } from "vue";
import type { App } from "../types";

interface Props {
  app: App;
  showPlaytime?: boolean;
  showLastPlayed?: boolean;
  size?: "sm" | "md" | "lg";
}

const props = withDefaults(defineProps<Props>(), {
  showPlaytime: true,
  showLastPlayed: true,
  size: "md",
});

const playtimeHours = computed(() => {
  return Math.round((props.app.playtime_minutes / 60) * 10) / 10;
});

const lastPlayedFormatted = computed(() => {
  if (!props.app.last_played) return "Never";

  const date = new Date(props.app.last_played * 1000);
  const now = new Date();
  const seconds = Math.floor((now.getTime() - date.getTime()) / 1000);

  if (seconds < 86400) return "Today";
  if (seconds < 172800) return "Yesterday";
  if (seconds < 2592000) return `${Math.floor(seconds / 86400)} days ago`;
  if (seconds < 31536000) return `${Math.floor(seconds / 2592000)} months ago`;
  return `${Math.floor(seconds / 31536000)} years ago`;
});

const cardSizeClasses = computed(() => {
  const sizes = {
    sm: "p-2",
    md: "p-3",
    lg: "p-4",
  };
  return sizes[props.size];
});

// Steam game icon URL (approximate - Steam uses app_id for icons)
const gameIconUrl = computed(() => {
  return `https://cdn.akamai.steamstatic.com/steam/apps/${props.app.app_id}/library_600x900.jpg`;
});

const gameName = computed(() => {
  // For now, we'll show the app_id since we don't have game names
  // This could be enhanced with a game database lookup
  return `App ${props.app.app_id}`;
});
</script>

<template>
  <div
    class="border border-secondary/30 rounded-lg bg-sbg/30 hover:bg-sbg/50 transition-all duration-200 hover:border-secondary/50"
    :class="cardSizeClasses"
  >
    <div class="flex items-center gap-3">
      <!-- Game Icon -->
      <div class="flex-shrink-0">
        <div
          class="w-12 h-12 bg-secondary/20 rounded border border-secondary/30 flex items-center justify-center overflow-hidden"
        >
          <img
            :src="gameIconUrl"
            :alt="`${gameName} icon`"
            class="w-full h-full object-cover"
            @error="($event.target as HTMLImageElement).style.display = 'none'"
          />
          <span class="text-xs text-sub font-mono">{{ app.app_id }}</span>
        </div>
      </div>

      <!-- Game Info -->
      <div class="flex-1 min-w-0">
        <div class="font-medium text-text text-sm truncate">
          {{ gameName }}
        </div>

        <div class="flex items-center gap-4 mt-1">
          <div v-if="showPlaytime" class="text-xs text-sub">
            <span class="font-medium">{{ playtimeHours }}h</span> played
          </div>

          <div v-if="showLastPlayed" class="text-xs text-sub">
            {{ lastPlayedFormatted }}
          </div>
        </div>

        <!-- Playtime bar for visual representation -->
        <div v-if="showPlaytime && playtimeHours > 0" class="mt-2">
          <div class="w-full bg-secondary/20 rounded-full h-1.5">
            <div
              class="bg-blue-500/60 h-1.5 rounded-full transition-all duration-300"
              :style="{
                width: `${Math.min((playtimeHours / 100) * 100, 100)}%`,
              }"
            ></div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
