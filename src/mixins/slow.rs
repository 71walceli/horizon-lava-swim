fn lava_swim_slow(player: &crate::motion::PlayerMotion, value: bool) -> bool {
    let eligible = mod_lava_swim::is_eligible(true, player.status.fire_resistance);
    if value && !player.local_crouching {
        mod_lava_swim::should_slow_for_crawling(value, player.swimming, player.is_in_lava(), eligible)
    } else {
        value
    }
}
