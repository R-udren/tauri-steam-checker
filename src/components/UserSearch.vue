<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { computed, ref } from "vue";
import type { FetchedProfile, SteamUser } from "../types.ts";
import CopyButton from "./CopyButton.vue";

const props = defineProps<{
  users: SteamUser[];
  profiles?: Map<string, FetchedProfile>;
}>();

const searchQuery = ref("");
const filteredUsers = computed(() => {
  if (!searchQuery.value.trim()) return props.users;

  const query = searchQuery.value.toLowerCase().trim();

  return props.users.filter((user) => {
    // Check local user data
    // Check nickname (if exists)
    if (user.nickname?.toLowerCase().includes(query)) return true;

    // Check steam_id
    if (user.steam_id.toLowerCase().includes(query)) return true;

    // Check name history
    if (user.name_history?.some((name) => name.toLowerCase().includes(query))) {
      return true;
    }

    // Check fetched profile data (if available)
    if (props.profiles) {
      const profile = props.profiles.get(user.steam_id);
      if (profile) {
        // Check profile nickname/steamID
        if (profile.steamID?.toLowerCase().includes(query)) return true;

        // Check real name
        if (profile.realname?.toLowerCase().includes(query)) return true;

        // Check custom URL
        if (profile.customURL?.toLowerCase().includes(query)) return true;

        // Check profile summary
        if (profile.summary?.toLowerCase().includes(query)) return true;
      }
    }

    return false;
  });
});

// All Steam IDs for copying
const allSteamIDs = computed(() => {
  return props.users.map((user) => user.steam_id).join(" ");
});

// Helper to get plain string for CopyButton
function getAllSteamIDs() {
  return allSteamIDs.value;
}

// Emit the filtered users to parent
defineEmits<{
  (e: "update:filtered", users: SteamUser[]): void;
}>();

const errorMessage = ref<string | null>(null);
const isRegistering = ref(false);

async function registerUsers() {
  errorMessage.value = null;
  isRegistering.value = true;
  try {
    await invoke("register_users");
    // Optionally, you can emit an event or show a success message
  } catch (err: any) {
    errorMessage.value = err?.message || String(err) || "Unknown error";
  } finally {
    isRegistering.value = false;
  }
}
</script>

<template>
  <div class="mb-4">
    <!-- Search Input with Copy Button -->
    <div class="flex gap-2">
      <input
        v-model="searchQuery"
        type="text"
        placeholder="Search by nickname, Steam ID, name history, real name, location, or status..."
        class="text-text flex-1 px-4 py-3 rounded-md border-2 border-primary focus:outline-none focus:ring-2 focus:ring-primary transition duration-200 hover:bg-sbg/20 hover:border-primary-hover"
      />

      <CopyButton
        :value="getAllSteamIDs()"
        label="Copy All IDs"
        size="md"
        class="px-4 py-3 bg-primary hover:bg-primary-hover text-white rounded-md"
      />
      <!-- <button
        @click="registerUsers"
        :disabled="isRegistering"
        class="px-4 py-3 bg-secondary hover:bg-secondary/20 text-white rounded-md disabled:opacity-50"
      >
        <span v-if="isRegistering">Registering...</span>
        <span v-else>Register Users</span>
      </button> -->
    </div>

    <!-- Steam IDs Display -->
    <div class="mt-2 mb-4 flex justify-center align-center">
      <code class="text-text text-xl">{{
        filteredUsers.map((user) => user.steam_id).join(" ")
      }}</code>
    </div>

    <div v-if="errorMessage" class="text-red-600 mt-2">{{ errorMessage }}</div>

    <slot :filtered-users="filteredUsers"></slot>
  </div>
</template>
