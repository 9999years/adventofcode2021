use std::{collections::HashSet, fmt::Display, str::FromStr};

use crate::*;

use bitvec::prelude::*;

#[derive(Clone)]
pub struct TrenchScan {
    algorithm: BitVec,
    image: BitVec,
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize,
    oob_is_lit: bool,
}

impl Display for TrenchScan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in self.y_min - 2..=self.y_max + 2 {
            for x in self.x_min - 2..=self.x_max + 2 {
                let c = if self.is_lit(&(x, y)) { '#' } else { '.' };
                write!(f, "{}", c)?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl TrenchScan {
    #[inline]
    fn width(&self) -> isize {
        self.x_max - self.x_min + 1
    }

    #[inline]
    fn height(&self) -> isize {
        self.y_max - self.y_min + 1
    }

    #[inline]
    fn is_lit(&self, (x, y): &(isize, isize)) -> bool {
        if *x < self.x_min || *x > self.x_max || *y < self.y_min || *y > self.y_max {
            // Out of bounds.
            self.oob_is_lit
        } else if self.image[((y - self.y_min) * self.width() + x - self.x_min) as usize] {
            true
        } else {
            false
        }
    }

    #[inline]
    fn is_lit_bit(&self, point: &(isize, isize), bit: usize) -> usize {
        if self.is_lit(point) {
            1 << bit
        } else {
            0
        }
    }

    fn index(&self, (x, y): (isize, isize)) -> usize {
        self.is_lit_bit(&(x + 1, y + 1), 0)
            + self.is_lit_bit(&(x, y + 1), 1)
            + self.is_lit_bit(&(x - 1, y + 1), 2)
            + self.is_lit_bit(&(x + 1, y), 3)
            + self.is_lit_bit(&(x, y), 4)
            + self.is_lit_bit(&(x - 1, y), 5)
            + self.is_lit_bit(&(x + 1, y - 1), 6)
            + self.is_lit_bit(&(x, y - 1), 7)
            + self.is_lit_bit(&(x - 1, y - 1), 8)
    }

    fn is_lit_next_step(&self, point: (isize, isize)) -> bool {
        self.algorithm[self.index(point)]
    }

    pub fn step(&mut self) {
        self.image = {
            let mut image =
                BitVec::with_capacity(((self.width() + 4) * (self.height() + 4)) as usize);
            for y in self.y_min - 2..=self.y_max + 2 {
                for x in self.x_min - 2..=self.x_max + 2 {
                    image.push(self.is_lit_next_step((x, y)));
                }
            }
            image
        };
        self.x_min -= 2;
        self.x_max += 2;
        self.y_min -= 2;
        self.y_max += 2;
        self.oob_is_lit = !self.oob_is_lit && self.algorithm[0];
    }

    pub fn pixels_lit(&self) -> usize {
        self.image.count_ones()
    }
}

impl FromStr for TrenchScan {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let algorithm_line = lines.next().ok_or_else(|| format!("Missing first line"))?;

        // Skip blank line between algorithm and image.
        lines.next();

        let is_lit = |c: char| match c {
            '#' => Ok(true),
            '.' => Ok(false),
            _ => Err(format!("Unexpected character in image: {:?}", c)),
        };

        let mut algorithm = BitVec::with_capacity(algorithm_line.len());
        for c in algorithm_line.chars() {
            algorithm.push(is_lit(c)?);
        }

        let mut image = BitVec::new();
        let mut x_max = isize::MIN;
        let mut y_max = isize::MIN;
        for (y, line) in lines.enumerate() {
            let y = y as isize;
            y_max = y.max(y_max);
            for (x, c) in line.chars().enumerate() {
                let x = x as isize;
                image.push(is_lit(c)?);
                x_max = x.max(x_max);
            }
        }
        Ok(Self {
            algorithm,
            image,
            x_min: 0,
            x_max,
            y_min: 0,
            y_max,
            oob_is_lit: false,
        })
    }
}

type Input = TrenchScan;

pub fn part_1(mut input: Input) -> usize {
    input.step();
    input.step();
    input.pixels_lit()
}

pub fn part_2(mut input: Input) -> usize {
    for _ in 0..50 {
        input.step();
    }
    input.pixels_lit()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Input {
        "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##\
        #..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###\
        .######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.\
        .#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....\
        .#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..\
        ...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....\
        ..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#\n\
        \n\
        #..#.\n\
        #....\n\
        ##..#\n\
        ..#..\n\
        ..###"
            .parse()
            .unwrap()
    }

    fn input() -> Input {
        input!("day_20_trench_map").parse().unwrap()
    }

    #[test]
    fn test_part_1_sample() {
        let mut s = sample();
        assert_eq!(s.pixels_lit(), 10);
        s.step();
        assert_eq!(s.pixels_lit(), 24);
        s.step();
        assert_eq!(s.pixels_lit(), 35);
        assert_eq!(part_1(sample()), 35);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(input()), 5065);
    }

    #[test]
    fn test_part_2_sample() {
        assert_eq!(part_2(sample()), 3351);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(input()), 14790);
    }
}
