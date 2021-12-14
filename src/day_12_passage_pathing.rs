use std::collections::HashSet;
use std::collections::VecDeque;
use std::str::FromStr;

use crate::*;

#[derive(PartialEq, Eq, Hash)]
pub enum Cave {
    Start,
    End,
    Small(String),
    Large(String),
}

impl Cave {
    pub fn is_small(&self) -> bool {
        matches!(self, Cave::Small(_))
    }

    pub fn is_large(&self) -> bool {
        matches!(self, Cave::Large(_))
    }
}

impl FromStr for Cave {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "start" => Ok(Cave::Start),
            "end" => Ok(Cave::End),
            _ if s.chars().all(|c| c.is_ascii_lowercase()) => Ok(Cave::Small(s.to_owned())),
            _ if s.chars().all(|c| c.is_ascii_uppercase()) => Ok(Cave::Large(s.to_owned())),
            _ => Err(format!("Bad cave {:?}", s)),
        }
    }
}

pub struct CaveSystem {
    connections: Vec<(Cave, Cave)>,
}

impl FromStr for CaveSystem {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_line(line: &str) -> Result<(Cave, Cave), String> {
            let (a, b) = line
                .split_once('-')
                .ok_or_else(|| format!("No '-' in line: {:?}", line))?;
            Ok((a.parse()?, b.parse()?))
        }

        Ok(Self {
            connections: s.lines().map(parse_line).collect::<Result<_, _>>()?,
        })
    }
}

impl CaveSystem {
    pub fn neighbors(&self, cave: &Cave) -> Vec<&Cave> {
        self.connections
            .iter()
            .filter_map(|(a, b)| {
                if a == cave {
                    Some(b)
                } else if b == cave {
                    Some(a)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn distinct_paths(&self, allow_one_small_cave_twice: bool) -> usize {
        // Like the DFA regex solver strategy -- just keep track of where each
        // path is and move the pointers around step-by-step until they get to
        // the end state.

        let mut finished = 0;
        let mut nodes: Vec<(&Cave, Vec<&Cave>, bool)> = Vec::with_capacity(self.connections.len());
        nodes.push((&Cave::Start, Vec::new(), false));

        let mut new_nodes = Vec::with_capacity(self.connections.len());

        while !nodes.is_empty() {
            new_nodes.clear();
            for (cave, visited, visited_one_small_cave) in nodes.iter() {
                for neighbor in self.neighbors(cave) {
                    match neighbor {
                        Cave::Start => {}
                        Cave::End => {
                            finished += 1;
                        }
                        Cave::Small(_) => {
                            if !visited.contains(&neighbor) {
                                let mut new_visited = visited.clone();
                                new_visited.push(neighbor);
                                new_nodes.push((neighbor, new_visited, *visited_one_small_cave));
                            } else if allow_one_small_cave_twice && !visited_one_small_cave {
                                new_nodes.push((neighbor, visited.clone(), true));
                            }
                        }
                        Cave::Large(_) => {
                            new_nodes.push((neighbor, visited.clone(), *visited_one_small_cave));
                        }
                    }
                }
            }
            std::mem::swap(&mut nodes, &mut new_nodes);
        }

        finished
    }
}

type Input = CaveSystem;

pub fn part_1(input: Input) -> usize {
    input.distinct_paths(false)
}

pub fn part_2(input: Input) -> usize {
    input.distinct_paths(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Input {
        "fs-end\n\
        he-DX\n\
        fs-he\n\
        start-DX\n\
        pj-DX\n\
        end-zg\n\
        zg-sl\n\
        zg-pj\n\
        pj-he\n\
        RW-he\n\
        fs-DX\n\
        pj-RW\n\
        zg-RW\n\
        start-pj\n\
        he-WI\n\
        zg-he\n\
        pj-fs\n\
        start-RW"
            .parse()
            .unwrap()
    }

    fn input() -> Input {
        input!("day_12_passage_pathing").parse().unwrap()
    }

    #[test]
    fn test_part_1_sample() {
        assert_eq!(part_1(sample()), 226);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(input()), 3576);
    }

    #[test]
    fn test_part_2_sample() {
        assert_eq!(part_2(sample()), 3509);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(input()), 84271);
    }
}
