# Class evolution prototype

This branch begins a class-specific evolution system for all nine Classic classes, starting with Shaman.

## Design
- Keep the existing automatic combat and all nine class selections.
- Define recipes separately from existing ability definitions.
- A recipe unlocks when every prerequisite spell reaches its required rank.
- First proposed evolution: **Elemental Storm** from Lightning Bolt rank 3 and Chain Lightning rank 3.

## Status
The initial Rust recipe model and unit tests are in `crates/benilla-app/src/survivors/evolutions.rs`.
**This is a prototype, not yet wired into the game:** it does not currently appear as a level-up card, trigger effects, or alter combat. The ability keys and required ranks must be checked against the existing data tables before integration.

## Next implementation steps
1. Verify the Shaman ability keys in `data.rs`.
2. Register the evolution module in `mod.rs`.
3. Extend the level-up card offer logic to present an unlocked evolution.
4. Add its auto-cast combat behavior and visual effects.
5. Build and playtest outside the work PC before merging.
