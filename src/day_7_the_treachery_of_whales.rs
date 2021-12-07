use crate::*;

fn abs_diff(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}

// See: https://cseweb.ucsd.edu/groups/tatami/handdemos/sum/
fn sum_of_first_n(n: usize) -> usize {
    (n * (n + 1)) / 2
}

// Order doesn't matter so we can sort the data.
fn median(data: &mut [usize]) -> usize {
    data.sort();
    data[data.len() / 2]
}

fn alignment_cost(positions: &[usize], metric: impl Fn(usize) -> usize) -> usize {
    positions
        .into_iter()
        .map(|position| metric(*position))
        .sum()
}

fn linear_alignment_cost(positions: &[usize], align_at: usize) -> usize {
    alignment_cost(positions, |position| abs_diff(position, align_at))
}

fn increasing_alignment_cost(positions: &[usize], align_at: usize) -> usize {
    alignment_cost(positions, |position| {
        sum_of_first_n(abs_diff(position, align_at))
    })
}

pub fn part_1(mut positions: Vec<usize>) -> usize {
    let guess = median(&mut positions);
    linear_alignment_cost(&positions, guess)
}

pub fn part_2(mut positions: Vec<usize>) -> usize {
    let initial_guess = median(&mut positions);
    let mut best_cost = increasing_alignment_cost(&positions, initial_guess);
    // If we start seeing costs 2x the initial guess, we're probably done:
    let bailout_cost = 2 * best_cost;

    // If our initial guess is `n`, our guesses are `n + 1, n - 1, n + 2, n - 2, ...`
    let guesses = (0..).flat_map(|i| {
        [Some(initial_guess + i), initial_guess.checked_sub(i)]
            .into_iter()
            .flatten()
    });
    for guess in guesses {
        let cost = increasing_alignment_cost(&positions, guess);
        best_cost = best_cost.min(cost);
        if cost > bailout_cost {
            break;
        }
    }
    best_cost
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Vec<usize> {
        [16, 1, 2, 0, 4, 2, 7, 1, 2, 14].into()
    }

    fn input() -> Vec<usize> {
        input!("day_7_the_treachery_of_whales")
            .trim()
            .split(',')
            .map(|num| num.parse().unwrap())
            .collect()
    }

    #[test]
    fn test_part_1_sample() {
        assert_eq!(part_1(sample()), 37);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(input()), 341558);
    }

    #[test]
    fn test_part_2_sample() {
        assert_eq!(part_2(sample()), 168);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(input()), 93214037);
    }
}
