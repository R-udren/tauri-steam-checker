<script setup lang="ts">
import { computed } from "vue";
import type { FetchedProfile, SteamUser } from "../types";
import DetailsModal from "./DetailsModal.vue";
import StatusBar from "./StatusBar.vue";
import UserIdentity from "./UserIdentity.vue";

interface Props {
  user: SteamUser;
  profile?: FetchedProfile;
}

const props = defineProps<Props>();

// Calculate total playtime in hours
const totalHours = computed(() => {
  const totalMinutes = props.user.apps.reduce(
    (sum, app) => sum + (app.playtime_minutes ?? 0),
    0
  );
  return Math.round((totalMinutes / 60) * 10) / 10; // Round to 1 decimal
});

// Get most recent game
const mostRecentGame = computed(() => {
  if (!props.user.apps.length) return null;

  return props.user.apps.reduce(
    (recent, app) =>
      (app.last_played ?? 0) > (recent.last_played ?? 0) ? app : recent,
    props.user.apps[0]
  );
});
</script>

<template>
  <div
    :class="[
      'rounded-lg p-4 space-y-3',
      user.sources.length < 3
        ? 'bg-warning/10 border-warning'
        : 'bg-bg-secondary border-border',
      'border',
    ]"
  >
    <!-- Core Identity Section -->
    <UserIdentity :user="user" :profile="profile" />

    <!-- Quick Status & Stats -->
    <StatusBar
      :user="user"
      :profile="profile"
      :total-hours="totalHours"
      :most-recent-game="mostRecentGame"
    />

    <!-- Details Modal -->
    <DetailsModal :user="user" :profile="profile" />
  </div>
</template>
