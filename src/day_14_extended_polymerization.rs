use std::collections::HashMap;
use std::str::FromStr;

use crate::*;

pub struct Polymer {
    first: char,
    last: char,
    pairs: HashMap<(char, char), usize>,
    rules: HashMap<(char, char), char>,
}

fn parse_pairs(pairs: &str) -> HashMap<(char, char), usize> {
    pairs
        .chars()
        .tuple_windows()
        .fold(HashMap::new(), |mut map, pair| {
            let entry = map.entry(pair).or_default();
            *entry += 1;
            map
        })
}

impl FromStr for Polymer {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let pairs = lines.next().ok_or_else(|| format!("Expected first line"))?;
        let first = pairs
            .chars()
            .next()
            .ok_or_else(|| format!("Expected template to have a first and last char"))?;
        let last = pairs
            .chars()
            .last()
            .ok_or_else(|| format!("Expected template to have a first and last char"))?;
        let pairs = parse_pairs(pairs);

        lines.next(); // Discard empty line.

        fn parse_rule(rule: &str) -> Result<((char, char), char), String> {
            let (pair, insert) = rule.split_once(" -> ").ok_or_else(|| {
                format!("Expected rule to be delimited by ' -> ', got: {:?}", rule)
            })?;
            let mut pair_chars = pair.chars();
            Ok((
                (
                    pair_chars
                        .next()
                        .ok_or_else(|| format!("Expected pair to have two chars: {:?}", pair))?,
                    pair_chars
                        .next()
                        .ok_or_else(|| format!("Expected pair to have two chars: {:?}", pair))?,
                ),
                insert
                    .chars()
                    .next()
                    .ok_or_else(|| format!("Expected insert to have char: {:?}", insert))?,
            ))
        }

        Ok(Self {
            first,
            last,
            pairs,
            rules: lines.map(parse_rule).collect::<Result<_, _>>()?,
        })
    }
}

impl Polymer {
    pub fn tick(&mut self) {
        let mut new_pairs = HashMap::with_capacity(self.pairs.len());

        for (pair, count) in self.pairs.iter() {
            if let Some(between) = self.rules.get(&pair) {
                let pair_0 = new_pairs.entry((pair.0, *between)).or_default();
                *pair_0 += *count;
                let pair_1 = new_pairs.entry((*between, pair.1)).or_default();
                *pair_1 += *count;
            } else {
                let new_pair = new_pairs.entry(*pair).or_default();
                *new_pair += *count;
            }
        }

        self.pairs = new_pairs;
    }

    pub fn counts(&self) -> HashMap<char, usize> {
        let mut counts = HashMap::new();
        for (pair, count) in self.pairs.iter() {
            let count_0 = counts.entry(pair.0).or_default();
            *count_0 += count;
            let count_1 = counts.entry(pair.1).or_default();
            *count_1 += count;
        }

        for (_polymer, count) in counts.iter_mut() {
            *count /= 2;
        }

        let count_first = counts.entry(self.first).or_default();
        *count_first += 1;
        let count_last = counts.entry(self.last).or_default();
        *count_last += 1;
        counts
    }

    pub fn least_most_common_diff(&self) -> usize {
        match self.counts().into_values().minmax() {
            itertools::MinMaxResult::MinMax(min, max) => max - min,
            _ => unreachable!(),
        }
    }
}

type Input = Polymer;

pub fn part_1(mut input: Input) -> usize {
    for _ in 0..10 {
        input.tick();
    }
    input.least_most_common_diff()
}

pub fn part_2(mut input: Input) -> usize {
    for _ in 0..40 {
        input.tick();
    }
    input.least_most_common_diff()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Input {
        "NNCB\n\
        \n\
        CH -> B\n\
        HH -> N\n\
        CB -> H\n\
        NH -> C\n\
        HB -> C\n\
        HC -> B\n\
        HN -> C\n\
        NN -> C\n\
        BH -> H\n\
        NC -> B\n\
        NB -> B\n\
        BN -> B\n\
        BB -> N\n\
        BC -> B\n\
        CC -> N\n\
        CN -> C"
            .parse()
            .unwrap()
    }

    fn input() -> Input {
        input!("day_14_extended_polymerization").parse().unwrap()
    }

    #[test]
    fn test_steps_sample() {
        let mut s = sample();
        assert_eq!(s.pairs, parse_pairs("NNCB"));
        s.tick();
        assert_eq!(s.pairs, parse_pairs("NCNBCHB"));
        s.tick();
        assert_eq!(s.pairs, parse_pairs("NBCCNBBBCBHCB"));
        s.tick();
        assert_eq!(s.pairs, parse_pairs("NBBBCNCCNBBNBNBBCHBHHBCHB"));
        s.tick();
        assert_eq!(
            s.pairs,
            parse_pairs("NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB")
        );
    }

    #[test]
    fn test_part_1_sample() {
        assert_eq!(part_1(sample()), 1588);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(input()), 3697);
    }

    #[test]
    fn test_part_2_sample() {
        assert_eq!(part_2(sample()), 2188189693529);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(input()), 4371307836157);
    }
}
