use std::{
    collections::{HashSet, VecDeque},
    fmt::Debug,
    str::FromStr,
};

use crate::*;

const OCTOPI: usize = 100;

#[derive(PartialEq)]
pub struct OctoGrid([u8; OCTOPI]);

impl Debug for OctoGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for chunk in self.0.chunks(10) {
            for energy in chunk {
                write!(f, "{}", energy)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl FromStr for OctoGrid {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines()
            .flat_map(|s| s.chars())
            .map(|c| c.parse())
            .collect::<Result<Vec<_>, _>>()
            .and_then(|energies| {
                energies.try_into().map_err(|e: Vec<u8>| {
                    format!("Wrong number of octopi: {} not {}", e.len(), OCTOPI)
                })
            })
            .map(Self)
    }
}

impl OctoGrid {
    pub fn print_diff(&self, other: &OctoGrid) {
        for (c1, c2) in self.0.chunks(10).zip(other.0.chunks(10)) {
            for (a, b) in c1.iter().zip(c2.iter()) {
                print!("{:3} ", *a as i8 - *b as i8);
            }
            print!("\n");
        }
    }

    pub fn step(&mut self) -> usize {
        let mut explore = VecDeque::with_capacity(OCTOPI);
        let mut flashed = [false; OCTOPI];

        for (i, octopus) in self.0.iter_mut().enumerate() {
            *octopus += 1;
            if *octopus > 9 {
                explore.push_back(i);
                flashed[i] = true;
            }
        }

        while !explore.is_empty() {
            let to_flash = explore.pop_front().unwrap();
            for j in Self::neighbor_indices(to_flash) {
                self.0[j] += 1;
                if !flashed[j] && self.0[j] > 9 {
                    explore.push_back(j);
                    flashed[j] = true;
                }
            }
        }

        for (i, did_flash) in flashed.iter().enumerate() {
            if *did_flash {
                self.0[i] = 0;
            }
        }

        flashed.into_iter().filter(|flashed| *flashed).count()
    }

    fn neighbor_indices(i: usize) -> impl Iterator<Item = usize> {
        let x = i % 10;
        let is_left_edge = x == 0;
        let is_right_edge = x == 9;
        [
            i.checked_sub(11).filter(|_| !is_left_edge),
            i.checked_sub(10),
            i.checked_sub(9).filter(|_| !is_right_edge),
            i.checked_sub(1).filter(|_| !is_left_edge),
            Some(i + 1).filter(|_| !is_right_edge),
            Some(i + 9).filter(|_| !is_left_edge),
            Some(i + 10),
            Some(i + 11).filter(|_| !is_right_edge),
        ]
        .into_iter()
        .filter_map(|i| i.filter(|i| *i < OCTOPI))
    }
}

pub fn part_1(mut input: OctoGrid) -> usize {
    (0..100).map(|_| input.step()).sum()
}

pub fn part_2(mut input: OctoGrid) -> usize {
    for i in 0.. {
        if input.0.iter().all(|energy| *energy == 0) {
            return i;
        }
        input.step();
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> OctoGrid {
        "5483143223\n\
         2745854711\n\
         5264556173\n\
         6141336146\n\
         6357385478\n\
         4167524645\n\
         2176841721\n\
         6882881134\n\
         4846848554\n\
         5283751526"
            .parse()
            .unwrap()
    }

    fn input() -> OctoGrid {
        input!("day_11_dumbo_octopus").parse().unwrap()
    }

    #[test]
    fn test_part_1_sample_step() {
        let mut sample = sample();
        sample.step();
        assert_eq!(
            sample,
            "6594254334\n\
            3856965822\n\
            6375667284\n\
            7252447257\n\
            7468496589\n\
            5278635756\n\
            3287952832\n\
            7993992245\n\
            5957959665\n\
            6394862637"
                .parse()
                .unwrap()
        );
        sample.step();
        assert_eq!(
            sample,
            "8807476555\n\
            5089087054\n\
            8597889608\n\
            8485769600\n\
            8700908800\n\
            6600088989\n\
            6800005943\n\
            0000007456\n\
            9000000876\n\
            8700006848"
                .parse()
                .unwrap()
        );
    }

    #[test]
    fn test_part_1_sample() {
        assert_eq!(part_1(sample()), 1656);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(input()), 1705);
    }

    #[test]
    fn test_part_2_sample() {
        assert_eq!(part_2(sample()), 195);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(input()), 265);
    }
}
