use std::collections::HashSet;
use std::collections::VecDeque;
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
    pub fn neighbor_coords(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut ret = Vec::with_capacity(4);
        if y > 0 {
            ret.push((x, y - 1));
        }
        if y < self.0[x].len() - 1 {
            ret.push((x, y + 1));
        }
        if x > 0 {
            ret.push((x - 1, y));
        }
        if x < self.0.len() - 1 {
            ret.push((x + 1, y));
        }
        ret
    }

    pub fn neighbors(&self, x: usize, y: usize) -> Vec<u8> {
        self.neighbor_coords(x, y)
            .into_iter()
            .map(|(x, y)| self.0[x][y])
            .collect()
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

    pub fn basin_size(&self, x: usize, y: usize) -> usize {
        // Breadth-first search to increasing neighbors.
        let mut to_explore = VecDeque::with_capacity(self.0.len());
        to_explore.push_back((x, y));
        let mut explored = HashSet::with_capacity(self.0.len());
        explored.insert((x, y));
        let mut basin_size = 0;
        while !to_explore.is_empty() {
            basin_size += 1;
            let (current_x, current_y) = to_explore.pop_front().unwrap();
            for neighbor @ (neighbor_x, neighbor_y) in self.neighbor_coords(current_x, current_y) {
                if !explored.contains(&neighbor) {
                    let neighbor_height = self.0[neighbor_x][neighbor_y];
                    if neighbor_height != 9 && neighbor_height >= self.0[current_x][current_y] {
                        // Neighbor is in the basin:
                        explored.insert(neighbor);
                        to_explore.push_back(neighbor);
                    }
                }
            }
        }
        basin_size
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
    heights
        .coords()
        .filter(|(x, y)| heights.is_low_point(*x, *y))
        .map(|(x, y)| heights.basin_size(x, y))
        .sorted()
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .take(3)
        .product()
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
        assert_eq!(part_2(sample()), 1134);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(input()), 1330560);
    }
}
