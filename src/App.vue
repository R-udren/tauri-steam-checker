<script setup lang="ts">
import { onMounted, ref, watch } from "vue";
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

onMounted(() => {
  getSteamUsers();
});
</script>

<template>
  <div class="max-w-7xl mx-auto px-4 py-6 sm:px-6 lg:px-8 min-h-screen">
    <header class="mb-8 text-center">
      <h1 class="text-3xl font-bold text-gray-800">
        Blazing Fast Steam Account Finder
      </h1>
    </header>

    <ErrorMessage :message="errorMessage" @clear="clearErrorMessage" />

    <LoadingSpinner v-if="isLoading" message="Loading Steam users..." />

    <div v-else>
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
  </div>
</template>
