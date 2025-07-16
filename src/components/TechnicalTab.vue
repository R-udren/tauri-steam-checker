<script setup lang="ts">
import { computed } from "vue";
import type { FetchedProfile, SteamUser } from "../types";
import CopyButton from "./CopyButton.vue";
import InfoRow from "./InfoRow.vue";

interface Props {
  user: SteamUser;
  profile?: FetchedProfile;
}

const props = defineProps<Props>();

// --- Computed Properties for Data Transformation ---

const steamId64 = computed(() => props.user.steam_id);
const customUrl = computed(() => props.profile?.customURL);

const localSources = computed(() => props.user.sources?.join(", ") || "N/A");
const lastUpdated = computed(() =>
  props.user.time_stamp
    ? new Date(props.user.time_stamp * 1000).toLocaleString("en-GB")
    : "N/A"
);
const isActive = computed(() => (props.user.most_recent ? "Yes" : "No"));

const privacyState = computed(() => {
  const state = props.profile?.privacyState;
  if (!state) return { text: "Unknown", variant: "default" as const };
  if (state.toLowerCase() === "public")
    return { text: "Public", variant: "success" as const };
  if (state.toLowerCase() === "friendsonly")
    return { text: "Friends Only", variant: "info" as const };
  if (state.toLowerCase() === "private")
    return { text: "Private", variant: "warning" as const };
  return { text: state, variant: "default" as const };
});

const visibilityState = computed(() => {
  const state = props.profile?.VisibilityState;
  if (!state) return { text: "Unknown", variant: "default" as const };
  const map: Record<
    string,
    { text: string; variant: "success" | "info" | "warning" }
  > = {
    "3": { text: "Public", variant: "success" },
    "2": { text: "Friends Only", variant: "info" },
    "1": { text: "Private", variant: "warning" },
  };
  return map[state] || { text: "Unknown", variant: "default" as const };
});

const vacStatus = computed(() => {
  const isBanned = props.profile?.vacBanned === "1";
  return {
    text: isBanned ? "BANNED" : "Not Banned",
    variant: isBanned ? ("warning" as const) : ("success" as const),
  };
});

const tradeBanStatus = computed(() => {
  const isBanned = props.profile?.tradeBanState === "Banned";
  return {
    text: isBanned ? "BANNED" : "None",
    variant: isBanned ? ("error" as const) : ("success" as const),
  };
});

const isLimited = computed(() => {
  const limited = props.profile?.isLimitedAccount === "1";
  return {
    text: limited ? "Yes" : "No",
    variant: limited ? ("warning" as const) : ("success" as const),
  };
});
</script>

<template>
  <div class="space-y-6">
    <!-- Section: Steam Identifiers -->
    <section>
      <h3 class="text-lg font-semibold text-text mb-3 flex items-center gap-2">
        <span class="text-xl">🆔</span> Steam Identifiers
      </h3>
      <div class="space-y-2">
        <InfoRow
          v-if="steamId64"
          label="SteamID64"
          :value="steamId64"
          description="The user's unique 64-bit Steam identifier."
        >
          <template #actions>
            <CopyButton :value="steamId64" />
          </template>
        </InfoRow>
        <InfoRow
          v-if="customUrl"
          label="Custom URL"
          :value="customUrl"
          description="The user's personalized Steam community URL."
        >
          <template #actions>
            <CopyButton :value="customUrl" />
          </template>
        </InfoRow>
      </div>
    </section>

    <!-- Section: Data Provenance -->
    <section>
      <h3 class="text-lg font-semibold text-text mb-3 flex items-center gap-2">
        <span class="text-xl">📊</span> Data Provenance
      </h3>
      <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
        <InfoRow
          label="Local Data Sources"
          :value="localSources"
          description="Where the local data for this user was found."
        />
        <InfoRow
          label="Last Updated (Local)"
          :value="lastUpdated"
          description="The last time the local data was updated."
        />
        <InfoRow
          label="Is Active Profile"
          :value="isActive"
          :variant="user.most_recent ? 'success' : 'default'"
          description="Whether this is the most recently used profile for this SteamID."
        />
      </div>
    </section>

    <!-- Section: Privacy & Security -->
    <section>
      <h3 class="text-lg font-semibold text-text mb-3 flex items-center gap-2">
        <span class="text-xl">🔒</span> Privacy & Security
      </h3>
      <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
        <InfoRow
          label="Profile Privacy"
          :value="privacyState.text"
          :variant="privacyState.variant"
          description="Who can view the user's profile."
        />
        <InfoRow
          label="Game Details Visibility"
          :value="visibilityState.text"
          :variant="visibilityState.variant"
          description="Who can see the user's game library and playtime."
        />
        <InfoRow
          label="VAC Status"
          :value="vacStatus.text"
          :variant="vacStatus.variant"
          description="Valve Anti-Cheat system status."
        />
        <InfoRow
          label="Trade Ban Status"
          :value="tradeBanStatus.text"
          :variant="tradeBanStatus.variant"
          description="Whether the user is banned from trading."
        />
        <InfoRow
          label="Limited Account"
          :value="isLimited.text"
          :variant="isLimited.variant"
          description="Whether the account has spending restrictions."
        />
      </div>
    </section>

    <!-- Empty State -->
    <div v-if="!user && !profile" class="text-center py-12 text-text-muted">
      <p class="text-4xl mb-3">⚙️</p>
      <p>No technical data available for this user.</p>
    </div>
  </div>
</template>
