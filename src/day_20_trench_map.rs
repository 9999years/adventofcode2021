use std::{collections::HashSet, fmt::Display, str::FromStr};

use crate::*;

use bitvec::prelude::*;

#[derive(Clone)]
pub struct TrenchScan {
    algorithm: BitVec,
    image: HashSet<(isize, isize)>,
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
                let c = if self.is_lit(&(x, y), 0) == 1 {
                    '#'
                } else {
                    '.'
                };
                write!(f, "{}", c)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl TrenchScan {
    #[inline]
    fn is_lit(&self, point @ (x, y): &(isize, isize), bit: usize) -> usize {
        if (self.oob_is_lit
            && (*x < self.x_min || *x > self.x_max || *y < self.y_min || *y > self.y_max))
            || self.image.contains(point)
        {
            1 << bit
        } else {
            0
        }
    }

    fn index(&self, (x, y): (isize, isize)) -> usize {
        self.is_lit(&(x + 1, y + 1), 0)
            + self.is_lit(&(x, y + 1), 1)
            + self.is_lit(&(x - 1, y + 1), 2)
            + self.is_lit(&(x + 1, y), 3)
            + self.is_lit(&(x, y), 4)
            + self.is_lit(&(x - 1, y), 5)
            + self.is_lit(&(x + 1, y - 1), 6)
            + self.is_lit(&(x, y - 1), 7)
            + self.is_lit(&(x - 1, y - 1), 8)
    }

    fn is_lit_next_step(&self, point: (isize, isize)) -> bool {
        self.algorithm[self.index(point)]
    }

    pub fn step(&self) -> Self {
        let mut ret = self.clone();
        ret.oob_is_lit = !ret.oob_is_lit && ret.algorithm[0];
        ret.image.clear();

        for x in self.x_min - 2..=self.x_max + 2 {
            for y in self.y_min - 2..=self.y_max + 2 {
                if self.is_lit_next_step((x, y)) {
                    ret.image.insert((x, y));
                    ret.x_min = ret.x_min.min(x);
                    ret.x_max = ret.x_max.max(x);
                    ret.y_min = ret.y_min.min(y);
                    ret.y_max = ret.y_max.max(y);
                }
            }
        }

        ret
    }

    pub fn pixels_lit(&self) -> usize {
        self.image.len()
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

        let mut image = HashSet::new();
        let mut x_min = isize::MAX;
        let mut x_max = isize::MIN;
        let mut y_min = isize::MAX;
        let mut y_max = isize::MIN;
        for (y, line) in lines.enumerate() {
            let y = y as isize;
            for (x, c) in line.chars().enumerate() {
                let x = x as isize;
                if is_lit(c)? {
                    image.insert((x, y));

                    x_min = x.min(x_min);
                    x_max = x.max(x_max);
                    y_min = y.min(y_min);
                    y_max = y.max(y_max);
                }
            }
        }
        Ok(Self {
            algorithm,
            image,
            x_min,
            x_max,
            y_min,
            y_max,
            oob_is_lit: false,
        })
    }
}

type Input = TrenchScan;

pub fn part_1(input: Input) -> usize {
    let input = input.step();
    let input = input.step();
    input.pixels_lit()
}

pub fn part_2(input: Input) -> usize {
    (0..50).fold(input, |input, _el| input.step()).pixels_lit()
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
        let s = sample();
        assert_eq!(s.pixels_lit(), 10);
        let s = s.step();
        assert_eq!(s.pixels_lit(), 24);
        let s = s.step();
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
