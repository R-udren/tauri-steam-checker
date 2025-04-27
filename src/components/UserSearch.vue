<script setup lang="ts">
import { computed, ref } from "vue";
import type { SteamUser } from "../types.ts";

const props = defineProps<{
  users: SteamUser[];
}>();

const searchQuery = ref("");
const filteredUsers = computed(() => {
  if (!searchQuery.value.trim()) return props.users;

  const query = searchQuery.value.toLowerCase().trim();

  return props.users.filter((user) => {
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

// Emit the filtered users to parent
defineEmits<{
  (e: "update:filtered", users: SteamUser[]): void;
}>();
</script>

<template>
  <div class="mb-4">
    <input
      v-model="searchQuery"
      type="text"
      placeholder="Search by nickname, Steam ID or name history"
      class="w-full px-4 py-3 rounded-md border border-gray-300 focus:outline-none focus:ring-2 focus:ring-blue-500 transition duration-200"
    />

    <div class="text-sm text-gray-500 mt-2 mb-4 flex justify-center">
      {{ filteredUsers.map((user) => user.steam_id).join(" ") }}
    </div>

    <slot :filtered-users="filteredUsers"></slot>
  </div>
</template>
