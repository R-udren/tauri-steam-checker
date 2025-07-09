<script setup lang="ts">
import { computed } from "vue";
import type { FetchedProfile, SteamUser } from "../types";
import DataSourceIndicator from "./DataSourceIndicator.vue";
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

const accountInfo = computed(() => {
  const info = [];

  if (props.profile?.memberSince) {
    info.push({
      label: "Member since",
      value: props.profile.memberSince,
      source: "remote" as const,
    });
  }

  if (props.profile?.VisibilityState) {
    const visibilityMap: Record<string, string> = {
      "1": "Private",
      "2": "Friends Only",
      "3": "Public",
    };
    info.push({
      label: "Profile",
      value: visibilityMap[props.profile.VisibilityState] || "Unknown",
      source: "remote" as const,
    });
  }

  return info;
});

const activityInfo = computed(() => {
  const info = [];

  if (props.user.time_stamp) {
    info.push({
      label: "Last activity",
      value: props.user.time_stamp,
      source: "local" as const,
    });
  }

  if (props.profile?.onlineState) {
    info.push({
      label: "Status",
      value: props.profile.onlineState,
      source: "remote" as const,
    });
  }

  if (props.profile?.stateMessage) {
    info.push({
      label: "Status message",
      value: props.profile.stateMessage,
      source: "remote" as const,
    });
  }

  return info;
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
</script>

<template>
  <div
    class="grid grid-cols-1 md:grid-cols-3 gap-4 p-4 border-b border-secondary/30"
  >
    <!-- Security Status -->
    <div class="space-y-2">
      <h3 class="text-sm font-semibold text-text mb-2">Security Status</h3>
      <div v-if="securityBadges.length > 0" class="flex flex-wrap gap-2">
        <StatusBadge
          v-for="badge in securityBadges"
          :key="badge.status"
          :type="badge.type"
          :status="badge.status"
          :icon="badge.icon"
        />
      </div>
      <div v-else class="text-sm text-sub">
        <StatusBadge type="online" status="Clean Account" icon="✅" />
      </div>
    </div>

    <!-- Account Information -->
    <div class="space-y-2">
      <h3 class="text-sm font-semibold text-text mb-2">Account Info</h3>
      <div class="space-y-1">
        <div
          v-for="info in accountInfo"
          :key="info.label"
          class="flex items-center justify-between text-sm"
        >
          <span class="text-sub">{{ info.label }}:</span>
          <div class="flex items-center gap-2">
            <span class="text-text">{{ info.value }}</span>
            <DataSourceIndicator :source="info.source" compact />
          </div>
        </div>
      </div>
    </div>

    <!-- Activity Information -->
    <div class="space-y-2">
      <h3 class="text-sm font-semibold text-text mb-2">Activity</h3>
      <div class="space-y-1">
        <div v-for="info in activityInfo" :key="info.label" class="text-sm">
          <div class="flex items-center justify-between">
            <span class="text-sub">{{ info.label }}:</span>
            <DataSourceIndicator :source="info.source" compact />
          </div>
          <div class="mt-1">
            <span
              v-if="
                info.label === 'Last activity' && typeof info.value === 'number'
              "
              class="text-text"
            >
              {{ getFancyDatetime(info.value) }}
              <span class="text-sub text-xs">
                ({{ getTimeAgo(info.value) }})
              </span>
            </span>
            <span v-else class="text-text">{{ info.value }}</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
