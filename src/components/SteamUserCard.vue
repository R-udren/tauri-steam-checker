<script setup lang="ts">
import type { FetchedProfile, SteamUser } from "../types.ts";
import GamingDashboard from "./GamingDashboard.vue";
import StatusPanel from "./StatusPanel.vue";
import TechnicalDetails from "./TechnicalDetails.vue";
import UserHeader from "./UserHeader.vue";

defineProps<{
  user: SteamUser;
  profile?: FetchedProfile;
}>();
</script>

<template>
  <div
    class="border border-secondary rounded-lg bg-sbg shadow-sm hover:shadow-xl transition-all hover:-translate-y-0.5 duration-300 overflow-hidden"
  >
    <!-- User Header Section -->
    <UserHeader :user="user" :profile="profile" />

    <!-- Status Panel Section -->
    <StatusPanel :user="user" :profile="profile" />

    <!-- Gaming Dashboard Section -->
    <GamingDashboard
      :user="user"
      :profile="profile"
      v-if="user.app_list && user.app_list.length > 0"
    />

    <!-- Technical Details Section -->
    <div class="p-4">
      <TechnicalDetails :user="user" :profile="profile" />
    </div>
  </div>
</template>
