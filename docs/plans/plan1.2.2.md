# Plan 1.2.2: The Evolutionary Physics Core

This phase focuses on finalizing the transition from hardcoded logic to an emergent simulation where physics and genetics dictate survival.

## Phase 1: Total Data-Driven Cleanup
*Goal: Remove the last vestiges of "Identity-based" logic.*

- **Visual-only Enums**: `ItemType` and `ActorType` will no longer be used for `is_wall()` or `passable()`. They will function strictly as lookups for `SpriteIndex`.
- **Physical Passability**:
    - Units calculate pathfinding costs based on the `Mass` and `Toughness` of entities at a location.
    - Low mass/toughness = Passable (crush/push aside).
    - High mass/toughness = Obstacle.
- **Generic Spawning**: Data for `MaterialProperties` will be stored in template configurations or generated procedurally, rather than hardcoded in `match` statements inside `objects.rs`.

## Phase 2: Linking & Mechanical Emergence
*Goal: Implement the "Link" action to allow tools and structures.*

- **Atomic Link**: Implement logic where Entity A can "link" to Entity B (Bevy parent-child link).
- **Force Amplification (Tools)**: 
    - When `ApplyForce` is executed, check for linked entities.
    - If a unit is holding a "Stone" (high hardness), the damage calculation uses the Stone's hardness instead of the fist's.
- **Dynamic Insulation**:
    - Update `EnvironmentalData` systems to detect "Enclosures" (areas surrounded by high-toughness entities).
    - Closed areas dampen temperature and humidity changes.

## Phase 3: The Evolutionary Engine
*Goal: Closing the loop of life and death.*

- **Reproduction Plugin**:
    - Implement `reproduction_system` that spawns a new entity when `energy_storage > threshold`.
    - Energy is split between parent and child.
- **Genetic Drift**:
    - Apply small random offsets to all `Genome` floats (size, aggression, weights) during spawning.
- **Natural Selection**:
    - High-efficiency foragers (those with genes weighting `energy_density` search) will reproduce faster, eventually dominating the population.

## Phase 4: Observability (The God View)
*Goal: Visualizing the hidden logic.*

- **Phenotype Mapping**:
    - Red tint $\propto$ Aggression.
    - Blue tint $\propto$ Sociality.
    - Scale $\propto$ Size Gene.
- **Narrative Overlay**:
    - Use the `StatusDisplay` system to show: `Motivation (Hunger) -> Action (Move -> Consume)`.
