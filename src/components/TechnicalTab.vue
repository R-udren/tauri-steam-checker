<script setup lang="ts">
import { computed } from "vue";
import type { FetchedProfile, SteamUser } from "../types";
import CopyButton from "./CopyButton.vue";

interface Props {
  user: SteamUser;
  profile?: FetchedProfile;
}

const props = defineProps<Props>();

// Steam identifiers
const identifiers = computed(() => {
  const ids = [];

  if (props.user.steam_id) {
    ids.push({
      label: "Steam ID64",
      value: props.user.steam_id,
    });
  }

  if (props.profile?.customURL) {
    ids.push({
      label: "Custom URL",
      value: props.profile.customURL,
    });
  }

  return ids;
});

// Data sources info
const dataInfo = computed(() => {
  const info = [];

  if (props.user.sources?.length > 0) {
    info.push({
      label: "Local Sources",
      value: props.user.sources.join(", "),
    });
  }

  if (props.user.time_stamp) {
    info.push({
      label: "Last Updated",
      value: new Date(props.user.time_stamp * 1000).toLocaleString(),
    });
  }

  info.push({
    label: "Data Freshness",
    value: props.user.most_recent ? "Most Recent" : "Cached",
  });

  return info;
});

// Privacy and security info
const securityInfo = computed(() => {
  const info = [];

  if (props.profile?.privacyState) {
    info.push({
      label: "Privacy State",
      value: props.profile.privacyState,
    });
  }

  if (props.profile?.VisibilityState) {
    info.push({
      label: "Visibility State",
      value: props.profile.VisibilityState,
    });
  }

  if (props.profile?.vacBanned) {
    info.push({
      label: "VAC Status",
      value: props.profile.vacBanned,
    });
  }

  if (props.profile?.tradeBanState) {
    info.push({
      label: "Trade Ban",
      value: props.profile.tradeBanState,
    });
  }

  if (props.profile?.isLimitedAccount) {
    info.push({
      label: "Limited Account",
      value: props.profile.isLimitedAccount,
    });
  }

  return info;
});
</script>

<template>
  <div class="space-y-6">
    <!-- Steam Identifiers -->
    <div v-if="identifiers.length > 0" class="space-y-3">
      <h3
        class="text-sm font-medium text-text-secondary flex items-center gap-2"
      >
        🆔 Steam Identifiers
      </h3>

      <div class="space-y-2">
        <div
          v-for="id in identifiers"
          :key="id.label"
          class="flex items-center justify-between p-3 bg-bg-tertiary rounded-lg border border-border"
        >
          <div>
            <div class="text-xs text-text-muted uppercase tracking-wide">
              {{ id.label }}
            </div>
            <code class="text-sm font-mono text-text">{{ id.value }}</code>
          </div>
          <CopyButton :value="id.value" />
        </div>
      </div>
    </div>

    <!-- Data Sources -->
    <div v-if="dataInfo.length > 0" class="space-y-3">
      <h3
        class="text-sm font-medium text-text-secondary flex items-center gap-2"
      >
        📊 Data Information
      </h3>

      <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
        <div
          v-for="info in dataInfo"
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

    <!-- Security & Privacy -->
    <div v-if="securityInfo.length > 0" class="space-y-3">
      <h3
        class="text-sm font-medium text-text-secondary flex items-center gap-2"
      >
        🔒 Privacy & Security
      </h3>

      <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
        <div
          v-for="info in securityInfo"
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

    <!-- Empty State -->
    <div
      v-if="
        identifiers.length === 0 &&
        dataInfo.length === 0 &&
        securityInfo.length === 0
      "
      class="text-center py-8"
    >
      <div class="text-4xl mb-2">⚙️</div>
      <div class="text-text-muted">No technical data available</div>
    </div>
  </div>
</template>
