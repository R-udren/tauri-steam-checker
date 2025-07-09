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

const activeTab = ref<"gaming" | "technical" | "profile">("gaming");

const tabs = [
  { id: "gaming", label: "Gaming", icon: "🎮" },
  { id: "technical", label: "Technical", icon: "⚙️" },
  { id: "profile", label: "Profile", icon: "👤" },
] as const;
</script>

<template>
  <div class="space-y-3">
    <!-- Expandable Details Section -->
    <details class="group">
      <summary
        class="flex items-center justify-between p-3 bg-bg-tertiary/50 border border-border rounded-lg cursor-pointer hover:bg-bg-tertiary transition-colors"
      >
        <div class="flex items-center gap-2">
          <span class="group-open:rotate-90 transition-transform text-text-muted">▶</span>
          <span class="text-sm font-medium text-text-secondary">Detailed Information</span>
        </div>
        <div class="flex items-center gap-1 text-xs text-text-muted">
          <span v-for="tab in tabs" :key="tab.id" class="flex items-center gap-1">
            {{ tab.icon }}
          </span>
        </div>
      </summary>

      <div class="mt-3 space-y-3">
        <!-- Tab Navigation -->
        <div class="flex border-b border-border">
          <button
            v-for="tab in tabs"
            :key="tab.id"
            @click="activeTab = tab.id"
            :class="[
              'flex items-center gap-2 px-4 py-2 text-sm font-medium transition-colors relative',
              activeTab === tab.id
                ? 'text-primary border-b-2 border-primary bg-bg-tertiary/50'
                : 'text-text-muted hover:text-text-secondary hover:bg-bg-tertiary/30',
            ]"
          >
            <span>{{ tab.icon }}</span>
            <span>{{ tab.label }}</span>
          </button>
        </div>

        <!-- Tab Content -->
        <div class="min-h-[200px]">
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
    </details>
  </div>
</template>
