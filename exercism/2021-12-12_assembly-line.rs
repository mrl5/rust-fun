// https://github.com/exercism/rust/tree/630a56517c9015341cdd96233632eda9ad8f44c4/exercises/concept/assembly-line
// https://exercism.org/tracks/rust/exercises/assembly-line/solutions/mrl5

#![feature(exclusive_range_pattern)]

const WORKER_CAPACITY_PER_HOUR: u8 = 221;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    speed as f64 * WORKER_CAPACITY_PER_HOUR as f64 * get_success_rate(speed)
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}

fn get_success_rate(speed: u8) -> f64 {
    match speed {
        0 => 0.0,
        1 .. 5 => 1.0,
        5 .. 9 => 0.9,
        9 ..= 10 => 0.77,
        _ => panic!(),
    }
}
