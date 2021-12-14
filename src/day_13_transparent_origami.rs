use std::collections::HashSet;
use std::str::FromStr;

use crate::*;

pub struct Origami {
    paper: Paper,
    folds: Vec<Fold>,
}

fn parse_coords(line: &str) -> Result<(usize, usize), String> {
    let (x, y) = line
        .split_once(',')
        .ok_or_else(|| format!("Expected coordinates delimited by ',', got: {:?}", line))?;
    Ok((
        x.parse().map_err(|e| format!("{}: {:?}", e, line))?,
        y.parse().map_err(|e| format!("{}: {:?}", e, line))?,
    ))
}

impl FromStr for Origami {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parsing_coords = true;

        let mut coords = HashSet::new();
        let mut folds = Vec::new();

        for line in s.lines() {
            if parsing_coords {
                if line.is_empty() {
                    parsing_coords = false;
                } else {
                    coords.insert(parse_coords(line)?);
                }
            } else {
                folds.push(line.parse()?);
            }
        }

        Ok(Self {
            paper: Paper(coords),
            folds,
        })
    }
}

pub struct Paper(HashSet<(usize, usize)>);

impl Paper {
    pub fn fold(&self, fold: Fold) -> Paper {
        let mut ret = HashSet::with_capacity(self.0.len());

        for (x, y) in self.0.iter() {
            match &fold {
                Fold::X(fold_at) => {
                    if x < fold_at {
                        ret.insert((*x, *y));
                    } else {
                        ret.insert((2 * fold_at - *x, *y));
                    }
                }
                Fold::Y(fold_at) => {
                    if y < fold_at {
                        ret.insert((*x, *y));
                    } else {
                        ret.insert((*x, 2 * fold_at - *y));
                    }
                }
            }
        }

        Paper(ret)
    }
}

#[derive(Clone, Copy)]
pub enum Fold {
    X(usize),
    Y(usize),
}

impl FromStr for Fold {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const PREFIX: &str = "fold along ";

        if !s.starts_with(PREFIX) {
            return Err(format!("Fold must start with 'fold along', got: {:?}", s));
        }

        let s = &s[PREFIX.len()..];

        let (axis, coord) = s
            .split_once('=')
            .ok_or_else(|| format!("Expected fold to be delimited with '=', got: {:?}", s))?;

        let coord: usize = coord.parse().map_err(|e| format!("{}: {:?}", e, coord))?;

        match axis {
            "x" => Ok(Fold::X(coord)),
            "y" => Ok(Fold::Y(coord)),
            _ => Err(format!("Axis must be 'x' or 'y', got {:?}", axis)),
        }
    }
}

type Input = Origami;

pub fn part_1(input: Input) -> usize {
    input.paper.fold(input.folds[0]).0.len()
}

pub fn part_2(input: Input) -> String {
    let final_paper = input
        .folds
        .into_iter()
        .fold(input.paper, |paper, fold| paper.fold(fold));
    let (x_extent, y_extent) = final_paper
        .0
        .iter()
        .fold((0, 0), |(x_extent, y_extent), (x, y)| {
            (x_extent.max(*x), y_extent.max(*y))
        });

    let mut grid = vec![vec![false; x_extent + 1]; y_extent + 1];

    for (x, y) in final_paper.0 {
        grid[y][x] = true;
    }

    let mut ret = String::with_capacity((x_extent + 1) * y_extent);
    for row in grid {
        for col in row {
            ret.push(if col { '#' } else { '.' });
        }
        ret.push('\n');
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Input {
        "6,10\n\
        0,14\n\
        9,10\n\
        0,3\n\
        10,4\n\
        4,11\n\
        6,0\n\
        6,12\n\
        4,1\n\
        0,13\n\
        10,12\n\
        3,4\n\
        3,0\n\
        8,4\n\
        1,10\n\
        2,14\n\
        8,10\n\
        9,0\n\
        \n\
        fold along y=7\n\
        fold along x=5"
            .parse()
            .unwrap()
    }

    fn input() -> Input {
        input!("day_13_transparent_origami").parse().unwrap()
    }

    #[test]
    fn test_part_1_sample() {
        assert_eq!(part_1(sample()), 17);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(input()), 618);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(input()),
            // "ALREKFKU"
            ".##..#....###..####.#..#.####.#..#.#..#\n\
             #..#.#....#..#.#....#.#..#....#.#..#..#\n\
             #..#.#....#..#.###..##...###..##...#..#\n\
             ####.#....###..#....#.#..#....#.#..#..#\n\
             #..#.#....#.#..#....#.#..#....#.#..#..#\n\
             #..#.####.#..#.####.#..#.#....#..#..##.\n"
        );
    }
}
