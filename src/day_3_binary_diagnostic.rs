use std::str::FromStr;

use crate::*;

fn ones_per_bit<I: Iterator<Item = usize>, const BITS: usize>(numbers: I) -> [usize; BITS] {
    let mut ones: [usize; BITS] = [0; BITS];
    for num in numbers {
        for bit in 0..BITS {
            ones[bit] += (num >> bit) & 1;
        }
    }
    ones
}

pub fn part_1<I: Iterator<Item = usize>, const BITS: usize>(numbers: I) -> usize {
    let numbers = numbers.collect::<Vec<_>>();
    let ones = ones_per_bit::<_, BITS>(numbers.iter().copied());
    // If there are more than `threshold` 1-bits at a certain position, 1 is the
    // most common bit at that position.
    let threshold = numbers.len() / 2;
    let gamma: usize = ones
        .into_iter()
        .enumerate()
        .fold(0, |acc, (bit, bit_ones)| {
            let bit_value = if bit_ones >= threshold { 1 } else { 0 };
            acc + (bit_value << bit)
        });
    // 0b1111...1111 for `BITS` ones.
    let all_ones = (2 << (BITS - 1)) - 1;
    let epsilon = !gamma & all_ones;
    gamma * epsilon
}

pub fn part_2<I: Iterator<Item = usize>, const BITS: usize>(numbers: I) -> usize {
    let numbers = numbers.collect::<Vec<_>>();
    let mut oxygen_numbers = numbers.clone();
    let mut oxygen_rating = None;
    for bit in (0..BITS).rev() {
        let ones = ones_per_bit::<_, BITS>(oxygen_numbers.iter().copied());
        // Letting `b = ones[bit]` and `n = oxygen_numbers.len()`;
        // The zero count is given by `n - b`, so "there are more ones than
        // zeros" is:
        //      b ≥ n - b
        //       ...
        //     2b ≥ n
        let most_common_bit = if 2 * ones[bit] >= oxygen_numbers.len() {
            1
        } else {
            0
        };
        oxygen_numbers.retain(|num| (num >> bit) & 1 == most_common_bit);

        if oxygen_numbers.len() == 1 {
            oxygen_rating = Some(oxygen_numbers[0]);
            break;
        }
    }

    let mut co2_numbers = numbers.clone();
    let mut co2_rating = None;
    for bit in (0..BITS).rev() {
        let ones = ones_per_bit::<_, BITS>(co2_numbers.iter().copied());
        let least_common_bit = if 2 * ones[bit] >= co2_numbers.len() {
            0
        } else {
            1
        };
        co2_numbers.retain(|num| (num >> bit) & 1 == least_common_bit);

        if co2_numbers.len() == 1 {
            co2_rating = Some(co2_numbers[0]);
            break;
        }
    }

    println!(
        "oxygen rating: {:?}, co2 rating: {:?}",
        oxygen_rating, co2_rating
    );
    oxygen_rating.expect("No oxygen generator rating found!")
        * co2_rating.expect("No CO₂ scrubber rating found!")
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE: [usize; 12] = [
        0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000, 0b11001,
        0b00010, 0b01010,
    ];

    fn input() -> impl Iterator<Item = usize> {
        input!("day_3_binary_diagnostic")
            .lines()
            .map(|num| usize::from_str_radix(num, 2).expect("Input numbers should be binary"))
    }

    #[test]
    fn test_part_1_sample() {
        assert_eq!(part_1::<_, 5>(SAMPLE.into_iter()), 198);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1::<_, 12>(input()), 3985686);
    }

    #[test]
    fn test_part_2_sample() {
        assert_eq!(part_2::<_, 5>(SAMPLE.into_iter()), 230);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2::<_, 12>(input()), 2555739);
    }
}
