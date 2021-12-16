use std::{cell::RefCell, collections::BinaryHeap, str::FromStr};

use crate::*;

pub struct RiskMap(Vec<Vec<u8>>);

impl FromStr for RiskMap {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .map(|line| {
                    line.chars()
                        .map(|c| c.parse())
                        .collect::<Result<Vec<_>, _>>()
                })
                .collect::<Result<_, _>>()?,
        ))
    }
}

impl RiskMap {
    pub fn expanded(&self) -> Self {
        const FACTOR: usize = 5;

        let (width, height) = (self.width(), self.height());

        let mut ret = vec![vec![0; width * FACTOR]; height * FACTOR];

        for y_tile in 0..FACTOR {
            for x_tile in 0..FACTOR {
                for x in 0..width {
                    for y in 0..height {
                        ret[height * y_tile + y][width * x_tile + x] = {
                            let risk = (self.0[y][x] + x_tile as u8 + y_tile as u8) % 9;
                            match risk {
                                0 => 9,
                                _ => risk,
                            }
                        };
                    }
                }
            }
        }

        Self(ret)
    }

    pub fn shortest_path(&self) -> usize {
        // Dijkstra's algorithm
        // https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm

        // This implementation is basically copied (by hand, at least...) from
        // https://doc.rust-lang.org/std/collections/binary_heap/index.html

        #[derive(Clone, Copy, PartialEq, Eq)]
        struct State {
            distance: usize,
            index: (usize, usize),
        }

        impl Ord for State {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                // Reversed ordering to make a min-heap.
                other
                    .distance
                    .cmp(&self.distance)
                    // Tie-breaker for consistency with Eq.
                    .then_with(|| self.index.cmp(&other.index))
            }
        }

        impl PartialOrd for State {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl State {
            fn is_goal(&self, risk_map: &RiskMap) -> bool {
                self.index == (risk_map.width() - 1, risk_map.height() - 1)
            }
        }

        let mut distances = vec![vec![None; self.width()]; self.height()];
        let mut heap = BinaryHeap::new();
        distances[0][0] = Some(0);
        heap.push(State {
            distance: 0,
            index: (0, 0),
        });

        while let Some(state) = heap.pop() {
            if state.is_goal(&self) {
                return state.distance;
            }

            if distances[state.index.1][state.index.0]
                .map(|d| state.distance > d)
                .unwrap_or(false)
            {
                continue;
            }

            for neighbor in self.neighbor_indices(state.index) {
                let neighbor_cost = self.0[neighbor.1][neighbor.0];
                let next = State {
                    distance: state.distance + neighbor_cost as usize,
                    index: neighbor,
                };

                let prev_distance = distances
                    .get_mut(next.index.1)
                    .unwrap()
                    .get_mut(next.index.0)
                    .unwrap();
                if prev_distance.map(|d| next.distance < d).unwrap_or(true) {
                    heap.push(next);
                    *prev_distance = Some(next.distance);
                }
            }
        }

        unreachable!()
    }

    fn height(&self) -> usize {
        self.0.len()
    }

    fn width(&self) -> usize {
        self.0[0].len()
    }

    fn neighbor_indices(&self, (x, y): (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
        let rows = self.height();
        let cols = self.width();
        [
            (x.checked_sub(1), Some(y)),
            (Some(x + 1), Some(y)),
            (Some(x), y.checked_sub(1)),
            (Some(x), Some(y + 1)),
        ]
        .into_iter()
        .filter_map(move |index| match index {
            (Some(x), Some(y)) if x < cols && y < rows => Some((x, y)),
            _ => None,
        })
    }
}

type Input = RiskMap;

pub fn part_1(input: Input) -> usize {
    input.shortest_path()
}

pub fn part_2(input: Input) -> usize {
    input.expanded().shortest_path()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Input {
        "1163751742\n\
        1381373672\n\
        2136511328\n\
        3694931569\n\
        7463417111\n\
        1319128137\n\
        1359912421\n\
        3125421639\n\
        1293138521\n\
        2311944581"
            .parse()
            .unwrap()
    }

    fn input() -> Input {
        input!("day_15_chiton").parse().unwrap()
    }

    #[test]
    fn test_part_1_sample() {
        assert_eq!(part_1(sample()), 40);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(input()), 458);
    }

    #[test]
    fn test_part_2_sample() {
        assert_eq!(part_2(sample()), 315);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(input()), 2800);
    }
}
