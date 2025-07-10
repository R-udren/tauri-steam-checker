<script setup lang="ts">
import { computed } from "vue";
import type { App, FetchedProfile, SteamUser } from "../types";

interface Props {
  user: SteamUser;
  profile?: FetchedProfile;
  totalHours: number;
  mostRecentGame: App | null;
}

const props = defineProps<Props>();

// Format last active time
const lastActive = computed(() => {
  if (!props.user.time_stamp) return "Unknown";

  const date = new Date(props.user.time_stamp * 1000);
  const now = new Date();
  const diffMs = now.getTime() - date.getTime();
  const diffHours = Math.floor(diffMs / (1000 * 60 * 60));
  const diffDays = Math.floor(diffHours / 24);

  if (diffHours < 1) return "Less than 1 hour ago";
  if (diffHours < 24) return `${diffHours} hours ago`;
  if (diffDays < 7) return `${diffDays} days ago`;
  return date.toLocaleDateString();
});

// Game count
const gameCount = computed(() => props.user.apps.length);

// Member since
const memberSince = computed(() => {
  if (!props.profile?.memberSince) return null;
  return new Date(props.profile.memberSince).getFullYear();
});
</script>

<template>
  <div
    class="flex items-center justify-between py-2 px-3 bg-bg-tertiary rounded-lg border border-border"
  >
    <div class="flex items-center gap-6 text-sm">
      <!-- Last Active -->
      <div class="flex items-center gap-2">
        <span class="text-text-muted">Last Active:</span>
        <span class="text-text-secondary font-medium">{{ lastActive }}</span>
      </div>

      <!-- Games Count -->
      <div class="flex items-center gap-2">
        <span class="text-text-muted">Games:</span>
        <span class="text-text-secondary font-medium">{{ gameCount }}</span>
      </div>

      <!-- Total Hours -->
      <div class="flex items-center gap-2">
        <span class="text-text-muted">Hours:</span>
        <span class="text-text-secondary font-medium">{{ totalHours }}h</span>
      </div>
    </div>

    <!-- Right side info -->
    <div class="flex items-center gap-4 text-sm">
      <!-- Member Since -->
      <div v-if="memberSince" class="flex items-center gap-2">
        <span class="text-text-muted">Member since:</span>
        <span class="text-text-secondary font-medium">{{ memberSince }}</span>
      </div>

      <!-- Active user indicator -->
      <div class="flex items-center gap-1" v-if="user.most_recent">
        <div
          :class="['w-2 h-2 rounded-full', user.most_recent ?? 'bg-success']"
        ></div>
        <span class="text-xs text-text-muted">
          {{ user.most_recent ?? "Latest" }}
        </span>
      </div>
    </div>
  </div>
</template>
