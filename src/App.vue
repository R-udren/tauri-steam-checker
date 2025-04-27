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
const steam_ids = ref<Array<string>>([]);

async function getSteamUsers() {
  try {
    const users = await invoke("get_steam_users_list");
    steamUsers.value = users as Array<SteamUser>;
    steam_ids.value = users.map((user: SteamUser) => user.steam_id);
  } catch (error) {
    console.error("Error fetching Steam users:", error);
  }
}

onMounted(() => {
  getSteamUsers();
});
</script>

<template>
  <div class="min-h-screen bg-gradient-to-br from-[#1b2838] to-[#171a21] p-4">
    <div class="header flex flex-col items-center justify-center mb-8">
      <h1 class="text-5xl font-bold text-[#c7d5e0] drop-shadow-lg">
        Steam Checker
      </h1>
    </div>

    <div class="container mx-auto mt-4 p-4 rounded-xl shadow-2xl">
      <h2
        class="text-3xl text-[#66c0f4] mb-6 font-semibold border-b-2 border-[#1b2838] pb-2"
      >
        Total Steam Users: {{ steamUsers.length }}
      </h2>
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        <div
          v-for="user in steamUsers"
          :key="user.steam_id"
          class="user-card p-5 rounded-lg shadow-lg hover:shadow-xl hover:scale-[1.02] transition-all duration-300 ease-in-out border border-transparent hover:border-[#66c0f4]"
        >
          <h3 class="text-xl text-white font-semibold mb-2 truncate">
            {{ user.nickname || user.steam_id }}
          </h3>
          <div class="space-y-1 text-sm">
            <p class="text-gray-200 text-bold flex items-center">
              <span class="font-medium text-gray-300 w-28 inline-block"
                >Steam ID:</span
              >
              <span class="truncate">{{ user.steam_id }}</span>
            </p>
            <p class="text-gray-400 flex items-center">
              <span class="font-medium text-gray-300 w-28 inline-block"
                >Name History:</span
              >
              <span class="truncate">{{ user.name_history.join(", ") }}</span>
            </p>
            <p class="text-gray-400 flex items-center">
              <span class="font-medium text-gray-300 w-28 inline-block"
                >Most Recent:</span
              >
              <span
                :class="user.most_recent ? 'text-green-400' : 'text-red-400'"
                >{{ user.most_recent ? "Yes" : "No" }}</span
              >
            </p>
            <p class="text-gray-400 flex items-center">
              <span class="font-medium text-gray-300 w-28 inline-block"
                >Sources:</span
              >
              <span class="truncate">{{ user.sources.join(", ") }}</span>
            </p>
            <p class="text-gray-400 flex items-center">
              <span class="font-medium text-gray-300 w-28 inline-block"
                >Time Stamp:</span
              >
              {{
                user.time_stamp
                  ? new Date(user.time_stamp * 1000).toLocaleString()
                  : "N/A"
              }}
            </p>
          </div>
        </div>
      </div>
      <div class="mt-8">
        <h2
          class="text-3xl text-[#66c0f4] mb-6 font-semibold border-b-2 border-[#1b2838] pb-2"
        >
          Steam IDs
        </h2>
        <ul class="list-disc list-inside text-gray-300">
          {{
            steam_ids.join(" ")
          }}
        </ul>
      </div>
    </div>
  </div>
</template>
