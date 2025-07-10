# Tauri Steam Account Checker

A modern desktop app for scanning, viewing, and analyzing Steam user accounts. Built with **Tauri**, **Vue 3**, **TypeScript**, and **TailwindCSS** for a fast, beautiful, and privacy-friendly experience.

## Features

- 🔍 **Search** Steam users by nickname, SteamID, name history, or profile info
- 🖼️ **Real Steam game banners** and avatars
- 🧑‍💻 **Compact, scannable UI** with dark theme and high contrast
- 🏆 **Game stats**: hours, most played, recently played, and more
- 📝 **Profile & technical details**: previous names, account status, privacy, VAC/trade bans
- 📋 **Copy** any SteamID or all IDs at once
- 🌐 **Open Steam profiles** in your browser
- ⚡ **Fast**: All data is processed locally, no external servers

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Node.js](https://nodejs.org/)
- [deno](https://deno.land/) or [npm](https://www.npmjs.com/)

### Setup

```sh
deno install # or npm install
deno task tauri dev # or npm run tauri dev
```

### Usage

- Enter or load Steam user data (see app for details)
- Search, filter, and scan accounts
- Click any user for details, copy IDs, or open their Steam profile

## Tech Stack

- [Tauri](https://tauri.app/) — secure, lightweight desktop runtime
- [Vue 3](https://vuejs.org/) + [TypeScript](https://www.typescriptlang.org/)
- [TailwindCSS](https://tailwindcss.com/) — custom dark theme
- [Vite](https://vitejs.dev/) — lightning-fast dev/build

## Development

- Recommended: [VS Code](https://code.visualstudio.com/) + [Volar](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri extension](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
- All `.vue` files use `<script setup>` and full TypeScript support

## License

This project is **closed source** and not available for public distribution or modification.
