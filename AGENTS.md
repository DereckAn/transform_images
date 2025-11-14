# Repository Guidelines

## Project Structure & Module Organization
The Vite UI lives in `src/`, with orchestration under `app/services/ImageService.ts`, UI state in `app/state/AppState.ts`, shared contracts in `models/types.ts`, and icons in `assets/`. Keep emergent widgets in `components/` and low-level helpers in `utils/` to preserve the clean/hexagonal layering referenced in `main.ts`. The Rust backend sits in `src-tauri/` (`application/`, `domain/`, `infrastructure/`), uses `tauri.conf.json` for bundling, and stores integration specs in `src-tauri/tests/`. Build artifacts belong in `dist/` and `src-tauri/target/`—never commit them.

## Build, Test, and Development Commands
- `npm run dev` – Vite dev server for web-only iteration.
- `npm run tauri:dev` – Desktop shell with live reload across TS + Rust.
- `npm run build` – `tsc --noEmit` plus Vite production bundle to `dist/`.
- `npm run preview` – Serves `dist/` for production sanity checks.
- `npm run tauri:build` – Creates installers/binaries in `src-tauri/target/`.
- `cargo test` (inside `src-tauri/`) – Runs Rust unit/integration suites.

## Coding Style & Naming Conventions
Use 2-space indentation in TypeScript, keep services and state containers in PascalCase classes, and DOM refs/functions in camelCase. Centralize reusable `type`/`enum` definitions in `models/types.ts`, move shared logic into pure helpers under `utils/`, and keep Tailwind utility classes declarative in markup with global overrides confined to `styles.css`. Run `tsc` (via `npm run build`) before pushing. In Rust, keep files snake_case, prefer small traits per layer, and run `cargo fmt` to enforce formatting.

## Testing Guidelines
Automation currently lives in `src-tauri/tests/*_test.rs`; follow the naming pattern (`raw_transformation_test.rs`, `metadata_cleaner_test.rs`) and isolate fixtures through helper builders. Before running `cargo test`, point the `TEST_RAW_FILE` constant in `raw_transformation_test.rs` at a RAW sample that exists locally or mark the test `#[ignore]` to avoid false failures. Until a browser harness lands, describe any manual UI verification (drag/drop, metadata toggles, cancellation flows) inside your PR checklist.

## Commit & Pull Request Guidelines
History follows Conventional Commits (`feat:`, `refactor:`, etc.), so keep scopes short and describe behavior in present tense. Each PR should link its issue, summarize the change, attach screenshots or recordings for UI updates, and list the commands run (`npm run tauri:dev`, `cargo test`, etc.). Call out OS-specific steps (macOS permissions, Windows Defender exclusions) so reviewers can reproduce builds without surprises.
