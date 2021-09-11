pub fn production_rate_per_hour(speed: u8) -> f64 {
    const LOW_SPEED: f64 = 1.00;
    const MEDIUM_SPEED: f64 = 0.90;
    const HIGH_SPEED: f64 = 0.77;
    const UNITS_PER_HOUR: f64= 221.0;
    match speed {
        0 => 0.0, // 'off' setting
        1 | 2 | 3 | 4 => speed as f64 * UNITS_PER_HOUR as f64 * LOW_SPEED,
        5 | 6 | 7 | 8 => speed as f64 * UNITS_PER_HOUR as f64 * MEDIUM_SPEED,
        9 | 10 => speed as f64 * UNITS_PER_HOUR * HIGH_SPEED,
        _ => panic!("Invalid speed setting."),
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    const MINUTES_PER_HOUR: f64 = 60.0;
    (production_rate_per_hour(speed) / MINUTES_PER_HOUR) as u32
}

