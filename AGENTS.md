# Repository Guidelines

## Project Structure & Module Organization

This is a Rust 2021 desktop game built with Bevy 0.14. The application entrypoint is `src/main.rs`. Gameplay systems, player state, combat, background, and shared game data live in `src/game/mod.rs`, with enemy behavior split into `src/game/enemies.rs`. UI setup and state handling live in `src/ui/mod.rs`. Plugin wiring is kept in `src/plugins/mod.rs`. Runtime art assets are stored in `assets/sprites/`; keep new sprite filenames lowercase and hyphenated, for example `shield-carrier.png`. Build output in `target/` is generated and should not be edited.

## Build, Test, and Development Commands

- `cargo run` starts the Bevy game locally.
- `cargo build` compiles the debug executable without launching it.
- `cargo check` performs a faster compile check for iteration.
- `cargo test` runs the inline unit tests across game, UI, and plugin modules.
- `cargo fmt` formats Rust code using the standard Rust formatter.

Run commands from the repository root so Bevy can resolve `assets/` paths correctly.

## Coding Style & Naming Conventions

Follow idiomatic Rust formatting with `cargo fmt`; use 4-space indentation and keep imports organized by `rustfmt`. Use `snake_case` for functions, variables, modules, and test names. Use `PascalCase` for structs, enums, resources, components, and plugins. Prefer Bevy ECS patterns already used in the codebase: components and resources hold data, while systems perform behavior. Keep gameplay constants close to the systems they tune unless they are shared across modules.

## Testing Guidelines

Tests are currently inline under `#[cfg(test)] mod tests` blocks in the relevant module files. Add focused unit tests next to the behavior they cover, especially for enemy spawning, combat math, UI state transitions, and plugin registration. Name tests by expected behavior, for example `drone_wave_size_stays_within_bounds`. Run `cargo test` before opening a pull request.

## Commit & Pull Request Guidelines

Recent commits use short imperative subjects such as `Add kamikaze drone waves` and `Fix kamikaze sprite rotation alignment`. Keep commit subjects concise, capitalized, and action-oriented.

Pull requests should include a brief summary, testing performed, and any gameplay or visual impact. Attach screenshots or short recordings for UI, sprite, movement, or animation changes. Link related issues when available and call out follow-up work separately from the implemented change.

## Asset & Configuration Notes

Do not commit generated `target/` contents. Keep `Cargo.lock` committed for reproducible application builds. When adding assets, reference them through stable paths under `assets/` and verify they load with `cargo run`.
