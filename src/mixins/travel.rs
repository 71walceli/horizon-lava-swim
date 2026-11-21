fn lava_swim_travel(world: &mut MotionWorld<'_, '_, '_>, input: Vec3) -> ::core::ops::ControlFlow<Result<()>> {
    // Player.travel's fluid branch for an eligible player in lava (not water): Lava Swim's travelInLava.
    if world.player.flying
        || world.player.fluids.was_touching_water
        || !world.player.is_in_lava()
        || !mod_lava_swim::is_eligible(true, world.player.status.fire_resistance)
    {
        return ::core::ops::ControlFlow::Continue(());
    }
    let run = |world: &mut MotionWorld<'_, '_, '_>| -> crate::Result<()> {
        let mut gate = ::horizon_world::move_gate::MoveGate::new();
        // Player.travel's swim steering, as in player_travel.
        if world.player.swimming {
            let look_y = ::horizon_world::view_direction::view_vector(world.player.pitch, world.player.yaw).y;
            let near_surface = ::horizon_core::position::BlockPos::containing(Vec3::new(
                world.player.position.x,
                world.player.position.y + 1.0 - 0.1,
                world.player.position.z,
            ));
            if look_y <= 0.0 || world.player.jumping || world.fluid_at(near_surface).kind() != ::horizon_world::fluid::FluidKind::Empty {
                let blend = if look_y < -0.2 { 0.085 } else { 0.06 };
                world.player.velocity.y += (look_y - world.player.velocity.y) * blend;
            }
        }
        let sprinting = world.player.sprinting;
        let falling = <MotionWorld<'_, '_, '_> as ::horizon_world::living_air::LivingAirWorld>::velocity(world).y <= 0.0;
        let old_y = <MotionWorld<'_, '_, '_> as ::horizon_world::living_air::LivingAirWorld>::y(world);
        let gravity = <MotionWorld<'_, '_, '_> as ::horizon_world::living_air::LivingAirWorld>::effective_gravity(world)?;
        let before = <MotionWorld<'_, '_, '_> as ::horizon_world::living_air::LivingAirWorld>::velocity(world);
        ::horizon_world::living_air::move_relative(world, input, 0.02)?;
        let after = <MotionWorld<'_, '_, '_> as ::horizon_world::living_air::LivingAirWorld>::velocity(world);
        let (x, z) = mod_lava_swim::horizontal_boost((before.x, before.z), (after.x, after.z), sprinting);
        <MotionWorld<'_, '_, '_> as ::horizon_world::living_air::LivingAirWorld>::set_velocity(world, Vec3::new(x, after.y, z))?;
        let movement = <MotionWorld<'_, '_, '_> as ::horizon_world::living_air::LivingAirWorld>::velocity(world);
        <MotionWorld<'_, '_, '_> as ::horizon_world::living_air::LivingAirWorld>::move_self(world, &mut gate, movement)?;
        if <MotionWorld<'_, '_, '_> as ::horizon_world::living_air::LivingAirWorld>::horizontal_collision(world)
            && <MotionWorld<'_, '_, '_> as ::horizon_world::living_air::LivingAirWorld>::on_climbable(world)?
        {
            let v = <MotionWorld<'_, '_, '_> as ::horizon_world::living_air::LivingAirWorld>::velocity(world);
            <MotionWorld<'_, '_, '_> as ::horizon_world::living_air::LivingAirWorld>::set_velocity(
                world,
                Vec3::new(v.x, mod_lava_swim::WATER_CLIMB_VELOCITY_Y, v.z),
            )?;
        }
        let height = <MotionWorld<'_, '_, '_> as ::horizon_world::living_fluid::LivingFluidWorld>::fluid_height(
            world,
            ::horizon_world::living_fluid::TravelFluidTag::Lava,
        )?;
        let threshold = <MotionWorld<'_, '_, '_> as ::horizon_world::living_fluid::LivingFluidWorld>::fluid_jump_threshold(world)?;
        let v = <MotionWorld<'_, '_, '_> as ::horizon_world::living_air::LivingAirWorld>::velocity(world);
        let dragged = if height <= threshold {
            Vec3::new(v.x * 0.5, v.y * f64::from(0.8f32), v.z * 0.5)
        } else {
            Vec3::new(v.x * 0.5, v.y * mod_lava_swim::VANILLA_WATER_VERTICAL_DRAG, v.z * 0.5)
        };
        let adjusted = ::horizon_world::living_fluid::fluid_falling_adjusted_movement(world, gravity, falling, dragged)?;
        <MotionWorld<'_, '_, '_> as ::horizon_world::living_air::LivingAirWorld>::set_velocity(world, adjusted)?;
        // Lava's extra -gravity / 4 is skipped for eligible players.
        ::horizon_world::living_fluid::jump_out_of_fluid(world, old_y)?;
        ::horizon_world::living_fluid::float_in_liquid_while_ridden(
            world,
            ::horizon_world::living_fluid::TravelFluidTag::EntityFloatable,
        )?;
        ::core::result::Result::Ok(())
    };
    ::core::ops::ControlFlow::Break(run(world))
}
