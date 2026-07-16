# TODO

Build order: structure → rendering → placement/movement → attacks → gameplay polish → UI → textures → effects. Naming conventions and locked-in architecture decisions are in `CLAUDE.md` — read that first.

## Phase 1 — Basic structure
- [x] Create `CLAUDE.md` at the repo root recording naming conventions and resolved decisions.
- [ ] Add a crate-level allow for the `non_snake_case` lint in `main.rs` — needed before writing the first camelCase function, otherwise every build warns.
- [ ] Configure the window as resizable, not a fixed size — the camera (Phase 2) is what adapts to whatever size the window ends up being, not the other way around.
- [ ] Sanity check: `cargo run` still shows the current placeholder scene before any structural change.

## Phase 2 — Basic rendering (squares, lines, circles)
- [ ] Add a constants file: tile size, grid width/height in cells, border-width/-height fractions, camera min/max zoom, camera pan/zoom speed — one source of truth for all of it from the start.
- [ ] `camera.rs`: camera state (position + zoom), keyboard-pan/scroll-zoom input handling, zoom clamping (≤80% of grid visible at max zoom-out), pan clamping (stay within grid+border), and building the actual `Camera2D` fresh each frame from that state.
- [ ] Rendering: draw the grey border, then the grid area on top of it, then a grid-line routine over that — all drawn in world coordinates with the camera already active (`set_camera`), not computed against raw `screen_width()/screen_height()` like the current placeholder does.
- [ ] Swap `main.rs`'s placeholder rectangle/circle/text for the new border+grid rendering, driven by the camera, just to prove the whole pipeline (resizable window → camera → world-space drawing) works end to end.

## Phase 3 — Tower placement + monster movement
- [ ] Tower id / monster id types.
- [ ] Grid position, cell state, and the placement grid, including one hardcoded default map (fixed spawn tile, a couple of goal tiles, base tile) — map authoring/editing isn't in scope at all right now, a single baked-in layout is enough.
- [ ] Tower kind / stats / entity shapes — every tower placed in this phase is the `Basic` kind, since the roll system doesn't exist until Phase 5.
- [ ] Monster entity shape, with just one placeholder kind.
- [ ] Pathfinding: the fine occupancy grid, A*, and the string-pulling simplification step.
- [ ] Wave state/phase, plus one hardcoded wave definition — just enough to see monsters move.
- [ ] Input handling: mouse click → screen-to-world via that frame's `Camera2D` (from `camera.rs`) → convert to grid coordinates → only allow placement when between waves → place the tower. Clicks that land in the grey border (outside the grid) are just a no-op.
- [ ] Assemble the top-level game state; the per-frame update should cover spawning and movement only at this point, nothing else yet.
- [ ] Rendering: draw towers as squares, monsters as circles, and — genuinely worth keeping even after this phase — a debug overlay of the current wave's path, so any A*/smoothing weirdness is visible immediately rather than inferred from monster behavior.

## Phase 4 — Tower attacks
- [ ] Pull in a random-number crate (needed for crit rolls — nothing before this phase needed real randomness).
- [ ] Add the first real attacking tower kind, with its attack cooldown actually ticking down.
- [ ] Combat logic: target acquisition (find something in range) and attack resolution (range check, crit roll, flat damage, death check). Shield/defense/resistances/effects stay unused fields until Phase 5 — don't build that part of the pipeline yet.
- [ ] Restructure the per-frame update into the agreed order: move monsters, tick effects (no-op for now), let towers attack, then a single pass that removes anything dead or that reached the base.
- [ ] Make sure "monster reached base" removal and "monster died in combat" removal both go through that same single cleanup pass rather than being handled in two different places.

## Phase 5 — Gameplay polish (stats, tower types, monsters)
- [ ] Weighted-roll table, rarity enum, and the stage progression; wire the type/color roll (uniform by default, skewable) and the rarity roll (stage-dependent) into tower placement.
- [ ] Flesh out effects: damage-over-time ticking, resistance application; settle the `Money` effect and bonus-vs-flying questions for real once they're actually being used.
- [ ] Extend the damage pipeline to the full sequence: shield, crit-negation, defense, resistance-adjusted effects.
- [ ] Expand the tower-kind roster; a per-kind base-stats table plus rarity scaling; wire up real leveling (xp from kills, gated by the damage-log recency window) and the upgrade-tier path.
- [ ] Round mechanic: offer some number of candidate towers, let the player keep a smaller number, convert the rest to `Basic`.
- [ ] Implement the actual combo-matching algorithm (pattern of neighboring tower kinds → consume + upgrade).
- [ ] Expand the monster roster and template table; decide and implement the flying-bypasses-the-maze question.

## Phase 6 — UI
- [ ] Tower-choice UI for the round offer/keep mechanic.
- [ ] Wave-start control, plus a money / base-health / wave-number HUD.
- [ ] Make sure UI clicks are intercepted before they fall through to grid placement input.

## Phase 7 — Textures
- [ ] Texture loading and a kind-keyed lookup table (tower kind → texture, monster kind → texture).
- [ ] Swap primitive draw calls for textured ones — call sites shouldn't need to change, just what happens inside the rendering routine.

## Phase 8 — Effects/juice
- [ ] A simple particle system for hit/death/placement feedback.
- [ ] Optional cosmetic projectile-travel visuals — purely visual, damage should still apply at attack-trigger time so this doesn't reopen Phase 4's combat timing.
- [ ] Audio hooks (macroquad's audio support) tied to combat/UI events.

## Assumptions to confirm along the way

- No base damage stat was in the original tower-stats list — added, since attack speed needs something to scale.
- Bonus damage vs. flying is a plain multiplier stat, not one of the applied effects (no duration/resistance, doesn't fit that shape).
- `Money` effect assumed to be an on-kill bonus-gold marker, not a tower-side passive economy trait — confirm before Phase 5 wires it up.
- Defense is a percent reduction, not flat.
- "Round" and "wave" are treated as the same counter for now (see `CLAUDE.md`).
- Whether flying monsters bypass the maze entirely isn't decided yet — doesn't block Phases 1-4.
- No monster pooling — confirmed skip for this prototype.
- Border split: 10% width / 5% height assumed to mean total added border, split evenly between both sides of each axis. If it was meant per-side instead, that's a one-constant change.
- Camera zoom assumed uniform (scales both axes together) so shapes never distort as you zoom.
- Camera/letterbox approach assumes macroquad's `Camera2D` handles the resize-every-frame + screen-to-world conversion cleanly — worth confirming against actual behavior once it's on screen (y-axis convention especially).
