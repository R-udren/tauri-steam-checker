<script setup lang="ts">
import { ref, onMounted, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";

interface SteamUser {
  steam_id: string;
  nickname?: string;
  name_history: Array<string>;
  most_recent: boolean;
  sources: Array<string>; // Should be hidden in the UI, only count
  time_stamp?: number; // UNIX timestamp of the last update
  app_list: Array<App>;
}

interface App {
  app_id: number;
  playtime_minutes: number;
  last_played: number;
}

interface MostPlayedGame {
  gameName?: string;
  gameLink?: string;
  gameIcon?: string;
  gameLogo?: string;
  hoursOnRecord?: string;
}

interface MostPlayedGames {
  mostPlayedGame: MostPlayedGame[];
}

interface FetchedProfile {
  steamID64: string;
  steamID?: string; // Nickname
  onlineState?: string;
  stateMessage?: string;
  visibilityState?: string;
  privacyState?: string;
  avatarFull?: string;
  memberSince?: string;
  mostPlayedGames?: MostPlayedGames;
  vacBanned?: string;
  tradeBanState?: string;
  isLimitedAccount?: string;
  customURL?: string;
  location?: string;
  realname?: string;
  summary?: string;
}

const steamUsers = ref<Array<SteamUser>>([]);
const fetchedProfiles = ref<Array<FetchedProfile>>([]);
const steam_ids = ref<Array<string>>([]); // List of all steam_ids should be displayed in the UI
const errorMessage = ref<string | null>(null);

// Search functionality
const searchQuery = ref("");
const filteredUsers = computed(() => {
  if (!searchQuery.value.trim()) return steamUsers.value;

  const query = searchQuery.value.toLowerCase().trim();

  return steamUsers.value.filter((user) => {
    // Check nickname (if exists)
    if (user.nickname?.toLowerCase().includes(query)) return true;

    // Check steam_id
    if (user.steam_id.toLowerCase().includes(query)) return true;

    // Check name history
    if (user.name_history?.some((name) => name.toLowerCase().includes(query))) {
      return true;
    }

    return false;
  });
});

async function getSteamUsers() {
  errorMessage.value = null; // Clear any previous error message
  try {
    const users = await invoke("get_steam_users_list");
    steamUsers.value = users as Array<SteamUser>;
    steam_ids.value = users.map((user: SteamUser) => user.steam_id);

    // Fetch profiles for each user
    for (const user of steamUsers.value) {
      try {
        await fetchSteamProfile(user.steam_id);
      } catch (error) {
        console.error("Error fetching Steam profile:", error);
        errorMessage.value = "Failed to fetch Steam profile. Details: " + error;
      }
    }
  } catch (error) {
    console.error("Error fetching Steam users:", error);
    errorMessage.value = "Failed to gather Steam users. Details: " + error;
  }
}

async function fetchSteamProfile(steamID: string) {
  try {
    const profile = await invoke("fetch_steam_profile", { steamID });
    const parsedProfile = JSON.parse(profile as string) as FetchedProfile;
    fetchedProfiles.value.push(parsedProfile);
  } catch (error) {
    console.error("Error fetching Steam profile:", error);
    errorMessage.value = "Failed to fetch Steam profile. Details: " + error;
  }
}

onMounted(() => {
  getSteamUsers();
});
</script>

<template></template>
