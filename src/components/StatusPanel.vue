<script setup lang="ts">
import { computed } from "vue";
import type { FetchedProfile, SteamUser } from "../types";
import StatusBadge from "./StatusBadge.vue";

interface Props {
  user: SteamUser;
  profile?: FetchedProfile;
}

const props = defineProps<Props>();

const securityBadges = computed(() => {
  const badges = [];

  if (props.profile?.vacBanned === "1") {
    badges.push({
      type: "vac" as const,
      status: "VAC Banned",
      icon: "🚫",
    });
  }

  if (props.profile?.tradeBanState === "Banned") {
    badges.push({
      type: "trade" as const,
      status: "Trade Banned",
      icon: "🔒",
    });
  }

  if (props.profile?.isLimitedAccount === "1") {
    badges.push({
      type: "limited" as const,
      status: "Limited Account",
      icon: "⚠️",
    });
  }

  return badges;
});

function getFancyDatetime(timestamp: number) {
  const date = new Date(timestamp * 1000);
  return date
    .toLocaleString("en-GB", {
      day: "2-digit",
      month: "2-digit",
      year: "numeric",
      hour: "2-digit",
      minute: "2-digit",
      hour12: false,
    })
    .replace(/\//g, ".")
    .replace(/,/g, " ");
}

function getTimeAgo(timestamp: number) {
  const date = new Date(timestamp * 1000);
  const now = new Date();
  const seconds = Math.floor((now.getTime() - date.getTime()) / 1000);

  let interval = Math.floor(seconds / 31536000);
  if (interval > 1) return `${interval} years ago`;
  interval = Math.floor(seconds / 2592000);
  if (interval > 1) return `${interval} months ago`;
  interval = Math.floor(seconds / 86400);
  if (interval > 1) return `${interval} days ago`;
  interval = Math.floor(seconds / 3600);
  if (interval > 1) return `${interval} hours ago`;
  interval = Math.floor(seconds / 60);
  if (interval > 1) return `${interval} minutes ago`;
  return `${seconds} seconds ago`;
}

function getVisibilityText(state: string) {
  const visibilityMap: Record<string, string> = {
    "1": "Private",
    "2": "Friends Only",
    "3": "Public",
  };
  return visibilityMap[state] || "Unknown";
}

function getStatusIcon(status: string) {
  const iconMap: Record<string, string> = {
    online: "🟢",
    "in-game": "🎮",
    away: "🟡",
    offline: "⚫",
  };
  return iconMap[status.toLowerCase()] || "❓";
}
</script>

<template>
  <div class="p-4 border-b border-secondary/30">
    <!-- Security Status Row -->
    <div class="flex items-center gap-6 mb-4">
      <div class="flex items-center gap-2">
        <span class="text-xs text-sub uppercase tracking-wide">Security</span>
        <div class="h-px bg-secondary/30 flex-1 w-12"></div>
      </div>
      <div class="flex gap-2">
        <StatusBadge
          v-if="securityBadges.length > 0"
          v-for="badge in securityBadges"
          :key="badge.status"
          :type="badge.type"
          :status="badge.status"
          :icon="badge.icon"
        />
        <StatusBadge v-else type="online" status="Clean Account" icon="✅" />
      </div>
    </div>

    <!-- Info Grid - Steam UI Style -->
    <div class="grid grid-cols-2 lg:grid-cols-4 gap-x-8 gap-y-3 text-sm">
      <!-- Member Since -->
      <div v-if="profile?.memberSince" class="space-y-1">
        <div class="text-xs text-sub uppercase tracking-wide">
          Account created
        </div>
        <div class="text-text font-medium">{{ profile.memberSince }}</div>
      </div>

      <!-- Profile Privacy -->
      <div v-if="profile?.VisibilityState" class="space-y-1">
        <div class="text-xs text-sub uppercase tracking-wide">Privacy</div>
        <div class="text-text font-medium">
          {{ getVisibilityText(profile.VisibilityState) }}
        </div>
      </div>

      <!-- Last Activity -->
      <div v-if="user.time_stamp" class="space-y-1">
        <div class="text-xs text-sub uppercase tracking-wide">Last seen</div>
        <div class="text-text font-medium">
          {{ getFancyDatetime(user.time_stamp) }}
        </div>
        <div class="text-xs text-sub">{{ getTimeAgo(user.time_stamp) }}</div>
      </div>

      <!-- Online Status -->
      <div v-if="profile?.onlineState" class="space-y-1">
        <div class="text-xs text-sub uppercase tracking-wide">Status</div>
        <div class="text-text font-medium flex items-center gap-2">
          {{ profile.onlineState }}
          <span class="text-xs">{{ getStatusIcon(profile.onlineState) }}</span>
        </div>
      </div>

      <!-- Location -->
      <div v-if="profile?.location" class="space-y-1">
        <div class="text-xs text-sub uppercase tracking-wide">Location</div>
        <div class="text-text font-medium">{{ profile.location }}</div>
      </div>

      <!-- Trade Ban Status -->
      <div
        v-if="profile?.tradeBanState && profile.tradeBanState !== 'None'"
        class="space-y-1"
      >
        <div class="text-xs text-sub uppercase tracking-wide">Trade status</div>
        <div class="text-orange-400 font-medium">
          {{ profile.tradeBanState }}
        </div>
      </div>

      <!-- Limited Account -->
      <div v-if="profile?.isLimitedAccount === '1'" class="space-y-1">
        <div class="text-xs text-sub uppercase tracking-wide">Account type</div>
        <div class="text-yellow-400 font-medium">Limited</div>
      </div>

      <!-- Status Message -->
      <div v-if="profile?.stateMessage" class="space-y-1 col-span-2">
        <div class="text-xs text-sub uppercase tracking-wide">
          Status message
        </div>
        <div class="text-text font-medium italic">
          "{{ profile.stateMessage }}"
        </div>
      </div>
    </div>
  </div>
</template>
