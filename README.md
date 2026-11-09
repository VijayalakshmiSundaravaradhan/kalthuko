# Kalthuko - Phase 0

Kalthuko is a language-learning application prototype built with React and Tauri.
This project currently focuses on Phase 0: UI scaffold and integration with a language model backend.

## Prerequisites
- Node.js >= 20
- npm >= 10
- Rust toolchain (for Tauri): rustc, cargo, and the standard build tools
- Taura CLI dependencies handled by npm install

## Quick Start
1. Install dependencies
   - npm install
2. Run for development (UI + Electron-like packaging via Tauri)
   - npm run tauri dev
   or run the web UI separately:
   - npm run dev:web
3. Build for distribution
   - npm run tauri build

## Project structure
- src/: React UI core (App.jsx, main.jsx, styles.css)
- docs/: documentation and progress tracking (PLAN.md, PROGRESS.md)
- package.json: project metadata and build scripts

## How to contribute
- Open issues and feature requests in the repo. Follow the existing coding conventions.
- Add tests for any new functionality you introduce.

## License
This project is under the MIT License.
