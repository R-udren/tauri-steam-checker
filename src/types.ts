export interface SteamUser {
  steam_id: string;
  nickname?: string;
  name_history: Array<string>;
  most_recent: boolean;
  sources: Array<string>;
  time_stamp?: number;
  app_list: Array<App>;
}

export interface App {
  app_id: number;
  playtime_minutes: number;
  last_played: number;
}

export interface MostPlayedGame {
  gameName?: string;
  gameLink?: string;
  gameIcon?: string;
  gameLogo?: string;
  hoursOnRecord?: string;
}

export interface MostPlayedGames {
  mostPlayedGame: MostPlayedGame[];
}

export interface FetchedProfile {
  steamID64: string;
  steamID?: string;
  onlineState?: string;
  stateMessage?: string;
  VisibilityState?: string;
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
