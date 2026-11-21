//! Lava Swim for Horizon: with Fire Resistance, a player swims in lava as in water.
//!
//! Port of Lava Swim 1.2.1 (Fabric 1.21.11, dev.wceli.lavaswim). The movement decisions are the pure
//! functions below, the same as `LavaSwimPolicy`, `LavaSwimEligibility` and `LavaSwimClientPolicy`. The
//! mixin helpers under `src/mixins/` apply them inside
//! Horizon's player movement (`horizon_client::motion`), which runs for the local player's prediction and
//! for the server's copy of the player alike.
use mod_api::{self, Hooks};

/// Horizontal acceleration multiplier in lava while not sprinting.
pub const BASE_HORIZONTAL_ACCELERATION_MULTIPLIER: f64 = 2.5;
/// Horizontal acceleration multiplier in lava while sprinting.
pub const SPRINT_HORIZONTAL_ACCELERATION_MULTIPLIER: f64 = 5.0;
/// Vanilla water's vertical drag, `0.8f` widened to double (`0.800000011920929`).
pub const VANILLA_WATER_VERTICAL_DRAG: f64 = 0.8f32 as f64;
/// Water's collision-assisted climb impulse.
pub const WATER_CLIMB_VELOCITY_Y: f64 = 0.2;

/// `LavaSwimEligibility.isEligible`: a player with active Fire Resistance. Never cached.
pub fn is_eligible(is_player: bool, has_fire_resistance: bool) -> bool {
    is_player && has_fire_resistance
}

/// `LavaSwimPolicy.shouldEnableLavaSwimming`: keep or start the swim flag in lava instead of vanilla's
/// water-only `updateSwimming`.
#[allow(clippy::too_many_arguments)]
pub fn should_enable_lava_swimming(
    currently_swimming: bool,
    is_sprinting: bool,
    is_submerged_in_lava: bool,
    feet_in_lava: bool,
    is_touching_lava: bool,
    has_vehicle: bool,
    is_flying: bool,
    is_spectator: bool,
    is_sleeping: bool,
    is_gliding: bool,
    is_using_riptide: bool,
) -> bool {
    if !is_sprinting || has_vehicle || is_flying || is_spectator || is_sleeping || is_gliding || is_using_riptide {
        return false;
    }
    if currently_swimming { is_touching_lava } else { is_submerged_in_lava && feet_in_lava }
}

pub fn horizontal_acceleration_multiplier(is_sprinting: bool) -> f64 {
    if is_sprinting { SPRINT_HORIZONTAL_ACCELERATION_MULTIPLIER } else { BASE_HORIZONTAL_ACCELERATION_MULTIPLIER }
}

/// `LavaSwimPolicy.applyPlayerHorizontalBoost`: scales only the horizontal change `moveRelative` made,
/// keeping the momentum from before it. `before` and `after` are `(x, z)`; Y is untouched by the caller.
pub fn horizontal_boost(before: (f64, f64), after: (f64, f64), is_sprinting: bool) -> (f64, f64) {
    let extra = horizontal_acceleration_multiplier(is_sprinting) - 1.0;
    (after.0 + (after.0 - before.0) * extra, after.1 + (after.1 - before.1) * extra)
}

/// `LavaSwimClientPolicy.shouldSlowForCrawling`: the swimming pose in lava is not a crawl.
pub fn should_slow_for_crawling(vanilla_crawling: bool, swimming: bool, in_lava: bool, eligible: bool) -> bool {
    vanilla_crawling && !(eligible && swimming && in_lava)
}

/// `LavaSwimClientPolicy.isPartlyTouchingEquivalentFluid`: shallow lava blocks starting a sprint as shallow
/// water does.
pub fn is_partly_touching_equivalent_fluid(
    partly_touching_water: bool,
    in_lava: bool,
    submerged_in_lava: bool,
    eligible: bool,
) -> bool {
    partly_touching_water || (eligible && in_lava && !submerged_in_lava)
}

/// `LavaSwimClientPolicy.shouldApplyShiftDive`: sneaking sinks faster in any lava contact.
pub fn should_apply_shift_dive(touching_water: bool, in_lava: bool, eligible: bool) -> bool {
    touching_water || (eligible && in_lava)
}

/// Whether Horizon's water-only `goDownInWater` site missed a dive this port must add: lava contact
/// without water (water already dived), sneaking and not flying, as vanilla's own gate.
pub fn needs_extra_lava_dive(
    touching_water: bool,
    in_lava: bool,
    eligible: bool,
    shift_key_down: bool,
    flying: bool,
) -> bool {
    !touching_water && shift_key_down && !flying && should_apply_shift_dive(touching_water, in_lava, eligible)
}

/// `LavaSwimClientPolicy.isTouchingEquivalentSwimmingFluid`: a lava swim does not stop sprint swimming.
pub fn is_touching_equivalent_swimming_fluid(touching_water: bool, swimming: bool, in_lava: bool, eligible: bool) -> bool {
    touching_water || (eligible && swimming && in_lava)
}

#[derive(Default)]
struct Mod;

#[mod_api::horizon_mod]
impl Hooks for Mod {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eligibility_needs_a_player_with_fire_resistance() {
        assert!(is_eligible(true, true));
        assert!(!is_eligible(true, false));
        assert!(!is_eligible(false, true));
        assert!(!is_eligible(false, false));
    }

    #[test]
    fn swimming_starts_submerged_with_feet_in_lava_and_continues_while_touching() {
        let base = |swimming, submerged, feet, touching| {
            should_enable_lava_swimming(swimming, true, submerged, feet, touching, false, false, false, false, false, false)
        };
        assert!(base(false, true, true, true));
        assert!(!base(false, true, false, true));
        assert!(!base(false, false, true, true));
        assert!(base(true, false, false, true));
        assert!(!base(true, true, true, false));
    }

    #[test]
    fn swimming_needs_sprint_and_no_blocking_state() {
        let all = |sprint, vehicle, flying, spectator, sleeping, gliding, riptide| {
            should_enable_lava_swimming(false, sprint, true, true, true, vehicle, flying, spectator, sleeping, gliding, riptide)
        };
        assert!(all(true, false, false, false, false, false, false));
        assert!(!all(false, false, false, false, false, false, false));
        assert!(!all(true, true, false, false, false, false, false));
        assert!(!all(true, false, true, false, false, false, false));
        assert!(!all(true, false, false, true, false, false, false));
        assert!(!all(true, false, false, false, true, false, false));
        assert!(!all(true, false, false, false, false, true, false));
        assert!(!all(true, false, false, false, false, false, true));
    }

    #[test]
    fn horizontal_boost_scales_only_the_new_acceleration() {
        let (x, z) = horizontal_boost((0.1, -0.2), (0.12, -0.21), false);
        assert!((x - (0.12 + 0.02 * 1.5)).abs() < 1e-12);
        assert!((z - (-0.21 - 0.01 * 1.5)).abs() < 1e-12);
        let (x, z) = horizontal_boost((0.0, 0.0), (0.02, 0.0), true);
        assert!((x - 0.1).abs() < 1e-12);
        assert_eq!(z, 0.0);
        assert_eq!(horizontal_boost((0.3, 0.4), (0.3, 0.4), true), (0.3, 0.4));
    }

    #[test]
    fn water_vertical_drag_is_vanillas_float() {
        assert_eq!(VANILLA_WATER_VERTICAL_DRAG, 0.800000011920929);
    }

    #[test]
    fn client_gates_match_the_java_truth_tables() {
        // shouldSlowForCrawling(vanillaCrawling, swimming, inLava, eligible)
        assert!(!should_slow_for_crawling(true, true, true, true));
        assert!(should_slow_for_crawling(true, true, true, false));
        assert!(should_slow_for_crawling(true, false, true, true));
        assert!(!should_slow_for_crawling(false, true, true, true));
        // isPartlyTouchingEquivalentFluid(partlyWater, inLava, submergedLava, eligible)
        assert!(is_partly_touching_equivalent_fluid(true, false, false, false));
        assert!(is_partly_touching_equivalent_fluid(false, true, false, true));
        assert!(!is_partly_touching_equivalent_fluid(false, true, true, true));
        assert!(!is_partly_touching_equivalent_fluid(false, true, false, false));
        // shouldApplyShiftDive(touchingWater, inLava, eligible) = (ZZZ)Z in 1.2.1
        assert!(should_apply_shift_dive(true, false, false));
        assert!(should_apply_shift_dive(false, true, true));
        assert!(!should_apply_shift_dive(false, true, false));
        assert!(!should_apply_shift_dive(false, false, true));
        // isTouchingEquivalentSwimmingFluid(touchingWater, swimming, inLava, eligible)
        assert!(is_touching_equivalent_swimming_fluid(true, false, false, false));
        assert!(is_touching_equivalent_swimming_fluid(false, true, true, true));
        assert!(!is_touching_equivalent_swimming_fluid(false, false, true, true));
        assert!(!is_touching_equivalent_swimming_fluid(false, true, true, false));
    }

    #[test]
    fn extra_dive_only_where_vanilla_did_not_dive() {
        assert!(needs_extra_lava_dive(false, true, true, true, false));
        assert!(!needs_extra_lava_dive(true, true, true, true, false), "water already dived");
        assert!(!needs_extra_lava_dive(false, true, true, false, false), "not sneaking");
        assert!(!needs_extra_lava_dive(false, true, true, true, true), "flying");
        assert!(!needs_extra_lava_dive(false, true, false, true, false), "no Fire Resistance");
        assert!(!needs_extra_lava_dive(false, false, true, true, false), "no lava");
    }
}
