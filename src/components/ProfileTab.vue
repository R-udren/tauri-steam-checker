<script setup lang="ts">
import { computed } from "vue";
import type { FetchedProfile, SteamUser } from "../types";

interface Props {
  user: SteamUser;
  profile?: FetchedProfile;
}

const props = defineProps<Props>();

// Profile information
const profileInfo = computed(() => {
  const info = [];

  if (props.profile?.realname) {
    info.push({
      label: "Real Name",
      value: props.profile.realname,
    });
  }

  if (props.profile?.location) {
    info.push({
      label: "Location",
      value: props.profile.location,
    });
  }

  if (props.profile?.memberSince) {
    info.push({
      label: "Member Since",
      value: new Date(props.profile.memberSince).toLocaleDateString(),
    });
  }

  return info;
});

// Profile status
const profileStatus = computed(() => {
  const status = [];

  if (props.profile?.onlineState) {
    status.push({
      label: "Online State",
      value: props.profile.onlineState,
    });
  }

  if (props.profile?.stateMessage) {
    status.push({
      label: "Status Message",
      value: props.profile.stateMessage,
    });
  }

  return status;
});

// Name history with better formatting
const nameHistory = computed(() => {
  if (!props.user.name_history?.length) return [];

  return props.user.name_history.map((name, index) => ({
    name,
    position: index + 1,
    isCurrent: index === 0,
  }));
});
</script>

<template>
  <div class="space-y-6">
    <!-- Profile Information -->
    <div v-if="profileInfo.length > 0" class="space-y-3">
      <h3
        class="text-sm font-medium text-text-secondary flex items-center gap-2"
      >
        👤 Profile Information
      </h3>

      <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
        <div
          v-for="info in profileInfo"
          :key="info.label"
          class="p-3 bg-bg-tertiary rounded-lg border border-border"
        >
          <div class="text-xs text-text-muted uppercase tracking-wide mb-1">
            {{ info.label }}
          </div>
          <div class="text-sm text-text">{{ info.value }}</div>
        </div>
      </div>
    </div>

    <!-- Profile Summary -->
    <div v-if="profile?.summary" class="space-y-3">
      <h3
        class="text-sm font-medium text-text-secondary flex items-center gap-2"
      >
        📝 Profile Summary
      </h3>

      <div class="p-4 bg-bg-tertiary rounded-lg border border-border">
        <p class="text-sm text-text leading-relaxed">{{ profile.summary }}</p>
      </div>
    </div>

    <!-- Current Status -->
    <div v-if="profileStatus.length > 0" class="space-y-3">
      <h3
        class="text-sm font-medium text-text-secondary flex items-center gap-2"
      >
        💬 Current Status
      </h3>

      <div class="space-y-2">
        <div
          v-for="status in profileStatus"
          :key="status.label"
          class="p-3 bg-bg-tertiary rounded-lg border border-border"
        >
          <div class="text-xs text-text-muted uppercase tracking-wide mb-1">
            {{ status.label }}
          </div>
          <div class="text-sm text-text">{{ status.value }}</div>
        </div>
      </div>
    </div>

    <!-- Name History -->
    <div v-if="nameHistory.length > 0" class="space-y-3">
      <h3
        class="text-sm font-medium text-text-secondary flex items-center gap-2"
      >
        📜 Name History
        <span class="text-xs text-text-muted"
          >({{ nameHistory.length }} names)</span
        >
      </h3>

      <div class="space-y-2 max-h-60 overflow-y-auto">
        <div
          v-for="(entry, index) in nameHistory"
          :key="index"
          :class="[
            'flex items-center justify-between p-3 rounded-lg border',
            entry.isCurrent
              ? 'bg-primary/10 border-primary/30'
              : 'bg-bg-tertiary border-border',
          ]"
        >
          <div class="flex items-center gap-3">
            <div class="text-xs text-text-muted font-mono w-6">
              #{{ entry.position }}
            </div>
            <div class="text-sm text-text">{{ entry.name }}</div>
          </div>
          <div v-if="entry.isCurrent" class="text-xs text-primary font-medium">
            Current
          </div>
        </div>
      </div>
    </div>

    <!-- Empty State -->
    <div
      v-if="
        profileInfo.length === 0 &&
        !profile?.summary &&
        profileStatus.length === 0 &&
        nameHistory.length === 0
      "
      class="text-center py-8"
    >
      <div class="text-4xl mb-2">👤</div>
      <div class="text-text-muted">No profile data available</div>
    </div>
  </div>
</template>
