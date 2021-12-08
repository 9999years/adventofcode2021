use std::collections::HashSet;
use std::str::FromStr;

use crate::*;

#[derive(Debug)]
pub struct Display {
    signals: [HashSet<Segment>; 10],
    output: [HashSet<Segment>; 4],
}

impl FromStr for Display {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (signals, output) = s
            .split_once(" | ")
            .ok_or_else(|| format!("No '|' in display: {:?}", s))?;
        Ok(Self {
            signals: signals
                .split_ascii_whitespace()
                .map(parse_segments)
                .collect::<Result<Vec<_>, _>>()?
                .try_into()
                .map_err(|err: Vec<_>| format!("Expected 10 digits, got {}", err.len()))?,
            output: output
                .split_ascii_whitespace()
                .map(parse_segments)
                .collect::<Result<Vec<_>, _>>()?
                .try_into()
                .map_err(|err: Vec<_>| format!("Expected 4 digits, got {}", err.len()))?,
        })
    }
}

fn parse_segments(segments: &str) -> Result<HashSet<Segment>, String> {
    segments.chars().map(|segment| segment.try_into()).collect()
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Segment {
    Top,
    TopLeft,
    TopRight,
    Middle,
    BottomLeft,
    BottomRight,
    Bottom,
}

impl TryFrom<char> for Segment {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'a' => Ok(Segment::Top),
            'b' => Ok(Segment::TopLeft),
            'c' => Ok(Segment::TopRight),
            'd' => Ok(Segment::Middle),
            'e' => Ok(Segment::BottomLeft),
            'f' => Ok(Segment::BottomRight),
            'g' => Ok(Segment::Bottom),
            _ => Err(format!(
                "Unknown segment {}; expected one of 'a'-'f'",
                value
            )),
        }
    }
}

pub fn part_1(displays: impl Iterator<Item = Display>) -> usize {
    displays
        .flat_map(|display| display.output.into_iter())
        .filter(|digit| match digit.len() {
            2 => true, // 1 = cf
            4 => true, // 4 = bcdf
            3 => true, // 7 = acf
            7 => true, // 8 = abcdefg
            _ => false,
        })
        .count()
}

pub fn part_2() -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> impl Iterator<Item = Display> {
        [
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
            "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc",
            "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg",
            "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb",
            "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea",
            "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb",
            "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe",
            "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef",
            "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb",
            "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce",
        ].into_iter().map(|line| line.parse().unwrap())
    }

    fn input() -> impl Iterator<Item = Display> {
        input!("day_8_seven_segment_search", Display)
    }

    #[test]
    fn test_part_1_sample() {
        assert_eq!(part_1(sample()), 26);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(input()), 383);
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
