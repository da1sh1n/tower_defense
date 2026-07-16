# CLAUDE.md

Tower defense prototype in Rust + macroquad (0.4.15, edition 2024).

## Working with this repo

The user writes the game code themselves. Don't write or edit Rust source files (`src/**/*.rs`) unless explicitly asked to — default to planning, reviewing, and documentation. Project rules and decisions belong here, in-repo, rather than in any assistant-side memory.

## Naming conventions

Defined in `naming_comment_conventions.md`, followed literally. This is a deliberate, cross-project preference — the camelCase function rule does not match idiomatic Rust (snake_case functions), and that's intentional, not an oversight.

- Constants: `SCREAMING_SNAKE`
- Structs / enums / enum variants: `PascalCase`
- Functions / methods: `camelCase` — add `#![allow(non_snake_case)]` at the crate root so rustc's built-in lint doesn't warn on every function
- Variables / fields / parameters: `snake_case`
- Files / modules: `snake_case`
- Private variables: leading underscore
- Section banners inside larger files: `========== TITLE ==========` or `---------- Title ----------`

## Key architecture decisions

- **IDs**: tower IDs and monster IDs are separate types, each with its own independent counter — not one shared global id space. No parallel id list is kept; the id-to-entity hashmaps are iterated directly when "all live ids" are needed.
- **Per-frame update order**: monsters move, then towers attack. Towers react to each monster's actual position that frame rather than a stale one.
- **Tower dispatch**: one enum covering all tower kinds, with plain functions branching on it — not trait objects. An exhaustive match forces every call site to handle new kinds at compile time; the `Basic` kind (no-op, wall only) is the equivalent of "inherits do-nothing from a default."
- **Grid**: two separate grids. A coarse placement grid (1 tower = 2×2 tiles; dense array, not a hashmap) tracks empty/tower/goal state. A finer grid derived from it (subdivided, tunable factor) is used only for pathfinding (blocked/not-blocked).
- **Pathfinding**: A* (8-directional, octile heuristic, integer step costs — `f32` isn't `Ord` so it can't drive a `BinaryHeap` directly) over the fine grid, then string-pulled into world-space waypoints so monster movement looks continuous instead of stepping cell-to-cell. Recomputed once per wave (towers are static during a wave) and shared by every monster spawned that wave. Towers act purely as maze walls; enemies are never grid-locked for movement.
- **Damage pipeline**: roll crit → if the target has shield, the crit is discarded entirely for this hit → apply flying-damage bonus if relevant → apply defense (percent-based reduction, not flat) → subtract from shield first, overflow into health.
- **Round == wave**: one counter currently serves both the spawn cadence and the meta/round-based mechanics (rarity stage lookup, XP kill-credit recency window). Revisit if a round ever needs to span multiple waves.
- **Window/camera**: the window is resizable, not fixed-size. The camera always fills the entire window (its aspect ratio tracks the window's, recomputed every frame) — there is no letterboxing. The "world" is the grid plus a grey border around it (~10% of grid width, ~5% of grid height, split evenly per side); the camera is clamped to that combined area, and at max zoom-out shows at most ~80% of the grid itself. Built via macroquad's `Camera2D`, rebuilt fresh each frame rather than cached, since the window can resize at any time.

## Build order

Full phased roadmap and per-phase checklist: `TODO.md`.
