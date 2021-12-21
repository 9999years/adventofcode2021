#![allow(unused_imports)]

pub mod day_10_syntax_scoring;
pub mod day_11_dumbo_octopus;
pub mod day_12_passage_pathing;
pub mod day_13_transparent_origami;
pub mod day_14_extended_polymerization;
pub mod day_15_chiton;
pub mod day_16_packet_decoder;
pub mod day_17_trick_shot;
pub mod day_18_snailfish;
pub mod day_1_sonar_sweep;
pub mod day_2_dive;
pub mod day_3_binary_diagnostic;
pub mod day_4_giant_squid;
pub mod day_5_hydrothermal_venture;
pub mod day_6_lanternfish;
pub mod day_7_the_treachery_of_whales;
pub mod day_8_seven_segment_search;
pub mod day_9_smoke_basin;

pub(crate) use itertools::Itertools;
pub(crate) use tap::{Conv, TryConv};

pub(crate) trait Parsable<T> {
    type Err;
    fn parse(self) -> Result<T, Self::Err>;
}

impl Parsable<u8> for char {
    type Err = String;

    fn parse(self) -> Result<u8, Self::Err> {
        match self {
            '0' => Ok(0),
            '1' => Ok(1),
            '2' => Ok(2),
            '3' => Ok(3),
            '4' => Ok(4),
            '5' => Ok(5),
            '6' => Ok(6),
            '7' => Ok(7),
            '8' => Ok(8),
            '9' => Ok(9),
            'A' => Ok(10),
            'B' => Ok(11),
            'C' => Ok(12),
            'D' => Ok(13),
            'E' => Ok(14),
            'F' => Ok(15),
            _ => Err(format!("Cannot parse char {:?} as integer", self)),
        }
    }
}

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
