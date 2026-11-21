fn lava_swim_sprint(player: &PlayerMotion, value: crate::sprint::Conditions) -> crate::sprint::Conditions {
    let eligible = mod_lava_swim::is_eligible(true, player.status.fire_resistance);
    let in_lava = player.is_in_lava();
    let mut conditions = value;
    conditions.in_water =
        mod_lava_swim::is_touching_equivalent_swimming_fluid(value.in_water, value.swimming, in_lava, eligible);
    conditions.shallow_water = mod_lava_swim::is_partly_touching_equivalent_fluid(
        value.shallow_water,
        in_lava,
        player.is_eye_in_fluid(crate::motion::LAVA),
        eligible,
    );
    conditions
}
