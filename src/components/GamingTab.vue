<script setup lang="ts">
import { computed } from "vue";
import type { FetchedProfile, SteamUser } from "../types";

interface Props {
  user: SteamUser;
  profile?: FetchedProfile;
}

const props = defineProps<Props>();

// Specific games to highlight
const PRIORITY_GAMES = {
  252490: "Rust",
  480: "Spacewar",
  730: "CS2",
};

// Priority games (Rust, Spacewar, CS2)
const priorityGames = computed(() => {
  return props.user.apps
    .filter((app) => app.app_id in PRIORITY_GAMES)
    .sort((a, b) => b.playtime_minutes - a.playtime_minutes);
});

// Other games (excluding priority games)
const otherGames = computed(() => {
  return props.user.apps
    .filter((app) => !(app.app_id in PRIORITY_GAMES))
    .sort((a, b) => b.playtime_minutes - a.playtime_minutes);
});

// Top other games (for expandable section)
const topOtherGames = computed(() => {
  return otherGames.value.slice(0, 10);
});

// Recently played games (excluding priority games, for expandable)
const recentOtherGames = computed(() => {
  return otherGames.value
    .filter((app) => (app.last_played ?? 0) > 0)
    .sort((a, b) => b.last_played ?? 0 - (a.last_played ?? 0))
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

function getGameName(appId: number): string {
  return PRIORITY_GAMES[appId as keyof typeof PRIORITY_GAMES] || `App ${appId}`;
}

function formatHours(minutes: number): string {
  const hours = Math.round((minutes / 60) * 10) / 10;
  return `${hours}h`;
}

function formatLastPlayed(timestamp: number | undefined): string {
  if (timestamp === 0 || timestamp === undefined) return "Never";

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

function getGameHeaderUrl(appId: number): string {
  return `https://cdn.cloudflare.steamstatic.com/steam/apps/${appId}/header.jpg`;
}

function hideImageOnError(event: Event) {
  const target = event.target as HTMLImageElement | null;
  if (target) target.style.display = "none";
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

    <!-- Priority Games (Rust, Spacewar, CS2) -->
    <div v-if="priorityGames.length > 0" class="space-y-3">
      <h3
        class="text-sm font-medium text-text-secondary flex items-center gap-2"
      >
        ⭐ Featured Games
      </h3>
      <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
        <div
          v-for="game in priorityGames"
          :key="game.app_id"
          class="flex items-center justify-between p-3 bg-primary/10 border border-primary/30 rounded-lg"
        >
          <div class="flex items-center gap-3">
            <img
              :src="getGameHeaderUrl(game.app_id)"
              :alt="getGameName(game.app_id) + ' banner'"
              class="w-32 h-16 object-cover rounded border border-primary/40 bg-bg-secondary"
              loading="lazy"
              title="Steam game banner"
              @error="hideImageOnError"
            />
            <div>
              <div class="font-medium text-text">
                {{ getGameName(game.app_id) }}
              </div>
              <div class="text-sm text-text-muted">
                {{ formatLastPlayed(game.last_played) }}
              </div>
            </div>
          </div>
          <div class="font-mono font-semibold text-white">
            {{ formatHours(game.playtime_minutes) }}
          </div>
        </div>
      </div>
    </div>

    <!-- Other Games (Expandable) -->
    <div v-if="otherGames.length > 0" class="space-y-3">
      <details class="group">
        <summary
          class="text-sm font-medium text-text-secondary flex items-center gap-2 cursor-pointer hover:text-text transition-colors"
        >
          <span class="group-open:rotate-90 transition-transform">▶</span>
          👀 Other Games ({{ otherGames.length }})
        </summary>
        <div class="mt-3 space-y-4">
          <!-- Top Other Games -->
          <div v-if="topOtherGames.length > 0">
            <h4
              class="text-xs font-medium text-text-muted mb-2 uppercase tracking-wide"
            >
              Most Played
            </h4>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-2">
              <div
                v-for="game in topOtherGames"
                :key="game.app_id"
                class="flex items-center justify-between p-2 bg-bg-tertiary rounded border border-border"
              >
                <div class="flex items-center gap-2">
                  <img
                    :src="getGameHeaderUrl(game.app_id)"
                    :alt="getGameName(game.app_id) + ' banner'"
                    class="w-24 h-12 object-cover rounded border border-border bg-bg-secondary"
                    loading="lazy"
                    title="Steam game banner"
                    @error="hideImageOnError"
                  />
                  <div>
                    <div class="text-xs font-medium text-text">
                      App {{ game.app_id }}
                    </div>
                    <div class="text-xs text-text-muted">
                      {{ formatLastPlayed(game.last_played) }}
                    </div>
                  </div>
                </div>
                <div class="text-xs font-mono text-text-secondary">
                  {{ formatHours(game.playtime_minutes) }}
                </div>
              </div>
            </div>
          </div>

          <!-- Recent Other Games -->
          <div v-if="recentOtherGames.length > 0">
            <h4
              class="text-xs font-medium text-text-muted mb-2 uppercase tracking-wide"
            >
              Recently Played
            </h4>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-2">
              <div
                v-for="game in recentOtherGames"
                :key="game.app_id"
                class="flex items-center justify-between p-2 bg-bg-tertiary rounded border border-border"
              >
                <div class="flex items-center gap-2">
                  <img
                    :src="getGameHeaderUrl(game.app_id)"
                    :alt="getGameName(game.app_id) + ' banner'"
                    class="w-24 h-12 object-cover rounded border border-border bg-bg-secondary"
                    loading="lazy"
                    title="Steam game banner"
                    @error="hideImageOnError"
                  />
                  <div>
                    <div class="text-xs font-medium text-text">
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
        </div>
      </details>
    </div>

    <!-- Empty State -->
    <div
      v-if="priorityGames.length === 0 && otherGames.length === 0"
      class="text-center py-8"
    >
      <div class="text-4xl mb-2">🎮</div>
      <div class="text-text-muted">No game data available</div>
    </div>
  </div>
</template>
