# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Tauri v2 desktop application built with TypeScript/Vanilla JS frontend and Rust backend. The project enables building cross-platform native applications with web technologies.

**Note:** The project is named "tranform-images" (appears to be a typo for "transform-images") throughout the codebase.

## Tech Stack

- **Frontend:** Vanilla TypeScript, Vite, HTML/CSS
- **Backend:** Rust with Tauri v2
- **Build Tool:** Vite (frontend), Cargo (Rust)
- **Package Manager:** Bun (configured in tauri.conf.json)

## Development Commands

### Running the Application

```bash
# Development mode (starts Vite dev server + Tauri window)
npm run tauri dev
# or using the configured package manager
bun run tauri dev
```

### Building

```bash
# Build TypeScript and frontend assets
npm run build
# or
bun run build

# Build the complete Tauri application (creates distributable)
npm run tauri build
# or
bun run tauri build
```

### Frontend Only

```bash
# Run Vite dev server only (without Tauri)
npm run dev
# or
bun run dev

# Preview production build
npm run preview
# or
bun run preview
```

### Rust Backend

```bash
# From src-tauri directory:
cd src-tauri

# Build Rust code
cargo build

# Run tests
cargo test

# Check code without building
cargo check
```

## Architecture

### Frontend Architecture

- **Entry Point:** `index.html` → loads `/src/main.ts`
- **Main Logic:** `src/main.ts` handles UI interactions and invokes Rust commands via `@tauri-apps/api/core`
- **Styling:** `src/styles.css`
- **Assets:** Located in `src/assets/`

### Backend Architecture

- **Entry Point:** `src-tauri/src/main.rs` - minimal binary entry that calls the library
- **Core Logic:** `src-tauri/src/lib.rs` - contains Tauri app setup and command handlers
- **Library Name:** `tranform_images_lib` (configured to avoid Windows naming conflicts)
- **Build Configuration:** `src-tauri/build.rs`

### Communication Pattern

Frontend and backend communicate using Tauri's command system:

1. Frontend invokes Rust functions using `invoke()` from `@tauri-apps/api/core`
2. Backend functions must be decorated with `#[tauri::command]`
3. Commands must be registered in the `invoke_handler` in `lib.rs`

Example flow:
```typescript
// Frontend (src/main.ts)
const result = await invoke("greet", { name: "World" });
```

```rust
// Backend (src-tauri/src/lib.rs)
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

// Register in Builder
.invoke_handler(tauri::generate_handler![greet])
```

### Configuration Files

- **`src-tauri/tauri.conf.json`:** Main Tauri configuration
  - Window settings (800x600 default)
  - Build commands (uses bun)
  - Dev server URL: http://localhost:1420
  - Frontend dist: `../dist`
  - App identifier: `com.laruina.tranform-images`

- **`src-tauri/Cargo.toml`:** Rust dependencies and package metadata
  - Configured as both library and binary
  - Key plugins: `tauri-plugin-opener`

- **`vite.config.ts`:** Frontend build configuration
  - Fixed port: 1420 (required by Tauri)
  - Ignores `src-tauri` directory from watch
  - HMR port: 1421

- **`src-tauri/capabilities/default.json`:** Tauri permissions and capabilities

### Library Structure

The Rust code is configured as a library (`lib.rs`) with a thin binary wrapper (`main.rs`) to:
- Enable better code organization
- Avoid naming conflicts on Windows
- Allow the library to be used in tests and other contexts

## Key Patterns

### Adding a New Tauri Command

1. Define the command in `src-tauri/src/lib.rs`:
   ```rust
   #[tauri::command]
   fn my_command(param: &str) -> Result<String, String> {
       // implementation
   }
   ```

2. Register it in the builder:
   ```rust
   .invoke_handler(tauri::generate_handler![greet, my_command])
   ```

3. Call from frontend:
   ```typescript
   import { invoke } from "@tauri-apps/api/core";
   const result = await invoke("my_command", { param: "value" });
   ```

### TypeScript Configuration

The project uses strict TypeScript settings:
- Target: ES2020
- Strict mode enabled
- No unused locals/parameters
- Module resolution: bundler mode

## Installed Plugins

- **tauri-plugin-opener:** Allows opening URLs and files with default system applications
