use crate::*;

#[derive(PartialEq)]
pub enum Type {
    Round,
    Square,
    Curly,
    Angle,
}

pub enum Chunk {
    Open,
    Close,
}

pub type Token = (Chunk, Type);

use Chunk::*;
use Type::*;

pub fn tokenize(line: &str) -> impl Iterator<Item = Token> + '_ {
    line.chars().map(|c| match c {
        '(' => (Open, Round),
        ')' => (Close, Round),
        '[' => (Open, Square),
        ']' => (Close, Square),
        '{' => (Open, Curly),
        '}' => (Close, Curly),
        '<' => (Open, Angle),
        '>' => (Close, Angle),
        _ => panic!("Unexpected token {:?}, expected one of ()[]{{}}<>", c),
    })
}

enum LineAnalysis {
    Corrupted(Type),
    Incomplete(Vec<Type>),
}

fn analyze(line: impl Iterator<Item = Token>) -> LineAnalysis {
    let mut open_chunks = Vec::new();
    for (chunk, ty) in line {
        match chunk {
            Open => open_chunks.push(ty),
            Close => match open_chunks.pop() {
                Some(expected_ty) => {
                    if ty != expected_ty {
                        return LineAnalysis::Corrupted(ty);
                    }
                }
                None => unreachable!(),
            },
        }
    }
    open_chunks.reverse();
    LineAnalysis::Incomplete(open_chunks)
}

pub fn part_1<Outer, Inner>(lines: Outer) -> usize
where
    Outer: Iterator<Item = Inner>,
    Inner: Iterator<Item = Token>,
{
    lines
        .map(|line| match analyze(line) {
            LineAnalysis::Corrupted(ty) => match ty {
                Round => 3,
                Square => 57,
                Curly => 1197,
                Angle => 25137,
            },
            LineAnalysis::Incomplete(_) => 0,
        })
        .sum()
}

pub fn part_2<Outer, Inner>(lines: Outer) -> usize
where
    Outer: Iterator<Item = Inner>,
    Inner: Iterator<Item = Token>,
{
    let scores = lines
        .filter_map(|line| match analyze(line) {
            LineAnalysis::Corrupted(_) => None,
            LineAnalysis::Incomplete(closing) => Some(closing.into_iter().fold(0, |acc, ty| {
                5 * acc
                    + match ty {
                        Round => 1,
                        Square => 2,
                        Curly => 3,
                        Angle => 4,
                    }
            })),
        })
        .sorted()
        .collect::<Vec<_>>();
    scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> impl Iterator<Item = impl Iterator<Item = Token>> {
        "[({(<(())[]>[[{[]{<()<>>\n\
        [(()[<>])]({[<{<<[]>>(\n\
        {([(<{}[<>[]}>{[]{[(<()>\n\
        (((({<>}<{<{<>}{[]{[]{}\n\
        [[<[([]))<([[{}[[()]]]\n\
        [{[{({}]{}}([{[{{{}}([]\n\
        {<[[]]>}<{[{[{[]{()[[[]\n\
        [<(<(<(<{}))><([]([]()\n\
        <{([([[(<>()){}]>(<<{{\n\
        <{([{{}}[<[[[<>{}]]]>[]]"
            .lines()
            .map(tokenize)
    }

    fn input() -> impl Iterator<Item = impl Iterator<Item = Token>> {
        input!("day_10_syntax_scoring").lines().map(tokenize)
    }

    #[test]
    fn test_part_1_sample() {
        assert_eq!(part_1(sample()), 26397);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(input()), 343863);
    }

    #[test]
    fn test_part_2_sample() {
        assert_eq!(part_2(sample()), 288957);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(input()), 2924734236);
    }
}
