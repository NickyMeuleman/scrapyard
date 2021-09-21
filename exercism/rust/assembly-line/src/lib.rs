pub fn production_rate_per_hour(speed: u8) -> f64 {
    let success_rate: f64 = match speed {
        0..=4 => 1.0,
        5..=8 => 0.9,
        9.. => 0.77,
    };
    speed as f64 * 221.0 * success_rate
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0).floor() as u32
}
