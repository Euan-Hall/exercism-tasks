// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
pub fn production_rate_per_hour(speed: u8) -> f64 {
    // My solution
    /*
    let items: f64 =  (221 * speed).into();
    if 1 <= speed && speed  <= 4 {items}
    else if 5 <= speed && speed <= 8 { items * 0.9 }
    else if 9 <= speed && speed <= 10 { items * 0.77 }
    else { 0.00 }*/

    // Better Solution
    221.0 * (speed as f64) * match speed {
        0 => 0.0,
        1..=4 => 1.0,
        5..=8 => 0.9,
        9..=u8::MAX => 0.77,
    }
    // EXPLANATION 
    // Match is like regex. We convert speed to f64 to satisfy the output
    // n..=M == n <= speed <= m.
}

pub fn working_items_per_minute(speed: u8) -> u32 { (production_rate_per_hour(speed) / 60.0) as u32 }
