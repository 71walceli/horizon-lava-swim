fn lava_swim_swimming(world: &mut MotionWorld<'_, '_, '_>) -> ::core::ops::ControlFlow<Result<()>> {
    if !mod_lava_swim::is_eligible(true, world.player.status.fire_resistance) {
        return ::core::ops::ControlFlow::Continue(());
    }
    // fluid_base_tick, with Player.updateSwimming's Lava Swim HEAD mixin before vanilla's water rule.
    world.player.fluids.was_eye_in_water = world.player.is_eye_in_fluid(WATER);
    if let ::core::result::Result::Err(e) = world.update_fluids() {
        return ::core::ops::ControlFlow::Break(::core::result::Result::Err(e));
    }
    let previously = world.player.swimming;
    // getFluidState(getBlockPos()).isIn(FluidTags.LAVA)
    let feet_in_lava = is_tagged(
        world.fluid_at(::horizon_core::position::BlockPos::containing(world.player.position)).kind(),
        LAVA,
    );
    let lava = mod_lava_swim::should_enable_lava_swimming(
        previously,
        world.player.sprinting,
        world.player.is_eye_in_fluid(LAVA),
        feet_in_lava,
        world.player.is_in_lava(),
        world.player.is_passenger,
        world.player.flying,
        world.player.spectator,
        world.player.sleeping,
        world.player.fall_flying,
        world.player.auto_spin_attack_ticks > 0,
    );
    world.player.swimming = lava
        || (!world.player.flying
            && !world.player.is_passenger
            && world.player.sprinting
            && if previously {
                world.player.fluids.was_touching_water
            } else {
                world.player.is_under_water()
                    && is_tagged(
                        world.fluid_at(::horizon_core::position::BlockPos::containing(world.player.position)).kind(),
                        WATER,
                    )
            });
    if let ::core::result::Result::Err(e) = world.tick_remaining_fire() {
        return ::core::ops::ControlFlow::Break(::core::result::Result::Err(e));
    }
    if world.player.is_in_lava() {
        world.player.fall_distance *= 0.5;
    }
    world.player.fluids.first_tick = false;
    ::core::ops::ControlFlow::Break(::core::result::Result::Ok(()))
}
