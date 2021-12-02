use std::str::FromStr;

use crate::*;

#[derive(Clone, Copy, Debug)]
enum Direction {
    Forward,
    Down,
    Up,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => Ok(Direction::Forward),
            "down" => Ok(Direction::Down),
            "up" => Ok(Direction::Up),
            _ => Err(format!("Unknown direction {:#?}", s)),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Command {
    direction: Direction,
    amount: i32,
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, amount) = s
            .split_once(' ')
            .ok_or_else(|| format!("Missing space delimiter in {:#?}", s))?;
        Ok(Command {
            direction: direction.parse()?,
            amount: amount.parse::<i32>().map_err(|e| e.to_string())?,
        })
    }
}

#[derive(Clone, Copy, Default, Debug)]
struct Position {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

impl Position {
    fn part_1_combine(self, command: Command) -> Self {
        match command.direction {
            Direction::Forward => Self {
                horizontal: self.horizontal + command.amount,
                ..self
            },
            Direction::Down => Self {
                depth: self.depth + command.amount,
                ..self
            },
            Direction::Up => Self {
                depth: self.depth - command.amount,
                ..self
            },
        }
    }

    fn part_2_combine(self, command: Command) -> Self {
        match command.direction {
            Direction::Forward => Self {
                horizontal: self.horizontal + command.amount,
                depth: self.depth + self.aim * command.amount,
                ..self
            },
            Direction::Down => Self {
                aim: self.aim + command.amount,
                ..self
            },
            Direction::Up => Self {
                aim: self.aim - command.amount,
                ..self
            },
        }
    }
}

impl From<Position> for i32 {
    fn from(position: Position) -> Self {
        position.horizontal * position.depth
    }
}

pub fn part_1(commands: impl Iterator<Item = Command>) -> i32 {
    commands
        .fold(Position::default(), |position, command| {
            position.part_1_combine(command)
        })
        .into()
}

pub fn part_2(commands: impl Iterator<Item = Command>) -> i32 {
    commands
        .fold(Position::default(), |position, command| {
            position.part_2_combine(command)
        })
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE: [Command; 6] = [
        Command {
            direction: Direction::Forward,
            amount: 5,
        },
        Command {
            direction: Direction::Down,
            amount: 5,
        },
        Command {
            direction: Direction::Forward,
            amount: 8,
        },
        Command {
            direction: Direction::Up,
            amount: 3,
        },
        Command {
            direction: Direction::Down,
            amount: 8,
        },
        Command {
            direction: Direction::Forward,
            amount: 2,
        },
    ];

    #[test]
    fn test_part_1_sample() {
        assert_eq!(part_1(SAMPLE.into_iter()), 150);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(input!("day_2_dive", Command)), 2027977);
    }

    #[test]
    fn test_part_2_sample() {
        assert_eq!(part_2(SAMPLE.into_iter()), 900);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(input!("day_2_dive", Command)), 1903644897);
    }
}
