// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

const NUM_OF_CARS_PER_HOUR: u32 = 221;
pub fn production_rate_per_hour(speed: u8) -> f64 {
    let optimistic_produce: u32 = speed as u32 * NUM_OF_CARS_PER_HOUR;
    match speed {
        0 => {
            0.0f64
        },
        1..=4 => {
            optimistic_produce as f64
        },
        5..=8 => {
            ( 90f64 / 100f64) * optimistic_produce as f64
        },
        9..=10 => {
            (77f64 / 100f64) * optimistic_produce as f64
        },
        11_u8..=u8::MAX => todo!()
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let produced_per_hour = production_rate_per_hour(speed);
    (produced_per_hour / 60f64).floor() as u32
}
