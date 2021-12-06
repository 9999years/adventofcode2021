use std::str::FromStr;

use crate::*;

#[derive(Clone, Debug)]
pub struct BingoGame {
    numbers: Vec<usize>,
    boards: Vec<Board>,
}

impl FromStr for BingoGame {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chunks = s.split("\n\n");
        let numbers = chunks
            .next()
            .ok_or_else(|| "Expected first line".to_owned())?
            .split(',')
            .map(|num| num.parse())
            .collect::<Result<Vec<usize>, std::num::ParseIntError>>()
            .map_err(|err| err.to_string())?;

        let boards = chunks
            .map(|board| board.parse())
            .collect::<Result<Vec<Board>, _>>()?;

        Ok(Self { numbers, boards })
    }
}

#[derive(Clone, Debug)]
pub struct Board([(usize, bool); 25]);

impl FromStr for Board {
    type Err = String;

    fn from_str(board: &str) -> Result<Self, Self::Err> {
        let mut nums: Vec<(usize, bool)> = Vec::with_capacity(25);
        for line in board.lines() {
            for num in line.split_ascii_whitespace() {
                nums.push((num.parse::<usize>().map_err(|err| err.to_string())?, false));
            }
        }

        Ok(Board(nums.try_conv::<[(usize, bool); 25]>().map_err(
            |err| format!("Board had {} numbers instead of 25", err.len()),
        )?))
    }
}

impl Board {
    pub fn row(&self, row_index: usize) -> &[(usize, bool)] {
        if row_index >= 5 {
            panic!("Row index must be in 0..5");
        }
        &self.0[5 * row_index..5 * row_index + 5]
    }

    pub fn col(&self, col_index: usize) -> [(usize, bool); 5] {
        if col_index >= 5 {
            panic!("Col index must be in 0..5");
        }
        [
            self.0[col_index],
            self.0[col_index + 5],
            self.0[col_index + 10],
            self.0[col_index + 15],
            self.0[col_index + 20],
        ]
    }

    pub fn is_winning(&self) -> bool {
        (0..5)
            .map(|idx| {
                self.row(idx).into_iter().all(|(_, marked)| *marked)
                    || self.col(idx).into_iter().all(|(_, marked)| marked)
            })
            .any(|all_marked| all_marked)
    }

    pub fn unmarked_sum(&self) -> usize {
        self.0
            .iter()
            .filter(|(_, marked)| !marked)
            .map(|(num, _)| num)
            .sum()
    }

    /// Mark a given number on the board; returns if any number was marked.
    pub fn mark(&mut self, number: usize) -> bool {
        for spot in &mut self.0 {
            if spot.0 == number {
                spot.1 = true;
                return true;
            }
        }
        false
    }
}

pub fn part_1(mut game: BingoGame) -> usize {
    for number in game.numbers {
        for board in &mut game.boards {
            board.mark(number);
            if board.is_winning() {
                return number * board.unmarked_sum();
            }
        }
    }
    panic!("No winning board found");
}

pub fn part_2(mut game: BingoGame) -> usize {
    for number in game.numbers {
        for board in &mut game.boards {
            board.mark(number);
        }
        if game.boards.len() > 1 {
            game.boards.retain(|board| !board.is_winning());
        } else if game.boards[0].is_winning() {
            return number * game.boards[0].unmarked_sum();
        }
    }
    panic!("No last winner");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> BingoGame {
        input!("day_4_giant_squid").parse().unwrap()
    }

    fn sample() -> BingoGame {
        r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

        22 13 17 11  0
         8  2 23  4 24
        21  9 14 16  7
         6 10  3 18  5
         1 12 20 15 19

         3 15  0  2 22
         9 18 13 17  5
        19  8  7 25 23
        20 11 10 24  4
        14 21 16 12  6

        14 21 17 24  4
        10 16 15  9 19
        18  8 23 26 20
        22 11 13  6  5
         2  0 12  3  7
        "#
        .parse()
        .unwrap()
    }

    #[test]
    fn test_part_1_sample() {
        assert_eq!(part_1(sample()), 4512);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(input()), 41668);
    }

    #[test]
    fn test_part_2_sample() {
        assert_eq!(part_2(sample()), 1924);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(input()), 10478);
    }
}
