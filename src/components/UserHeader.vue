<script setup lang="ts">
import { computed } from "vue";
import type { FetchedProfile, SteamUser } from "../types";
import CopyButton from "./CopyButton.vue";
import DataSourceIndicator from "./DataSourceIndicator.vue";
import StatusBadge from "./StatusBadge.vue";

interface Props {
  user: SteamUser;
  profile?: FetchedProfile;
}

const props = defineProps<Props>();

const displayName = computed(() => {
  // Prioritize local nickname over remote steamID
  return props.user.nickname || props.profile?.steamID || props.user.steam_id;
});

const onlineStatusInfo = computed(() => {
  if (!props.profile?.onlineState) return null;

  const state = props.profile.onlineState.toLowerCase();
  const statusMap: Record<
    string,
    { type: "online" | "offline"; label: string; icon: string }
  > = {
    online: { type: "online" as const, label: "Online", icon: "🟢" },
    "in-game": { type: "online" as const, label: "In Game", icon: "🎮" },
    away: { type: "online" as const, label: "Away", icon: "🟡" },
    offline: { type: "offline" as const, label: "Offline", icon: "⚫" },
  };

  return (
    statusMap[state] || { type: "offline" as const, label: state, icon: "❓" }
  );
});

const avatarBorderClass = computed(() => {
  if (!props.profile?.onlineState) return "border-4 border-gray-500";

  const state = props.profile.onlineState.toLowerCase();
  const borderMap: Record<string, string> = {
    online: "border-4 border-green-500 shadow-lg shadow-green-500/20",
    "in-game": "border-4 border-blue-500 shadow-lg shadow-blue-500/20",
    away: "border-4 border-yellow-500 shadow-lg shadow-yellow-500/20",
    offline: "border-4 border-gray-500",
  };

  return borderMap[state] || "border-4 border-gray-500";
});

function openSteamProfile() {
  if (props.profile?.steamID64) {
    window.open(
      `https://steamcommunity.com/profiles/${props.profile.steamID64}`,
      "_blank"
    );
  }
}

function getNicknameHistory(nameHistory: Array<string>) {
  if (!nameHistory || nameHistory.length === 0) return "";
  return `(${nameHistory.join(", ")})`;
}
</script>

<template>
  <div class="flex items-start gap-4 p-4 border-b border-secondary/30">
    <!-- Avatar Section -->
    <div v-if="profile?.avatarFull" class="flex-shrink-0 relative">
      <img
        :src="profile.avatarFull"
        alt="Steam Avatar"
        class="w-24 h-24 rounded-xl object-cover shadow-md transition-all duration-300"
        :class="avatarBorderClass"
      />

      <!-- Online Status Indicator -->
      <div
        v-if="onlineStatusInfo"
        class="absolute -bottom-1 -right-1 bg-sbg rounded-full p-1 border-2 border-sbg"
      >
        <StatusBadge
          :type="onlineStatusInfo.type"
          :status="onlineStatusInfo.label"
          :icon="onlineStatusInfo.icon"
        />
      </div>
    </div>

    <!-- User Info Section -->
    <div class="flex-1 min-w-0">
      <!-- Primary Name -->
      <div class="flex items-center gap-2 mb-2">
        <h2 class="text-2xl font-bold text-text truncate">
          {{ displayName }}
        </h2>
        <DataSourceIndicator
          source="local"
          :last-updated="user.time_stamp"
          compact
        />
      </div>

      <!-- Nickname History -->
      <div
        v-if="user.name_history && user.name_history.length > 0"
        class="text-sm text-sub mb-2 font-medium"
      >
        Previous names: {{ getNicknameHistory(user.name_history) }}
      </div>

      <!-- Steam ID with Copy Button -->
      <div class="flex items-center gap-2 mb-2">
        <span class="text-text bg-bg/50 px-2 py-1 rounded-md font-mono text-sm">
          {{ user.steam_id }}
        </span>
        <CopyButton :value="user.steam_id" label="Copy ID" size="sm" />
      </div>

      <!-- Real Name (if available) -->
      <div v-if="profile?.realname" class="text-sm text-sub mb-1">
        <span class="font-medium">Real name:</span> {{ profile.realname }}
      </div>

      <!-- Location (if available) -->
      <div v-if="profile?.location" class="text-sm text-sub mb-1">
        <span class="font-medium">Location:</span> {{ profile.location }}
      </div>

      <!-- Custom URL (if available) -->
      <div v-if="profile?.customURL" class="text-sm text-sub">
        <span class="font-medium">Custom URL:</span>
        <a
          :href="`https://steamcommunity.com/id/${profile.customURL}`"
          target="_blank"
          class="text-blue-400 hover:text-blue-300 transition-colors ml-1"
        >
          {{ profile.customURL }}
        </a>
      </div>
    </div>

    <!-- Quick Actions -->
    <div class="flex flex-col gap-2 flex-shrink-0">
      <button
        v-if="profile?.steamID64"
        @click="openSteamProfile"
        class="px-3 py-1 bg-blue-600 hover:bg-blue-700 text-white text-xs rounded transition-colors"
      >
        View Profile
      </button>

      <CopyButton
        v-if="profile?.steamID64"
        :value="profile.steamID64"
        label="Copy Steam64"
        size="md"
      />
    </div>
  </div>
</template>
