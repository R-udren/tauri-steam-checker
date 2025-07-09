<script setup lang="ts">
import { computed, ref } from "vue";
import type { FetchedProfile, SteamUser } from "../types";
import ExpandableSection from "./ExpandableSection.vue";
import GamePreviewCard from "./GamePreviewCard.vue";

interface Props {
  user: SteamUser;
  profile?: FetchedProfile;
}

const props = defineProps<Props>();

const showAllGames = ref(false);

const gamingStats = computed(() => {
  console.log("Apps: ", props.user.app_list);
  const totalGames = props.user.app_list.length;
  const totalMinutes = props.user.app_list.reduce(
    (sum, app) => sum + app.playtime_minutes,
    0
  );
  const totalHours = Math.round((totalMinutes / 60) * 10) / 10;

  const playedGames = props.user.app_list.filter(
    (app) => app.playtime_minutes > 0
  );
  const recentGames = props.user.app_list
    .filter((app) => app.last_played > 0)
    .sort((a, b) => b.last_played - a.last_played);

  return {
    totalGames,
    totalHours,
    playedGames: playedGames.length,
    recentGames: recentGames.length,
  };
});

const topGamesByPlaytime = computed(() => {
  return [...props.user.app_list]
    .filter((app) => app.playtime_minutes > 0)
    .sort((a, b) => b.playtime_minutes - a.playtime_minutes)
    .slice(0, 8);
});

const recentGames = computed(() => {
  return [...props.user.app_list]
    .filter((app) => app.last_played > 0)
    .sort((a, b) => b.last_played - a.last_played)
    .slice(0, 5);
});

const mostPlayedFromProfile = computed(() => {
  if (!props.profile?.mostPlayedGames?.mostPlayedGame) return [];

  return props.profile.mostPlayedGames.mostPlayedGame.slice(0, 3);
});
</script>

<template>
  <div class="p-4 space-y-4">
    <!-- Gaming Statistics -->
    <div class="grid grid-cols-2 md:grid-cols-4 gap-4 mb-6">
      <div
        class="text-center p-3 bg-sbg/30 rounded-lg border border-secondary/30"
      >
        <div class="text-2xl font-bold text-text">
          {{ gamingStats.totalGames }}
        </div>
        <div class="text-xs text-sub">Total Games</div>
      </div>
      <div
        class="text-center p-3 bg-sbg/30 rounded-lg border border-secondary/30"
      >
        <div class="text-2xl font-bold text-text">
          {{ gamingStats.playedGames }}
        </div>
        <div class="text-xs text-sub">Played</div>
      </div>
      <div
        class="text-center p-3 bg-sbg/30 rounded-lg border border-secondary/30"
      >
        <div class="text-2xl font-bold text-text">
          {{ gamingStats.totalHours }}h
        </div>
        <div class="text-xs text-sub">Total Playtime</div>
      </div>
      <div
        class="text-center p-3 bg-sbg/30 rounded-lg border border-secondary/30"
      >
        <div class="text-2xl font-bold text-text">
          {{ gamingStats.recentGames }}
        </div>
        <div class="text-xs text-sub">Recently Played</div>
      </div>
    </div>

    <!-- Most Played Games from Profile -->
    <div v-if="mostPlayedFromProfile.length > 0" class="mb-6">
      <h3 class="text-lg font-semibold text-text mb-3 flex items-center gap-2">
        🏆 Most Played (Profile)
        <span class="text-xs text-sub bg-secondary/30 px-2 py-0.5 rounded-full"
          >Remote</span
        >
      </h3>
      <div class="grid grid-cols-1 md:grid-cols-3 gap-3">
        <div
          v-for="(game, index) in mostPlayedFromProfile"
          :key="index"
          class="p-3 bg-sbg/30 rounded-lg border border-secondary/30"
        >
          <div class="flex items-center gap-3">
            <img
              v-if="game.gameIcon"
              :src="game.gameIcon"
              :alt="game.gameName"
              class="w-12 h-12 rounded object-cover"
            />
            <div class="flex-1 min-w-0">
              <div class="font-medium text-sm text-text truncate">
                {{ game.gameName || "Unknown Game" }}
              </div>
              <div class="text-xs text-sub">
                {{ game.hoursOnRecord || "0" }} hours
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Top Games by Playtime (Local Data) -->
    <ExpandableSection
      title="Top Games by Playtime"
      icon="🎮"
      :count="topGamesByPlaytime.length"
      :default-expanded="true"
    >
      <div class="space-y-2">
        <div class="text-xs text-sub mb-3 flex items-center gap-2">
          💻 Local data (most accurate)
        </div>
        <div class="grid grid-cols-1 lg:grid-cols-2 gap-3">
          <GamePreviewCard
            v-for="app in topGamesByPlaytime.slice(
              0,
              showAllGames ? topGamesByPlaytime.length : 6
            )"
            :key="app.app_id"
            :app="app"
            :show-playtime="true"
            :show-last-played="true"
          />
        </div>
        <button
          v-if="topGamesByPlaytime.length > 6"
          @click="showAllGames = !showAllGames"
          class="w-full mt-3 px-3 py-2 text-sm bg-secondary/20 hover:bg-secondary/30 text-text rounded transition-colors"
        >
          {{
            showAllGames
              ? "Show Less"
              : `Show All ${topGamesByPlaytime.length} Games`
          }}
        </button>
      </div>
    </ExpandableSection>

    <!-- Recent Games -->
    <ExpandableSection
      title="Recently Played"
      icon="⏰"
      :count="recentGames.length"
      :default-expanded="false"
    >
      <div class="space-y-2">
        <div class="text-xs text-sub mb-3 flex items-center gap-2">
          💻 Local data - Last played times
        </div>
        <div class="grid grid-cols-1 lg:grid-cols-2 gap-3">
          <GamePreviewCard
            v-for="app in recentGames"
            :key="app.app_id"
            :app="app"
            :show-playtime="true"
            :show-last-played="true"
          />
        </div>
      </div>
    </ExpandableSection>

    <!-- Full Game Library -->
    <ExpandableSection
      title="Complete Game Library"
      icon="📚"
      :count="gamingStats.totalGames"
      :default-expanded="false"
    >
      <div class="space-y-3">
        <div class="text-xs text-sub">
          💻 Complete local game library - {{ gamingStats.totalGames }} games
          total
        </div>

        <!-- Search/Filter placeholder -->
        <div
          class="p-3 bg-secondary/10 rounded border border-secondary/30 text-center text-sub"
        >
          <div class="text-sm">
            🚧 Full library viewer with search/filter coming soon
          </div>
          <div class="text-xs mt-1">
            This will show all {{ gamingStats.totalGames }} games with search,
            sort, and filter options
          </div>
        </div>

        <!-- Quick sample of all games -->
        <div
          class="grid grid-cols-1 lg:grid-cols-2 gap-2 max-h-96 overflow-y-auto"
        >
          <GamePreviewCard
            v-for="app in user.app_list.slice(0, 20)"
            :key="app.app_id"
            :app="app"
            :show-playtime="true"
            :show-last-played="false"
            size="sm"
          />
        </div>

        <div
          v-if="user.app_list.length > 20"
          class="text-xs text-center text-sub"
        >
          Showing first 20 of {{ user.app_list.length }} games
        </div>
      </div>
    </ExpandableSection>
  </div>
</template>
