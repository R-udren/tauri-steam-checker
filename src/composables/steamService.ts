import { invoke } from "@tauri-apps/api/core";
import { ref } from "vue";
import type { FetchedProfile, SteamUser } from "../types.ts";

export function useSteamUsers() {
  const steamUsers = ref<Array<SteamUser>>([]);
  const fetchedProfiles = ref<Array<FetchedProfile>>([]);
  const steam_ids = ref<Array<string>>([]);
  const errorMessage = ref<string | null>(null);
  const isLoading = ref(false);

  async function getSteamUsers() {
    errorMessage.value = null;
    isLoading.value = true;

    try {
      const users = await invoke("get_steam_users_list");
      steamUsers.value = users as Array<SteamUser>;
      steam_ids.value = (users as Array<SteamUser>).map(
        (user: SteamUser) => user.steam_id
      );

      // Fetch profiles for each user in parallel and add as they arrive
      for (const user of steamUsers.value) {
        fetchSteamProfile(user.steam_id); // don't await to let them resolve independently
      }
    } catch (error) {
      console.error("Error fetching Steam users:", error);
      errorMessage.value = "Failed to gather Steam users. Details: " + error;
    } finally {
      isLoading.value = false;
    }
  }

  async function fetchSteamProfile(steamID: string) {
    try {
      const profile = await invoke("fetch_steam_profile", {
        steamId: steamID,
      });
      const parsedProfile = profile as FetchedProfile;
      fetchedProfiles.value.push(parsedProfile);
    } catch (error) {
      console.error("Error fetching Steam profile:", error);
      errorMessage.value = "Failed to fetch Steam profile. Details: " + error;
    }
  }

  return {
    steamUsers,
    fetchedProfiles,
    steam_ids,
    errorMessage,
    isLoading,
    getSteamUsers,
    fetchSteamProfile,
  };
}
