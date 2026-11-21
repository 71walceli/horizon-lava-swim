# Lava Swim for Horizon

A native [Horizon](https://github.com/horizon-rust/horizon) mod (Minecraft 26.3): with Fire Resistance, a
player swims in lava the way they swim in water. It has sprint swimming, faster acceleration, water's
vertical drag in deep lava, no extra lava gravity, water's climb impulse and a sneak dive.

This is a port of Lava Swim 1.2.1, a Fabric mod for 1.21.11 (`dev.wceli.lavaswim`).

## Status: needs a patched Horizon

Every function the mod changes lives in `horizon_client_lib::motion` and `::pose`. Horizon's public mod API
can't reach them yet, so on a stock Horizon `native-build` refuses each mixin row with
`E_MIXIN_TARGET_MISSING`. The mod builds and runs on a local developer base built from the fork branch
[`71walceli/horizon@lava-swim/replaceable-fluid-motion`](https://github.com/71walceli/horizon/tree/lava-swim/replaceable-fluid-motion),
which adds two commits on top of `0b14f8ac`:

1. `6957c6d`: marks `MotionWorld::player_travel`, `MotionWorld::dry_tick`, `MotionWorld::fluid_base_tick`,
   `PlayerMotion::sprint_conditions` and `PlayerMotion::moving_slowly` as `#[replaceable]`.
2. `49798f6`: makes `living_attributes::attribute_error`, `MotionWorld::tick_remaining_fire` and
   `MotionWorld::jump_phase` `pub`, so the replaceable copies compile in `horizon_patched_client_lib`.

The upstream request for public hooks that would replace this patch is tracked in
[horizon-rust/horizon: hooks for swimming and fluid travel in lava](https://github.com/horizon-rust/horizon/issues?q=is%3Aissue+can_swim_in_fluid).

## Layout

| Path | What it does |
| --- | --- |
| `src/lib.rs` | The movement rules as pure functions (eligibility, swim flag, acceleration, drag), with unit tests. |
| `src/mixins/travel.rs` | `Player.travel`'s lava branch (`LivingEntity.travelInLava`). |
| `src/mixins/swimming.rs` | `Player.updateSwimming`: starts or keeps the swim flag in lava. |
| `src/mixins/dive.rs` | `LocalPlayer.goDownInWater` in lava: the sneak dive. |
| `src/mixins/sprint.rs` | `shouldStopSwimSprinting` and `canSprint`'s water checks. |
| `src/mixins/slow.rs` | `shouldSlowDown`'s crawling check. |

## Build

`horizon-mod.toml` is pinned to `0b14f8ac`, the commit the SDK is fetched from. To install it on a developer
base built from the fork branch, change the `horizon` pin in the installed copy to `49798f6`, then run:

```sh
horizon-launcher native-build <mods dir> --instance <instance> --version <developer version> --trust-native-code
```

## Tested

- Unit tests for the rules in `src/lib.rs`.
- In game, deep lava, a 3-second forward sprint covers 6.18 blocks with Fire Resistance and 3.45 without.
- The sneak dive and the swim pose still need a manual check.

Ported with Claude Opus 5.5 via Claude Code.
