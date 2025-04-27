<script setup lang="ts">
import type { FetchedProfile, SteamUser } from "../types.ts";

defineProps<{
  user: SteamUser;
  profile?: FetchedProfile;
}>();
</script>

<template>
  <div
    class="flex border border-gray-200 rounded-lg p-4 mb-4 bg-gray-50 shadow-sm hover:shadow-xl transition-shadow"
  >
    <div v-if="profile?.avatarFull" class="mr-4">
      <img
        :src="profile.avatarFull"
        alt="Steam Avatar"
        class="w-24 h-24 rounded-md"
      />
    </div>

    <div class="flex-1">
      <h3 class="text-2xl font-semibold mb-1">
        {{ user.nickname || profile?.steamID || user.steam_id }}
      </h3>

      <div v-if="profile" class="flex items-center mb-1">
        <span
          class="inline-block w-2 h-2 rounded-full mr-1.5"
          :class="{
            'bg-green-500': profile.onlineState?.toLowerCase() === 'online',
            'bg-blue-500': profile.onlineState?.toLowerCase() === 'in-game',
            'bg-yellow-500': profile.onlineState?.toLowerCase() === 'away', // Cannot reproduce this state...
            'bg-gray-500': profile.onlineState?.toLowerCase() === 'offline',
          }"
        ></span>
        <!-- prettier-ignore -->
        <span v-if="profile.stateMessage" class="ml-1">
          {{ profile.stateMessage.replace("<br/>", " ") }}
        </span>
      </div>

      <div class="text-sm mb-1">
        <span class="font-medium text-gray-600">Steam ID: </span>
        {{ user.steam_id }}
      </div>

      <div v-if="profile?.memberSince" class="text-sm mb-1">
        <span class="font-medium text-gray-600">Member since: </span>
        {{ profile.memberSince }}
      </div>

      <div v-if="user.name_history?.length" class="text-sm mb-1">
        <span class="font-medium text-gray-600">Name history: </span>
        <span class="text-gray-700">{{ user.name_history.join(", ") }}</span>
      </div>

      <div v-if="profile?.vacBanned === '1'" class="text-sm mb-1">
        <span class="font-medium text-gray-600">VAC Status: </span>
        <span class="text-red-600">Banned</span>
      </div>

      <div v-if="profile?.mostPlayedGames?.mostPlayedGame?.length" class="mt-2">
        <span class="font-medium text-gray-600 text-sm">Most played: </span>
        <div
          class="text-sm text-gray-700"
          v-for="(game, index) in profile.mostPlayedGames.mostPlayedGame"
          :key="index"
        >
          <img :src="game.gameLogo" class="w-24 h-8 inline-block mr-1" />
          {{ game.hoursOnRecord }} hours
        </div>
      </div>
    </div>
  </div>
</template>
