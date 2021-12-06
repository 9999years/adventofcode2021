use std::{collections::HashMap, ops::RangeInclusive, str::FromStr};

use crate::*;

#[derive(Clone, Copy, Debug)]
pub struct LineSegment {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

impl LineSegment {
    pub fn is_horizontal(&self) -> bool {
        self.y1 == self.y2
    }

    pub fn is_vertical(&self) -> bool {
        self.x1 == self.x2
    }

    fn x_values(&self) -> Vec<usize> {
        // This is not efficient but writing an "either range or range-reverse"
        // iterator would be effort sooo
        if self.x2 < self.x1 {
            (self.x2..=self.x1).rev().collect()
        } else {
            (self.x1..=self.x2).collect()
        }
    }

    fn y_values(&self) -> Vec<usize> {
        if self.y2 < self.y1 {
            (self.y2..=self.y1).rev().collect()
        } else {
            (self.y1..=self.y2).collect()
        }
    }

    pub fn points(&self) -> Vec<(usize, usize)> {
        if self.is_vertical() {
            std::iter::repeat(self.x1).zip(self.y_values()).collect()
        } else if self.is_horizontal() {
            self.x_values()
                .into_iter()
                .zip(std::iter::repeat(self.y1))
                .collect()
        } else {
            self.x_values()
                .into_iter()
                .zip(self.y_values().into_iter())
                .collect()
        }
    }
}

impl FromStr for LineSegment {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s
            .split_once(" -> ")
            .ok_or_else(|| "Segment must contain ' -> '")?;
        let (x1, y1) = start
            .split_once(',')
            .ok_or_else(|| "Coordinates must be delimited by ','")?;
        let (x2, y2) = end
            .split_once(',')
            .ok_or_else(|| "Coordinates must be delimited by ','")?;
        let from_str = |s: &str| {
            s.parse::<usize>()
                .map_err(|err| format!("{}: {:?}", err, s))
        };
        Ok(LineSegment {
            x1: from_str(x1)?,
            y1: from_str(y1)?,
            x2: from_str(x2)?,
            y2: from_str(y2)?,
        })
    }
}

fn get_overlapping_points(segments: impl Iterator<Item = LineSegment>) -> usize {
    let mut point_counts = HashMap::<(usize, usize), usize>::new();
    for segment in segments {
        for point in segment.points() {
            let count = point_counts.entry(point).or_default();
            *count += 1;
        }
    }
    point_counts
        .into_values()
        .filter(|count| *count > 1)
        .count()
}

pub fn part_1(segments: impl Iterator<Item = LineSegment>) -> usize {
    get_overlapping_points(
        segments.filter(|segment| segment.is_horizontal() || segment.is_vertical()),
    )
}

pub fn part_2(segments: impl Iterator<Item = LineSegment>) -> usize {
    get_overlapping_points(segments)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn sample() -> Vec<LineSegment> {
        [
            "0,9 -> 5,9",
            "8,0 -> 0,8",
            "9,4 -> 3,4",
            "2,2 -> 2,1",
            "7,0 -> 7,4",
            "6,4 -> 2,0",
            "0,9 -> 2,9",
            "3,4 -> 1,4",
            "0,0 -> 8,8",
            "5,5 -> 8,2",
        ]
        .into_iter()
        .map(|line| line.parse().unwrap())
        .collect()
    }

    #[test]
    fn test_part_1_sample() {
        assert_eq!(part_1(sample().into_iter()), 5);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(input!("day_5_hydrothermal_venture", LineSegment)),
            7269
        );
    }

    #[test]
    fn test_part_2_sample() {
        assert_eq!(part_2(sample().into_iter()), 12);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(input!("day_5_hydrothermal_venture", LineSegment)),
            21140
        );
    }
}
