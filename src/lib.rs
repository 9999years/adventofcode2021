pub mod day_1_sonar_sweep;

pub use itertools::Itertools;

// Borrowed this macro from iliana:
// https://github.com/iliana/aoc2021/blob/d5d7eb7336b9078081a9f7a44ce7ebb6dce374f4/src/lib.rs
#[macro_export]
macro_rules! input {
    ($day:expr) => {{
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/", $day, ".txt",))
    }};

    ($day:expr, $ty:ty) => {{
        use std::str::FromStr;
        input!($day)
            .lines()
            .map(|line| <$ty>::from_str(line).unwrap())
    }};

    ($day:expr, $ty:ty, $split:expr) => {
        input!($day)
            .split($split)
            .map(|line| <$ty>::from_str(line).unwrap())
    };
}
