<script setup lang="ts">
import { computed, onMounted, ref, watch } from "vue";
import ErrorMessage from "./components/ErrorMessage.vue";
import LoadingSpinner from "./components/LoadingSpinner.vue";
import SteamUserCard from "./components/SteamUserCard.vue";
import UserSearch from "./components/UserSearch.vue";
import { useSteamUsers } from "./composables/steamService";

const {
  steamUsers,
  fetchedProfiles,
  errorMessage: serviceError,
  isLoading,
  getSteamUsers,
} = useSteamUsers();

const errorMessage = ref(serviceError.value);

watch(serviceError, (newError) => {
  errorMessage.value = newError;
});

const clearErrorMessage = () => {
  errorMessage.value = null;
};

const getProfileForUser = (steamId: string) => {
  return fetchedProfiles.value.find((profile) => profile.steamID64 === steamId);
};

// Convert profiles array to Map for efficient searching
const profilesMap = computed(() => {
  const map = new Map();
  fetchedProfiles.value.forEach((profile) => {
    map.set(profile.steamID64, profile);
  });
  return map;
});

onMounted(() => {
  getSteamUsers();
});
</script>

<template>
  <div class="mx-auto px-4 py-6 sm:px-6 lg:px-8 min-h-screen bg-bg">
    <header class="mb-8 text-center">
      <h1
        class="p-2 text-6xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-primary to-accent"
      >
        Steam Accounts Checker
      </h1>
    </header>

    <ErrorMessage :message="errorMessage" @clear="clearErrorMessage" />

    <LoadingSpinner v-if="isLoading" message="Loading Steam users..." />

    <div
      v-else
      class="flex flex-col max-w-6xl justify-center align-center mx-auto rounded-md p-8"
    >
      <UserSearch :users="steamUsers" :profiles="profilesMap">
        <template #default="{ filteredUsers }">
          <div class="flex flex-col gap-4 justify-center align-center mx-auto">
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
  </div>
</template>
