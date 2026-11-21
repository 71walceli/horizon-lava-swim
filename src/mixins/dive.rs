fn lava_swim_dive(world: &mut MotionWorld<'_, '_, '_>, input: &TickInput) {
    if let TickInput::PreparedLocal { .. } = input {
        let eligible = mod_lava_swim::is_eligible(true, world.player.status.fire_resistance);
        let p = &mut *world.player;
        if mod_lava_swim::needs_extra_lava_dive(
            p.fluids.was_touching_water,
            p.is_in_lava(),
            eligible,
            p.shift_key_down,
            p.flying,
        ) {
            p.velocity.y += f64::from(-0.04f32);
        }
    }
}
