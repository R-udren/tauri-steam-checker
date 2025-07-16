<script setup lang="ts">
import { ref } from "vue";
import type { FetchedProfile, SteamUser } from "../types";
import GamingTab from "./GamingTab.vue";
import ProfileTab from "./ProfileTab.vue";
import TechnicalTab from "./TechnicalTab.vue";

interface Props {
  user: SteamUser;
  profile?: FetchedProfile;
}

defineProps<Props>();

const isOpen = ref(false);
const activeTab = ref<"gaming" | "technical" | "profile">("gaming");

const tabs = [
  { id: "gaming", label: "Gaming", icon: "🎮" },
  { id: "technical", label: "Technical", icon: "⚙️" },
  { id: "profile", label: "Profile", icon: "👤" },
] as const;

function openModal() {
  isOpen.value = true;
}

function closeModal() {
  isOpen.value = false;
}
</script>

<template>
  <div>
    <!-- Trigger Button -->
    <button
      @click="openModal"
      class="w-full p-3 bg-gradient-to-r from-primary/10 to-bg-tertiary/70 border border-primary/40 rounded-xl shadow-md hover:shadow-lg transition-all duration-200 text-left group hover:bg-bg-secondary hover:border-primary hover:text-primary"
    >
      <div class="flex items-center justify-between">
        <span
          class="text-base font-semibold text-primary flex items-center gap-2"
        >
          <svg
            class="w-5 h-5 text-primary animate-pulse"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <circle cx="12" cy="12" r="10" stroke-width="2" />
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M8 12h8M12 8v8"
            />
          </svg>
          View Details
        </span>
        <div class="flex items-center gap-2 text-lg">
          <span
            v-for="tab in tabs"
            :key="tab.id"
            class="transition-transform duration-200 group-hover:scale-125"
            :title="tab.label"
          >
            {{ tab.icon }}
          </span>
        </div>
      </div>
    </button>

    <!-- Modal Overlay -->
    <div
      v-if="isOpen"
      class="fixed inset-0 bg-black/80 flex items-center justify-center z-50 p-4"
      @click="closeModal"
    >
      <!-- Modal Content -->
      <div
        class="bg-bg-secondary border border-border rounded-lg w-full max-w-4xl max-h-[90vh] overflow-hidden flex flex-col"
        @click.stop
      >
        <!-- Modal Header -->
        <div
          class="flex items-center justify-between p-4 border-b border-border"
        >
          <h2 class="text-lg font-semibold text-text">
            {{ profile?.steamID || user.nickname || user.steam_id }} - Details
          </h2>
          <button
            @click="closeModal"
            class="text-text-muted hover:text-text transition-colors p-1"
          >
            <svg
              class="w-6 h-6"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M6 18L18 6M6 6l12 12"
              ></path>
            </svg>
          </button>
        </div>

        <!-- Tab Navigation -->
        <div class="flex border-b border-border bg-bg-tertiary/30">
          <button
            v-for="tab in tabs"
            :key="tab.id"
            @click="activeTab = tab.id"
            :class="[
              'flex items-center gap-2 px-6 py-3 text-sm font-medium transition-colors relative',
              activeTab === tab.id
                ? 'text-primary border-b-2 border-primary bg-bg-secondary'
                : 'text-text-muted hover:text-text-secondary hover:bg-bg-tertiary/50',
            ]"
          >
            <span>{{ tab.icon }}</span>
            <span>{{ tab.label }}</span>
          </button>
        </div>

        <!-- Tab Content -->
        <div class="flex-1 overflow-y-auto p-6">
          <GamingTab
            v-if="activeTab === 'gaming'"
            :user="user"
            :profile="profile"
          />

          <TechnicalTab
            v-if="activeTab === 'technical'"
            :user="user"
            :profile="profile"
          />

          <ProfileTab
            v-if="activeTab === 'profile'"
            :user="user"
            :profile="profile"
          />
        </div>
      </div>
    </div>
  </div>
</template>
