# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Tauri v2 desktop wrapper for **app.useweve.com**. The application embeds the Weve web app in a native desktop window.

## Build and Development Commands

```bash
bun run tauri dev    # Run desktop app in development mode
bun run tauri build  # Build production desktop app (platform-specific bundles)
```

## Architecture

This is a minimal Tauri app that loads an external URL - there is no local frontend.

**Backend (`/src-tauri/`):** Rust code with Tauri framework. The main window loads `https://app.useweve.com` directly.

**Key files:**
- `src-tauri/tauri.conf.json` - Main Tauri configuration, bundle settings
- `src-tauri/capabilities/default.json` - Permissions for remote URL access and notifications
- `src-tauri/src/lib.rs` - Window creation and notification bridge script

**Notification Bridge:** The app injects a script that intercepts the browser's `Notification` API and redirects to native OS notifications via `tauri-plugin-notification`. The web app can use the standard `new Notification()` API and it will show native desktop notifications.

## Configuration

- **App identifier:** `com.weve.desktop-app`
- **Window size:** 1280x800
- **Remote URLs allowed:** `https://app.useweve.com/*`, `https://*.useweve.com/*`

## Adding Native Features

To add Rust commands callable from the web app:
1. Add function with `#[tauri::command]` in `src-tauri/src/lib.rs`
2. Register in `invoke_handler` on the Tauri builder
3. Add necessary permissions in `capabilities/default.json`
