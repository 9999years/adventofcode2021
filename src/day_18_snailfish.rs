use std::borrow::Cow;
use std::fmt::Debug;
use std::ops::Add;
use std::str::FromStr;

use crate::*;

#[allow(unused_macros)]
macro_rules! snailfish {
    ([$a:tt, $b:tt]) => {
        Snailfish::Pair(Box::new(snailfish!($a)), Box::new(snailfish!($b)))
    };
    ($a:tt) => {
        Snailfish::Number($a)
    };
}

#[derive(Debug, PartialEq)]
enum Turn {
    Left,
    Right,
}

#[derive(Clone, PartialEq)]
pub enum Snailfish {
    Number(usize),
    Pair(Box<Snailfish>, Box<Snailfish>),
}

impl Debug for Snailfish {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{}", n),
            Self::Pair(left, right) => {
                write!(f, "[{:?},{:?}]", left, right)
            }
        }
    }
}

impl FromStr for Snailfish {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_snailfish_number(input: &str) -> Result<(Snailfish, &str), String> {
            if input.chars().next().ok_or_else(|| {
                "Expected non-empty string when parsing snailfish number".to_owned()
            })? == '['
            {
                parse_pair(input)
            } else {
                parse_regular_number(input)
            }
        }

        fn parse_pair(input: &str) -> Result<(Snailfish, &str), String> {
            let input = parse_open_bracket(input)?;
            let (left, input) = parse_snailfish_number(input)?;
            let input = parse_comma(input)?;
            let (right, input) = parse_snailfish_number(input)?;
            let input = parse_close_bracket(input)?;
            Ok((Snailfish::new_pair(left, right), input))
        }

        fn parse_comma(input: &str) -> Result<&str, String> {
            if input.starts_with(',') {
                Ok(&input[1..])
            } else {
                Err(format!(
                    "Failed to parse pair (no comma) starting at: {:?}",
                    input
                ))
            }
        }

        fn parse_open_bracket(input: &str) -> Result<&str, String> {
            if input.starts_with('[') {
                Ok(&input[1..])
            } else {
                Err(format!(
                    "Failed to parse pair (no open bracket) starting at: {:?}",
                    input
                ))
            }
        }

        fn parse_close_bracket(input: &str) -> Result<&str, String> {
            if input.starts_with(']') {
                Ok(&input[1..])
            } else {
                Err(format!(
                    "Failed to parse pair (no close bracket) at: {:?}",
                    input
                ))
            }
        }

        fn parse_regular_number(input: &str) -> Result<(Snailfish, &str), String> {
            let parse_len = input
                .char_indices()
                .take_while(|(_index, c)| c.is_ascii_digit())
                .map(|(index, c)| index + c.len_utf8())
                .last()
                .ok_or_else(|| format!("Failed to parse number starting at: {:?}", input))?;
            if parse_len == 0 {
                return Err(format!("Failed to find number starting at: {:?}", input));
            }
            let number_str = &input[..parse_len];
            let number = number_str
                .parse()
                .map_err(|err| format!("{}: {:?}", err, number_str))?;
            Ok((Snailfish::Number(number), &input[parse_len..]))
        }

        Ok(parse_snailfish_number(s)?.0)
    }
}

impl Snailfish {
    pub fn new_pair(left: Self, right: Self) -> Self {
        Self::Pair(Box::new(left), Box::new(right))
    }

    pub fn leftmost_number_mut(&mut self) -> &mut usize {
        match self {
            Snailfish::Number(n) => n,
            Snailfish::Pair(left, _) => left.leftmost_number_mut(),
        }
    }

    pub fn rightmost_number_mut(&mut self) -> &mut usize {
        match self {
            Snailfish::Number(n) => n,
            Snailfish::Pair(_, right) => right.rightmost_number_mut(),
        }
    }

    fn node_from_path_mut(&mut self, path: &[Turn]) -> &mut Self {
        path.into_iter().fold(self, |node, turn| match node {
            Snailfish::Number(_) => unreachable!(),
            Snailfish::Pair(a, b) => match turn {
                Turn::Left => a,
                Turn::Right => b,
            },
        })
    }

    fn next_prev_number_mut(&mut self, path: &[Turn], find: Turn) -> Option<&mut usize> {
        let idx = path.len() - 1 - path.into_iter().rev().position(|turn| *turn == find)?;
        let node = self.node_from_path_mut(&path[..idx]);
        match node {
            Snailfish::Number(_) => None,
            Snailfish::Pair(left, right) => Some(match find {
                Turn::Left => right.leftmost_number_mut(),
                Turn::Right => left.rightmost_number_mut(),
            }),
        }
    }

    fn prev_number_mut(&mut self, path: &[Turn]) -> Option<&mut usize> {
        self.next_prev_number_mut(path, Turn::Right)
    }

    fn next_number_mut(&mut self, path: &[Turn]) -> Option<&mut usize> {
        self.next_prev_number_mut(path, Turn::Left)
    }

    fn path_to_explode<'a>(&self, path: &'a mut Vec<Turn>) -> bool {
        match self {
            Snailfish::Number(_) => {
                path.pop();
                false
            }
            Snailfish::Pair(left, right) => match (&**left, &**right) {
                (Snailfish::Number(_), Snailfish::Number(_)) if path.len() >= 4 => true,
                _ => {
                    path.push(Turn::Left);
                    if left.path_to_explode(path) {
                        true
                    } else {
                        path.push(Turn::Right);
                        if right.path_to_explode(path) {
                            true
                        } else {
                            path.pop();
                            false
                        }
                    }
                }
            },
        }
    }

    fn find_explode_pair(&self) -> Option<Vec<Turn>> {
        let mut ret = Vec::new();
        if self.path_to_explode(&mut ret) {
            Some(ret)
        } else {
            None
        }
    }

    fn explode(&mut self, path: &[Turn]) {
        let pair = self.node_from_path_mut(path);
        let (left, right) = pair.as_regular_pair();
        *pair = Self::Number(0);
        if let Some(prev) = self.prev_number_mut(path) {
            *prev += left;
        }
        if let Some(next) = self.next_number_mut(path) {
            *next += right;
        }
    }

    fn as_regular_pair(&self) -> (usize, usize) {
        match self {
            Snailfish::Number(_) => panic!("Snailfish number is not a regular pair: {:?}", self),
            Snailfish::Pair(left, right) => match (&**left, &**right) {
                (Snailfish::Number(left), Snailfish::Number(right)) => (*left, *right),
                _ => panic!("Snailfish number is not a regular pair: {:?}", self),
            },
        }
    }

    fn split(&mut self) -> bool {
        match self {
            Snailfish::Number(n) if *n >= 10 => {
                let n = *n;
                *self = Self::new_pair(
                    Self::Number(n / 2),
                    Self::Number(if n % 2 == 1 { n / 2 + 1 } else { n / 2 }),
                );
                true
            }
            Snailfish::Pair(left, right) => left.split() || right.split(),
            _ => false,
        }
    }

    fn reduce_once(&mut self) -> bool {
        if let Some(path) = self.find_explode_pair() {
            self.explode(&path);
            true
        } else {
            self.split()
        }
    }

    pub fn reduce(&mut self) {
        while self.reduce_once() {}
    }

    pub fn magnitude(&self) -> usize {
        match self {
            Snailfish::Number(n) => *n,
            Snailfish::Pair(left, right) => 3 * left.magnitude() + 2 * right.magnitude(),
        }
    }
}

impl Add for Snailfish {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut ret = Snailfish::new_pair(self, rhs);
        ret.reduce();
        ret
    }
}

type Input = Vec<Snailfish>;

pub fn part_1(input: Input) -> usize {
    input.into_iter().fold1(|a, b| a + b).unwrap().magnitude()
}

pub fn part_2(input: Input) -> usize {
    (&input)
        .into_iter()
        .cartesian_product(&input)
        .map(|(a, b)| ((*a).clone() + (*b).clone()).magnitude())
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Input {
        vec![
            snailfish!([[[0, [5, 8]], [[1, 7], [9, 6]]], [[4, [1, 2]], [[1, 4], 2]]]),
            snailfish!([[[5, [2, 8]], 4], [5, [[9, 9], 0]]]),
            snailfish!([6, [[[6, 2], [5, 6]], [[7, 6], [4, 7]]]]),
            snailfish!([[[6, [0, 7]], [0, 9]], [4, [9, [9, 0]]]]),
            snailfish!([[[7, [6, 4]], [3, [1, 3]]], [[[5, 5], 1], 9]]),
            snailfish!([[6, [[7, 3], [3, 2]]], [[[3, 8], [5, 7]], 4]]),
            snailfish!([[[[5, 4], [7, 7]], 8], [[8, 3], 8]]),
            snailfish!([[9, 3], [[9, 9], [6, [4, 9]]]]),
            snailfish!([[2, [[7, 7], 7]], [[5, 8], [[9, 3], [0, 2]]]]),
            snailfish!([[[[5, 2], 5], [8, [3, 7]]], [[5, [7, 5]], [4, 4]]]),
        ]
    }

    fn input() -> Input {
        input!("day_18_snailfish", Snailfish).collect()
    }

    #[test]
    fn test_part_1_sample() {
        assert_eq!(part_1(sample()), 4140);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(input()), 4145);
    }

    #[test]
    fn test_part_2_sample() {
        assert_eq!(part_2(sample()), 3993);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(input()), 4855);
    }
}
