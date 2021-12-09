use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;

use crate::*;

#[derive(Debug)]
pub struct Display {
    digits: [HashSet<Segment>; 10],
    output: [HashSet<Segment>; 4],
}

impl Display {
    fn segment_counts(&self) -> HashMap<Segment, usize> {
        let mut segment_counts = HashMap::<Segment, usize>::with_capacity(7);
        for digit in &self.digits {
            for segment in digit {
                let count = segment_counts.entry(*segment).or_default();
                *count += 1;
            }
        }
        segment_counts
    }

    fn decode(&self) -> HashMap<Segment, Segment> {
        let segment_counts = self.segment_counts();

        // If a mapping `i -> o` is in `inv_swaps`, it means that when segment
        // `i` is input to the display, segment `o` is lit up.
        let mut inv_swaps = HashMap::<Segment, Segment>::with_capacity(7);

        let mut candidates_ac = Vec::<Segment>::with_capacity(2);
        let mut candidates_dg = Vec::<Segment>::with_capacity(2);

        for (segment, count) in segment_counts.iter() {
            match *count {
                4 => {
                    inv_swaps.insert(Segment::E, *segment);
                }
                6 => {
                    inv_swaps.insert(Segment::B, *segment);
                }
                7 => {
                    candidates_dg.push(*segment);
                }
                8 => {
                    candidates_ac.push(*segment);
                }
                9 => {
                    inv_swaps.insert(Segment::F, *segment);
                }
                _ => {
                    unreachable!();
                }
            }
        }

        let digit_one = self.digits.iter().find(|digit| digit.len() == 2).unwrap();
        let digit_four = self.digits.iter().find(|digit| digit.len() == 4).unwrap();

        // The 2 segments in `digit` are C and F.
        // We know F; C is the other one.
        let f = *inv_swaps.get(&Segment::F).unwrap();

        for segment in digit_one {
            if *segment != f {
                let c = *segment;
                inv_swaps.insert(Segment::C, c);

                // Now we know C, which lets us determine A.
                for candidate in &candidates_ac {
                    if *candidate != c {
                        inv_swaps.insert(Segment::A, *candidate);
                        break;
                    }
                }
                break;
            }
        }

        // The 4 segments in `digit` are B, C, D, and F.
        // We know B, C, and F; D is the other one.
        let b = *inv_swaps.get(&Segment::B).unwrap();
        let c = *inv_swaps.get(&Segment::C).unwrap();

        for segment in digit_four {
            if *segment != b && *segment != c && *segment != f {
                let d = *segment;
                inv_swaps.insert(Segment::D, d);

                // Now we know D, which lets us determine G.
                for candidate in &candidates_dg {
                    if *candidate != d {
                        inv_swaps.insert(Segment::G, *candidate);
                        break;
                    }
                }
                break;
            }
        }

        inv_swaps.into_iter().map(|(o, i)| (i, o)).collect()
    }

    pub fn output_value(&self) -> usize {
        let decode_map = self.decode();
        1000 * decode_digit(&decode_map, &self.output[0])
            + 100 * decode_digit(&decode_map, &self.output[1])
            + 10 * decode_digit(&decode_map, &self.output[2])
            + decode_digit(&decode_map, &self.output[3])
    }
}

fn decode_digit(map: &HashMap<Segment, Segment>, digit: &HashSet<Segment>) -> usize {
    let decoded_segments: Vec<_> = digit
        .into_iter()
        .map(|segment| map.get(&segment).unwrap())
        .sorted()
        .collect();
    use Segment::*;
    match &decoded_segments[..] {
        &[A, B, C, E, F, G] => 0,
        &[C, F] => 1,
        &[A, C, D, E, G] => 2,
        &[A, C, D, F, G] => 3,
        &[B, C, D, F] => 4,
        &[A, B, D, F, G] => 5,
        &[A, B, D, E, F, G] => 6,
        &[A, C, F] => 7,
        &[A, B, C, D, E, F, G] => 8,
        &[A, B, C, D, F, G] => 9,
        _ => unreachable!(),
    }
}

impl FromStr for Display {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (signals, output) = s
            .split_once(" | ")
            .ok_or_else(|| format!("No '|' in display: {:?}", s))?;
        Ok(Self {
            digits: signals
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Segment {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl TryFrom<char> for Segment {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'a' => Ok(Segment::A),
            'b' => Ok(Segment::B),
            'c' => Ok(Segment::C),
            'd' => Ok(Segment::D),
            'e' => Ok(Segment::E),
            'f' => Ok(Segment::F),
            'g' => Ok(Segment::G),
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

pub fn part_2(displays: impl Iterator<Item = Display>) -> usize {
    displays.map(|display| display.output_value()).sum()
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
        assert_eq!(part_2(sample()), 61229);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(input()), 998900);
    }
}
