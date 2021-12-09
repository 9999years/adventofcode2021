use std::str::FromStr;

use crate::*;

pub struct HeightMap(Vec<Vec<u8>>);

impl FromStr for HeightMap {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
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
                            _ => Err(format!("Non-numeric char {}", c)),
                        })
                        .collect()
                })
                .collect::<Result<_, _>>()?,
        ))
    }
}

impl HeightMap {
    pub fn neighbors(&self, x: usize, y: usize) -> Vec<u8> {
        let mut ret = Vec::with_capacity(4);
        if y > 0 {
            ret.push(self.0[x][y - 1]);
        }
        if y < self.0[x].len() - 1 {
            ret.push(self.0[x][y + 1]);
        }
        if x > 0 {
            ret.push(self.0[x - 1][y]);
        }
        if x < self.0.len() - 1 {
            ret.push(self.0[x + 1][y]);
        }
        ret
    }

    pub fn is_low_point(&self, x: usize, y: usize) -> bool {
        let center_height = self.0[x][y];
        self.neighbors(x, y)
            .into_iter()
            .all(|height| center_height < height)
    }

    pub fn risk_level(&self, x: usize, y: usize) -> u8 {
        self.0[x][y] + 1
    }

    pub fn coords(&self) -> impl Iterator<Item = (usize, usize)> {
        (0..self.0.len()).cartesian_product(0..self.0[0].len())
    }
}

pub fn part_1(heights: HeightMap) -> usize {
    heights
        .coords()
        .map(|(x, y)| {
            if heights.is_low_point(x, y) {
                heights.risk_level(x, y) as usize
            } else {
                0
            }
        })
        .sum()
}

pub fn part_2(heights: HeightMap) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> HeightMap {
        "2199943210\n\
         3987894921\n\
         9856789892\n\
         8767896789\n\
         9899965678\n"
            .parse()
            .unwrap()
    }

    fn input() -> HeightMap {
        input!("day_9_smoke_basin").parse().unwrap()
    }

    #[test]
    fn test_part_1_sample() {
        assert_eq!(part_1(sample()), 15);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(input()), 502);
    }

    #[test]
    fn test_part_2_sample() {
        // assert_eq!(part_2(sample()), 0);
    }

    #[test]
    fn test_part_2() {
        // assert_eq!(part_2(input()), 0);
    }
}
