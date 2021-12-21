use std::collections::HashSet;
use std::str::FromStr;

use crate::*;

pub struct Probe {
    x: isize,
    y: isize,
    x_velocity: isize,
    y_velocity: isize,
}

impl Probe {
    pub fn new(x_velocity: isize, y_velocity: isize) -> Self {
        Self {
            x: 0,
            y: 0,
            x_velocity,
            y_velocity,
        }
    }

    pub fn step(&mut self) {
        self.x += self.x_velocity;
        self.y += self.y_velocity;
        // Drag:
        self.x_velocity -= self.x_velocity.signum();
        // Gravity:
        self.y_velocity -= 1;
    }

    pub fn in_target_area(&self, target_area: &TargetArea) -> bool {
        self.x >= target_area.x_min
            && self.x <= target_area.x_max
            && self.y >= target_area.y_min
            && self.y <= target_area.y_max
    }

    pub fn might_hit_target_area(&self, target_area: &TargetArea) -> bool {
        if self.y_velocity <= 0 {
            // Going down
            self.y >= target_area.y_min // Haven't overshot bottom of target
        } else if self.x_velocity >= 0 {
            // Going right
            self.x <= target_area.x_max // Haven't overshot right of target
        } else {
            // Going left
            self.x >= target_area.x_min // Haven't overshot left of target
        }
    }
}

pub struct TargetArea {
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize,
}

impl FromStr for TargetArea {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let start = "target area: x=";
        if s.starts_with(start) {
            let s = &s[start.len()..];
            let (x_range, y_range) = s
                .split_once(", y=")
                .ok_or_else(|| format!("Expected to find ', y=' in {:?}", s))?;
            let (x_min, x_max) = x_range
                .split_once("..")
                .ok_or_else(|| format!("Expected coords delimited by '..': {:?}", x_range))?;
            let (y_min, y_max) = y_range
                .split_once("..")
                .ok_or_else(|| format!("Expected coords delimited by '..': {:?}", y_range))?;
            let parse = |num: &str| {
                num.parse::<isize>()
                    .map_err(|err| format!("{}: {:?}", err, num))
            };
            Ok(Self {
                x_min: parse(x_min)?,
                x_max: parse(x_max)?,
                y_min: parse(y_min)?,
                y_max: parse(y_max)?,
            })
        } else {
            Err(format!("Expected {:?} to start with {:?}", s, start))
        }
    }
}

pub fn hits_target_area(
    x_velocity: isize,
    y_velocity: isize,
    target_area: &TargetArea,
) -> Option<isize> {
    let mut max_height = 0;
    let mut probe = Probe::new(x_velocity, y_velocity);
    while !probe.in_target_area(target_area) && probe.might_hit_target_area(target_area) {
        probe.step();
        max_height = max_height.max(probe.y);
    }
    Some(max_height).filter(|_| probe.in_target_area(target_area))
}

fn velocity_pairs(target_area: &TargetArea) -> impl Iterator<Item = (isize, isize)> {
    (0..(target_area.x_max + 1)).cartesian_product(target_area.y_min..-target_area.y_min)
}

type Input = TargetArea;

pub fn part_1(input: Input) -> isize {
    let mut max_height = 0;
    for (x_velocity, y_velocity) in velocity_pairs(&input) {
        if let Some(height) = hits_target_area(x_velocity, y_velocity, &input) {
            max_height = max_height.max(height);
        }
    }
    max_height
}

pub fn part_2(input: Input) -> usize {
    let mut distinct_pairs = HashSet::<(isize, isize)>::new();
    for (x_velocity, y_velocity) in velocity_pairs(&input) {
        if hits_target_area(x_velocity, y_velocity, &input).is_some() {
            distinct_pairs.insert((x_velocity, y_velocity));
        }
    }
    distinct_pairs.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Input {
        TargetArea {
            x_min: 20,
            x_max: 30,
            y_min: -10,
            y_max: -5,
        }
    }

    fn input() -> Input {
        input!("day_17_trick_shot").parse().unwrap()
    }

    #[test]
    fn test_part_1_sample() {
        assert_eq!(part_1(sample()), 45);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(input()), 4095);
    }

    #[test]
    fn test_part_2_sample() {
        assert_eq!(part_2(sample()), 112);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(input()), 3773);
    }
}
