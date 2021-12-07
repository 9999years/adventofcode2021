#![allow(unused_imports)]

pub mod day_1_sonar_sweep;
pub mod day_2_dive;
pub mod day_3_binary_diagnostic;
pub mod day_4_giant_squid;
pub mod day_5_hydrothermal_venture;
pub mod day_6_lanternfish;
pub mod day_7_the_treachery_of_whales;

pub(crate) use itertools::Itertools;
pub(crate) use tap::{Conv, TryConv};

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
