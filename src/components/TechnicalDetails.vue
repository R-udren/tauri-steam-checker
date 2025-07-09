<script setup lang="ts">
import { computed } from "vue";
import type { FetchedProfile, SteamUser } from "../types";
import CopyButton from "./CopyButton.vue";
import ExpandableSection from "./ExpandableSection.vue";

interface Props {
  user: SteamUser;
  profile?: FetchedProfile;
}

const props = defineProps<Props>();

const identifiers = computed(() => {
  const ids = [];

  if (props.user.steam_id) {
    ids.push({
      label: "Steam ID",
      value: props.user.steam_id,
      source: "local" as const,
      description: "Primary Steam identifier",
    });
  }

  if (props.profile?.customURL) {
    ids.push({
      label: "Custom URL",
      value: props.profile.customURL,
      source: "remote" as const,
      description: "Custom profile URL slug",
    });
  }

  return ids;
});

const dataSources = computed(() => {
  const sources = [];

  if (props.user.sources && props.user.sources.length > 0) {
    sources.push({
      label: "Local Sources",
      value: props.user.sources.join(", "),
      source: "local" as const,
      description: "Where this data was collected locally",
    });
  }

  if (props.user.most_recent !== undefined) {
    sources.push({
      label: "Most Recent",
      value: props.user.most_recent ? "Yes" : "No",
      source: "local" as const,
      description: "Whether this is the most recent local data",
    });
  }

  return sources;
});

const profileDetails = computed(() => {
  const details = [];

  if (props.profile?.summary) {
    details.push({
      label: "Profile Summary",
      value: props.profile.summary,
      source: "remote" as const,
      description: "User-defined profile description",
    });
  }

  if (props.profile?.privacyState) {
    details.push({
      label: "Privacy State",
      value: props.profile.privacyState,
      source: "remote" as const,
      description: "Profile privacy setting",
    });
  }

  return details;
});

function formatNameHistory(nameHistory: string[]) {
  if (!nameHistory || nameHistory.length === 0) {
    return [];
  }

  return nameHistory.map((name, index) => ({
    name,
    position: index + 1,
  }));
}

const formattedNameHistory = computed(() => {
  return formatNameHistory(props.user.name_history || []);
});

const hasDataSources = computed(() => {
  return dataSources.value.length > 0 || props.user.time_stamp;
});
</script>

<template>
  <div class="space-y-4">
    <!-- Steam Identifiers - Steam UI Style -->
    <div v-if="identifiers.length > 0" class="space-y-3">
      <div class="flex items-center gap-2">
        <span class="text-xs text-sub uppercase tracking-wide"
          >Steam Identifiers</span
        >
        <div class="h-px bg-secondary/30 flex-1"></div>
      </div>

      <div class="grid grid-cols-2 lg:grid-cols-3 gap-x-6 gap-y-2 text-sm">
        <div v-for="id in identifiers" :key="id.label" class="space-y-1">
          <div class="text-xs text-sub uppercase tracking-wide">
            {{ id.label }}
          </div>
          <div class="flex items-center gap-2">
            <span class="text-text font-mono text-sm">{{ id.value }}</span>
            <CopyButton :value="id.value" size="sm" />
          </div>
        </div>
      </div>
    </div>

    <!-- Name History - Compact Display -->
    <div v-if="formattedNameHistory.length > 0" class="space-y-3">
      <div class="flex items-center gap-2">
        <span class="text-xs text-sub uppercase tracking-wide"
          >Previous Names</span
        >
        <div class="h-px bg-secondary/30 flex-1"></div>
        <span class="text-xs text-sub">{{ formattedNameHistory.length }}</span>
      </div>

      <div class="flex flex-wrap gap-2">
        <div
          v-for="(entry, index) in formattedNameHistory.slice(0, 8)"
          :key="index"
          class="flex items-center gap-1 px-2 py-1 bg-sbg/30 rounded text-sm border border-secondary/30"
        >
          <span class="text-text">{{ entry.name }}</span>
          <CopyButton :value="entry.name" size="sm" />
        </div>

        <ExpandableSection
          v-if="formattedNameHistory.length > 8"
          title="View All Names"
          icon="📝"
          :count="formattedNameHistory.length"
          :default-expanded="false"
        >
          <div class="grid grid-cols-2 gap-2">
            <div
              v-for="(entry, index) in formattedNameHistory"
              :key="index"
              class="flex items-center justify-between p-2 bg-sbg/20 rounded border border-secondary/30"
            >
              <span class="text-sm text-text">{{ entry.name }}</span>
              <CopyButton :value="entry.name" size="sm" />
            </div>
          </div>
        </ExpandableSection>
      </div>
    </div>

    <!-- Data Sources - Only if has meaningful data -->
    <div v-if="hasDataSources" class="space-y-3">
      <div class="flex items-center gap-2">
        <span class="text-xs text-sub uppercase tracking-wide"
          >Data Sources</span
        >
        <div class="h-px bg-secondary/30 flex-1"></div>
      </div>

      <div class="grid grid-cols-2 gap-x-6 gap-y-2 text-sm">
        <div
          v-for="source in dataSources"
          :key="source.label"
          class="space-y-1"
        >
          <div class="text-xs text-sub uppercase tracking-wide">
            {{ source.label }}
          </div>
          <div class="text-text font-medium">{{ source.value }}</div>
        </div>

        <div v-if="user.time_stamp" class="space-y-1">
          <div class="text-xs text-sub uppercase tracking-wide">
            Last Updated
          </div>
          <div class="text-text font-medium">
            {{ new Date(user.time_stamp * 1000).toLocaleDateString() }}
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
