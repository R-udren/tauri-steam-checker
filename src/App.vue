<script setup lang="ts">
import { ref } from "vue";
import { onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

interface SteamUser {
  steam_id: string;
  nickname?: string;
  name_history: Array<string>;
  most_recent: boolean;
  sources: Array<string>;
  time_stamp?: number;
  app_list: Array<App>;
}

interface App {
  app_id: number;
  playtime_minutes: number;
  last_played: number;
}

const steamUsers = ref<Array<SteamUser>>([]);

async function getSteamUsers() {
  try {
    const users = await invoke("get_steam_users_list");
    steamUsers.value = users as Array<SteamUser>;
  } catch (error) {
    console.error("Error fetching Steam users:", error);
  }
}

onMounted(() => {
  getSteamUsers();
});
</script>

<template>
  <div class="h-screen bg-zinc-800">
    <div class="header flex flex-col items-center justify-center">
      <h1 class="text-4xl text-white">Steam Checker</h1>
    </div>

    <div class="container mx-auto mt-4 p-4 bg-zinc-700 rounded-lg shadow-lg">
      <h2 class="text-2xl text-white mb-4">Steam Users</h2>
      <div
        v-for="user in steamUsers"
        :key="user.steam_id"
        class="user-card bg-zinc-600 p-4 mb-4 rounded-lg shadow-md"
      >
        <h3 class="text-xl text-white">{{ user.nickname || user.steam_id }}</h3>
        <p class="text-gray-300">
          Name History: {{ user.name_history.join(", ") }}
        </p>
        <p class="text-gray-300">
          Most Recent: {{ user.most_recent ? "Yes" : "No" }}
        </p>
        <p class="text-gray-300">Sources: {{ user.sources.join(", ") }}</p>
        <p class="text-gray-300">
          Time Stamp:
          {{
            user.time_stamp
              ? new Date(user.time_stamp * 1000).toLocaleString()
              : "N/A"
          }}
        </p>
      </div>
    </div>
  </div>
</template>
