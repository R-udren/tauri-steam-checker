<script setup lang="ts">
import { onMounted } from "vue";
import ErrorMessage from "./components/ErrorMessage.vue";
import SteamUserCard from "./components/SteamUserCard.vue";
import UserSearch from "./components/UserSearch.vue";
import { useSteamUsers } from "./composables/steamService";

const { steamUsers, fetchedProfiles, errorMessage, isLoading, getSteamUsers } =
  useSteamUsers();

// Get profile for a specific user
const getProfileForUser = (steamId: string) => {
  return fetchedProfiles.value.find((profile) => profile.steamID64 === steamId);
};

onMounted(() => {
  getSteamUsers();
});
</script>

<template>
  <div class="max-w-7xl mx-auto px-4 py-6 sm:px-6 lg:px-8">
    <header class="mb-8 text-center">
      <h1 class="text-3xl font-bold text-gray-800">
        Blazing Fast Steam Account Finder
      </h1>
    </header>

    <ErrorMessage :message="errorMessage" />

    <div
      v-if="isLoading"
      class="text-center py-10 italic text-gray-500 text-4xl"
    >
      Loading Steam users...
    </div>

    <UserSearch :users="steamUsers">
      <template #default="{ filteredUsers }">
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          <SteamUserCard
            v-for="user in filteredUsers"
            :key="user.steam_id"
            :user="user"
            :profile="getProfileForUser(user.steam_id)"
          />
        </div>
      </template>
    </UserSearch>
  </div>
</template>
