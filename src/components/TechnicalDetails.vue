<script setup lang="ts">
import { computed } from "vue";
import type { FetchedProfile, SteamUser } from "../types";
import CopyButton from "./CopyButton.vue";
import DataSourceIndicator from "./DataSourceIndicator.vue";
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

  if (props.profile?.steamID64) {
    ids.push({
      label: "Steam ID64",
      value: props.profile.steamID64,
      source: "remote" as const,
      description: "64-bit Steam identifier",
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
</script>

<template>
  <div class="space-y-4">
    <!-- Name History -->
    <ExpandableSection
      title="Name History"
      icon="📝"
      :count="user.name_history?.length || 0"
      :default-expanded="false"
    >
      <div class="space-y-2">
        <div class="text-xs text-sub mb-3 flex items-center gap-2">
          💻 Local name change history
        </div>

        <div
          v-if="formattedNameHistory.length === 0"
          class="text-sm text-sub text-center py-4"
        >
          No previous names recorded
        </div>

        <div v-else class="space-y-2">
          <div
            v-for="entry in formattedNameHistory"
            class="flex items-center justify-between p-2 rounded bg-sbg/20 border border-secondary/30"
          >
            <div class="flex items-center gap-3">
              <span class="text-sm text-text">{{ entry.name }}</span>
            </div>
            <div class="flex items-center gap-2">
              <CopyButton :value="entry.name" size="sm" />
            </div>
          </div>
        </div>
      </div>
    </ExpandableSection>

    <!-- Data Sources & Metadata -->
    <ExpandableSection
      title="Data Sources"
      icon="📊"
      :count="dataSources.length"
      :default-expanded="false"
    >
      <div class="space-y-3">
        <div
          v-for="source in dataSources"
          :key="source.label"
          class="p-3 bg-sbg/20 rounded border border-secondary/30"
        >
          <div class="flex items-center justify-between mb-2">
            <div class="flex items-center gap-2">
              <span class="font-medium text-text text-sm">{{
                source.label
              }}</span>
              <DataSourceIndicator :source="source.source" compact />
            </div>
          </div>
          <div class="text-sm text-text">
            {{ source.value }}
          </div>
          <div class="text-xs text-sub mt-1">
            {{ source.description }}
          </div>
        </div>

        <!-- Timestamps -->
        <div
          v-if="user.time_stamp"
          class="p-3 bg-sbg/20 rounded border border-secondary/30"
        >
          <div class="flex items-center gap-2 mb-2">
            <span class="font-medium text-text text-sm">Last Updated</span>
            <DataSourceIndicator
              source="local"
              :last-updated="user.time_stamp"
              compact
            />
          </div>
          <div class="text-sm text-text">
            {{ new Date(user.time_stamp * 1000).toLocaleString() }}
          </div>
          <div class="text-xs text-sub mt-1">
            When this local data was last collected
          </div>
        </div>
      </div>
    </ExpandableSection>

    <!-- Profile Details -->
    <ExpandableSection
      v-if="profileDetails.length > 0"
      title="Profile Details"
      icon="👤"
      :count="profileDetails.length"
      :default-expanded="false"
    >
      <div class="space-y-3">
        <div
          v-for="detail in profileDetails"
          :key="detail.label"
          class="p-3 bg-sbg/20 rounded border border-secondary/30"
        >
          <div class="flex items-center gap-2 mb-2">
            <span class="font-medium text-text text-sm">{{
              detail.label
            }}</span>
            <DataSourceIndicator :source="detail.source" compact />
          </div>
          <div class="text-sm text-text break-words">
            {{ detail.value }}
          </div>
          <div class="text-xs text-sub mt-1">
            {{ detail.description }}
          </div>
        </div>
      </div>
    </ExpandableSection>
  </div>
</template>
