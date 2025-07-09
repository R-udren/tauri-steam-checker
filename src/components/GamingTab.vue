<script setup lang="ts">
import { computed } from "vue";
import type { FetchedProfile, SteamUser } from "../types";

interface Props {
  user: SteamUser;
  profile?: FetchedProfile;
}

const props = defineProps<Props>();

// Top played games (by playtime)
const topGames = computed(() => {
  return [...props.user.apps]
    .sort((a, b) => b.playtime_minutes - a.playtime_minutes)
    .slice(0, 6);
});

// Recently played games
const recentGames = computed(() => {
  return [...props.user.apps]
    .filter((app) => app.last_played > 0)
    .sort((a, b) => b.last_played - a.last_played)
    .slice(0, 6);
});

// Gaming stats
const stats = computed(() => {
  const totalHours =
    props.user.apps.reduce((sum, app) => sum + app.playtime_minutes, 0) / 60;
  const playedGames = props.user.apps.filter(
    (app) => app.playtime_minutes > 0
  ).length;
  const avgHoursPerGame = playedGames > 0 ? totalHours / playedGames : 0;

  return {
    totalGames: props.user.apps.length,
    playedGames,
    totalHours: Math.round(totalHours * 10) / 10,
    avgHoursPerGame: Math.round(avgHoursPerGame * 10) / 10,
  };
});

function formatHours(minutes: number): string {
  const hours = Math.round((minutes / 60) * 10) / 10;
  return `${hours}h`;
}

function formatLastPlayed(timestamp: number): string {
  if (timestamp === 0) return "Never";

  const date = new Date(timestamp * 1000);
  const now = new Date();
  const diffDays = Math.floor(
    (now.getTime() - date.getTime()) / (1000 * 60 * 60 * 24)
  );

  if (diffDays === 0) return "Today";
  if (diffDays === 1) return "Yesterday";
  if (diffDays < 7) return `${diffDays} days ago`;
  if (diffDays < 30) return `${Math.floor(diffDays / 7)} weeks ago`;
  return date.toLocaleDateString();
}
</script>

<template>
  <div class="space-y-6">
    <!-- Gaming Statistics -->
    <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
      <div class="bg-bg-tertiary p-3 rounded-lg border border-border">
        <div class="text-xl font-bold text-text">{{ stats.totalGames }}</div>
        <div class="text-xs text-text-muted uppercase tracking-wide">
          Total Games
        </div>
      </div>

      <div class="bg-bg-tertiary p-3 rounded-lg border border-border">
        <div class="text-xl font-bold text-text">{{ stats.playedGames }}</div>
        <div class="text-xs text-text-muted uppercase tracking-wide">
          Played
        </div>
      </div>

      <div class="bg-bg-tertiary p-3 rounded-lg border border-border">
        <div class="text-xl font-bold text-text">{{ stats.totalHours }}h</div>
        <div class="text-xs text-text-muted uppercase tracking-wide">
          Total Hours
        </div>
      </div>

      <div class="bg-bg-tertiary p-3 rounded-lg border border-border">
        <div class="text-xl font-bold text-text">
          {{ stats.avgHoursPerGame }}h
        </div>
        <div class="text-xs text-text-muted uppercase tracking-wide">
          Avg per Game
        </div>
      </div>
    </div>

    <!-- Most Played Games -->
    <div v-if="topGames.length > 0" class="space-y-3">
      <h3
        class="text-sm font-medium text-text-secondary flex items-center gap-2"
      >
        🏆 Most Played Games
      </h3>

      <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
        <div
          v-for="game in topGames"
          :key="game.app_id"
          class="flex items-center justify-between p-3 bg-bg-tertiary rounded-lg border border-border"
        >
          <div class="flex items-center gap-3">
            <div
              class="w-10 h-10 bg-bg-secondary rounded border border-border flex items-center justify-center"
            >
              <span class="text-xs text-text-muted">{{
                game.app_id.toString().slice(-2)
              }}</span>
            </div>
            <div>
              <div class="text-sm font-medium text-text">
                App {{ game.app_id }}
              </div>
              <div class="text-xs text-text-muted">
                {{ formatLastPlayed(game.last_played) }}
              </div>
            </div>
          </div>
          <div class="text-sm font-mono text-text-secondary">
            {{ formatHours(game.playtime_minutes) }}
          </div>
        </div>
      </div>
    </div>

    <!-- Recently Played Games -->
    <div v-if="recentGames.length > 0" class="space-y-3">
      <h3
        class="text-sm font-medium text-text-secondary flex items-center gap-2"
      >
        🕒 Recently Played
      </h3>

      <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
        <div
          v-for="game in recentGames"
          :key="game.app_id"
          class="flex items-center justify-between p-3 bg-bg-tertiary rounded-lg border border-border"
        >
          <div class="flex items-center gap-3">
            <div
              class="w-10 h-10 bg-bg-secondary rounded border border-border flex items-center justify-center"
            >
              <span class="text-xs text-text-muted">{{
                game.app_id.toString().slice(-2)
              }}</span>
            </div>
            <div>
              <div class="text-sm font-medium text-text">
                App {{ game.app_id }}
              </div>
              <div class="text-xs text-text-muted">
                {{ formatHours(game.playtime_minutes) }}
              </div>
            </div>
          </div>
          <div class="text-xs text-text-secondary">
            {{ formatLastPlayed(game.last_played) }}
          </div>
        </div>
      </div>
    </div>

    <!-- Empty State -->
    <div v-if="topGames.length === 0" class="text-center py-8">
      <div class="text-4xl mb-2">🎮</div>
      <div class="text-text-muted">No game data available</div>
    </div>
  </div>
</template>
