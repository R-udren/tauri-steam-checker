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
  const profileName = props.profile?.steamID;
  const userNickname = props.user.nickname;
  const steamId = props.user.steam_id;
  return { profileName, userNickname, steamId };
});

// Show all previous names
const allNames = computed(() => {
  return props.user.name_history || [];
});

// Online status
const onlineStatusVariant = computed(() => {
  if (!props.profile?.onlineState) return "default";

  const state = props.profile.onlineState.toLowerCase();
  if (state.includes("online")) return "success";
  if (state.includes("offline")) return "default";
  return "default"; // Default to offline for unknown states
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

// Compute Steam profile URL
const steamProfileUrl = computed(() => {
  if (props.profile?.customURL) {
    return `https://steamcommunity.com/id/${props.profile.customURL}`;
  }
  return `https://steamcommunity.com/profiles/${props.user.steam_id}`;
});
</script>

<template>
  <div class="flex items-start gap-3">
    <!-- Avatar -->
    <div class="flex-shrink-0">
      <img
        :src="
          profile?.avatarFull ||
          'https://avatars.akamai.steamstatic.com/fef49e7fa7e1997310d705b2a6158ff8dc1cdfeb_full.jpg'
        "
        :alt="`${
          displayName.profileName ||
          displayName.userNickname ||
          displayName.steamId
        } avatar`"
        class="size-24 rounded border border-border-light"
        title="Steam profile avatar"
      />
    </div>

    <!-- Identity Info -->
    <div class="flex-1 min-w-0 space-y-2">
      <!-- Name Row -->
      <div class="flex items-center justify-between">
        <h2
          class="text-2xl font-semibold text-text truncate flex items-center gap-2"
        >
          <span
            v-if="displayName.profileName"
            class="mr-1"
            title="Current Steam profile name (from Steam)"
          >
            {{ displayName.profileName }}
          </span>
          <span
            v-if="!displayName.userNickname && !displayName.profileName"
            class="text-text-secondary text-lg"
            title="Steam ID"
          >
            {{ displayName.steamId }}
          </span>
          <span
            v-if="
              displayName.userNickname &&
              displayName.userNickname !== displayName.profileName
            "
            class="text-text-secondary"
            title="Local nickname (from local data)"
          >
            ({{ displayName.userNickname }})
          </span>
          <a
            :href="steamProfileUrl"
            class="ml-2 text-primary/75 underline underline-offset-2 hover:text-primary-hover transition-colors text-base"
            target="_blank"
            rel="noopener noreferrer"
            title="Open Steam profile in browser"
          >
            View Profile
          </a>
        </h2>
        <div class="flex items-center gap-2 flex-shrink-0">
          <StatusBadge
            :variant="
              user.sources.length < 3
                ? 'error'
                : user.sources.length < 5
                ? 'warning'
                : 'info'
            "
            :text="`Sources: ${user.sources.length}`"
          >
          </StatusBadge>
          <StatusBadge
            :variant="onlineStatusVariant"
            :text="statusLabel"
            title="Current online status"
          />
          <StatusBadge
            v-if="user.most_recent"
            variant="success"
            text="Latest"
            title="This is the most recent data"
          />
        </div>
      </div>

      <!-- Steam ID Row -->
      <div class="flex items-center gap-2">
        <span
          class="text-sm text-text-muted"
          title="Unique 64-bit Steam identifier"
          >Steam ID:</span
        >
        <code
          class="text-sm font-mono text-text-secondary bg-bg-tertiary px-2 py-1 rounded border border-border"
          title="Unique 64-bit Steam identifier"
        >
          {{ user.steam_id }}
        </code>
        <CopyButton :value="user.steam_id" label="Copy Steam ID" />
      </div>

      <!-- Previous Names Row -->
      <div v-if="allNames.length > 0" class="flex items-center flex-wrap gap-1">
        <span
          class="text-xs text-text-muted"
          title="Previous Steam profile names"
          >Previously:</span
        >
        <template v-for="(name, index) in allNames" :key="name">
          <span
            class="text-xs text-text-secondary bg-bg-tertiary px-2 py-0.5 rounded"
            title="Previous Steam profile name"
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
