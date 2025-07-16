<script setup lang="ts">
import DOMPurify from "dompurify";
import { computed } from "vue";
import type { FetchedProfile, SteamUser } from "../types";
import InfoRow from "./InfoRow.vue";

interface Props {
  user: SteamUser;
  profile?: FetchedProfile;
}

const props = defineProps<Props>();

// --- Computed Properties for Data Transformation ---

const realName = computed(() => props.profile?.realname);
const location = computed(() => props.profile?.location);
const memberSince = computed(() =>
  props.profile?.memberSince
    ? new Date(props.profile.memberSince).toDateString()
    : undefined
);

const onlineState = computed(() => {
  const state = props.profile?.onlineState;
  if (!state) return { text: "Unknown", variant: "default" as const };
  const lowerState = state.toLowerCase();
  if (lowerState === "online" || lowerState === "in-game")
    return { text: state, variant: "success" as const };
  if (lowerState === "away" || lowerState === "snooze")
    return { text: state, variant: "warning" as const };
  return { text: state, variant: "default" as const };
});

const statusMessage = computed(() => props.profile?.stateMessage);

const sanitizedSummary = computed(() => {
  if (!props.profile?.summary) return undefined;
  // Sanitize the HTML to prevent XSS
  return DOMPurify.sanitize(props.profile.summary);
});

const nameHistory = computed(() => {
  if (!props.user.name_history?.length) return [];
  return props.user.name_history.map((name, index) => ({
    name,
    position: index + 1,
    isCurrent: index === 0,
  }));
});

const hasProfileData = computed(() => {
  return (
    realName.value ||
    location.value ||
    memberSince.value ||
    sanitizedSummary.value ||
    onlineState.value.text !== "Unknown" ||
    statusMessage.value ||
    nameHistory.value.length > 0
  );
});
</script>

<template>
  <div v-if="hasProfileData" class="space-y-6">
    <!-- Section: Profile Information -->
    <section>
      <h3 class="text-lg font-semibold text-text mb-3 flex items-center gap-2">
        <span class="text-xl">👤</span> Profile Information
      </h3>
      <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
        <InfoRow v-if="realName" label="Real Name" :value="realName" />
        <InfoRow v-if="location" label="Location" :value="location" />
        <InfoRow v-if="memberSince" label="Member Since" :value="memberSince" />
      </div>
    </section>

    <!-- Section: Profile Summary -->
    <section v-if="sanitizedSummary">
      <h3 class="text-lg font-semibold text-text mb-3 flex items-center gap-2">
        <span class="text-xl">📝</span> Profile Summary
      </h3>
      <div
        class="p-4 bg-bg-tertiary rounded-lg border border-border prose prose-sm max-w-none"
      >
        <div class="text-text leading-relaxed" v-html="sanitizedSummary"></div>
      </div>
    </section>

    <!-- Section: Current Status -->
    <section>
      <h3 class="text-lg font-semibold text-text mb-3 flex items-center gap-2">
        <span class="text-xl">💬</span> Current Status
      </h3>
      <div class="space-y-2">
        <InfoRow
          label="Online State"
          :value="onlineState.text"
          :variant="onlineState.variant"
        />
        <InfoRow
          v-if="statusMessage"
          label="Status Message"
          :value="statusMessage"
        />
      </div>
    </section>

    <!-- Section: Name History -->
    <section v-if="nameHistory.length > 0">
      <h3 class="text-lg font-semibold text-text mb-3 flex items-center gap-2">
        <span class="text-xl">📜</span> Name History
        <span class="text-sm text-text-muted"
          >({{ nameHistory.length }} names)</span
        >
      </h3>
      <div class="space-y-2 max-h-72 overflow-y-auto">
        <div
          v-for="entry in nameHistory"
          :key="entry.position"
          class="flex items-center justify-between p-4 rounded-lg border bg-bg-tertiary border-border"
        >
          <div class="flex items-center gap-3">
            <div
              class="text-xs text-text-muted font-mono w-8 text-center flex-shrink-0"
            >
              #{{ entry.position }}
            </div>
            <div class="text-sm text-text font-medium">{{ entry.name }}</div>
          </div>
        </div>
      </div>
    </section>
  </div>

  <!-- Empty State -->
  <div v-else class="text-center py-12 text-text-muted">
    <p class="text-4xl mb-3">👤</p>
    <p>No profile data available for this user.</p>
  </div>
</template>
