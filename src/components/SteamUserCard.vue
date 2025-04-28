<script setup lang="ts">
import type { FetchedProfile, SteamUser } from "../types.ts";

defineProps<{
  user: SteamUser;
  profile?: FetchedProfile;
}>();
</script>

<template>
  <div
    class="flex border border-secondary rounded-lg p-4 mb-4 bg-sbg shadow-sm hover:shadow-xl transition-all hover:-translate-y-1 duration-300"
  >
    <div v-if="profile?.avatarFull" class="mr-4 flex-shrink-0">
      <img
        :src="profile.avatarFull"
        alt="Steam Avatar"
        class="w-32 h-32 rounded-lg object-cover shadow-md transition-all duration-300"
        :class="{
          'border-4 border-green-500 shadow-lg':
            profile.onlineState?.toLowerCase() === 'online',
          'border-4 border-blue-500 shadow-lg':
            profile.onlineState?.toLowerCase() === 'in-game',
          'border-4 border-yellow-500 shadow-lg':
            profile.onlineState?.toLowerCase() === 'away',
          'border-4 border-gray-500':
            profile.onlineState?.toLowerCase() === 'offline',
        }"
      />
    </div>

    <div class="flex-1">
      <h3 class="text-2xl font-semibold mb-1 text-text">
        {{ user.nickname || profile?.steamID || user.steam_id }}
        <span class="text-sub text-md"
          >({{ user.name_history.join(", ") }})</span
        >
      </h3>

      <div class="text-md mb-1 mt-1">
        <span class="text-text rounded-md font-medium max-w-fit text-md"
          >{{ user.steam_id }}
        </span>
      </div>

      <div v-if="profile?.vacBanned === '0'" class="text-sm mb-1">
        <span
          class="bg-[#f43f5e26] rounded-md p-1 text-[#f43f5e] text-bold text-sm"
          >VAC Banned</span
        >
      </div>

      <div v-if="profile?.memberSince" class="text-sm mb-1 text-text">
        <span class="font-medium text-sub">Member since: </span>
        {{ profile.memberSince }}
      </div>
    </div>
  </div>
</template>
