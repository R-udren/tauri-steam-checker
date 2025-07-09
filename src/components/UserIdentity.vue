<script setup lang="ts">
import { computed } from "vue";
import type { FetchedProfile, SteamUser } from "../types";
import CopyButton from "./CopyButton.vue";
import StatusBadge from "./StatusBadge.vue";

interface Props {
  user: SteamUser;
  profile?: FetchedProfile;
}

const props = defineProps<Props>();

// Display name priority: profile nickname > user nickname > steam ID
const displayName = computed(() => {
  return props.profile?.steamID || props.user.nickname || props.user.steam_id;
});

// Show all previous names
const allNames = computed(() => {
  return props.user.name_history || [];
});

// Online status
const onlineStatus = computed(() => {
  if (!props.profile?.onlineState) return "offline";

  const state = props.profile.onlineState.toLowerCase();
  if (state.includes("online")) return "online";
  if (state.includes("offline")) return "offline";
  return "offline"; // Default to offline for unknown states
});

const statusLabel = computed(() => {
  if (!props.profile?.onlineState) return "Unknown";

  const state = props.profile.onlineState.toLowerCase();
  if (state.includes("online")) return "Online";
  if (state.includes("offline")) return "Offline";
  if (state.includes("away")) return "Away";
  if (state.includes("busy")) return "Busy";
  return "Unknown";
});
</script>

<template>
  <div class="flex items-start gap-3">
    <!-- Avatar -->
    <div class="flex-shrink-0">
      <img
        :src="profile?.avatarFull || '/default-avatar.png'"
        :alt="`${displayName} avatar`"
        class="w-16 h-16 rounded-lg border border-border-light"
      />
    </div>

    <!-- Identity Info -->
    <div class="flex-1 min-w-0 space-y-2">
      <!-- Name Row -->
      <div class="flex items-center justify-between">
        <h2 class="text-xl font-semibold text-text truncate">
          {{ displayName }}
        </h2>
        <div class="flex items-center gap-2 flex-shrink-0">
          <StatusBadge :type="onlineStatus" :status="statusLabel" />
          <StatusBadge v-if="user.most_recent" type="online" status="Latest" />
        </div>
      </div>

      <!-- Steam ID Row -->
      <div class="flex items-center gap-2">
        <span class="text-sm text-text-muted">Steam ID:</span>
        <code
          class="text-sm font-mono text-text-secondary bg-bg-tertiary px-2 py-1 rounded border border-border"
        >
          {{ user.steam_id }}
        </code>
        <CopyButton :value="user.steam_id" />
      </div>

      <!-- Previous Names Row -->
      <div v-if="allNames.length > 0" class="flex items-center flex-wrap gap-1">
        <span class="text-xs text-text-muted">Previously:</span>
        <template v-for="(name, index) in allNames" :key="name">
          <span
            class="text-xs text-text-secondary bg-bg-tertiary px-2 py-0.5 rounded"
          >
            {{ name }}
          </span>
          <span v-if="index < allNames.length - 1" class="text-text-muted"
            >•</span
          >
        </template>
      </div>
    </div>
  </div>
</template>
