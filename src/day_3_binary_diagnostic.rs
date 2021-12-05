use std::str::FromStr;

use crate::*;

pub fn part_1<I: Iterator<Item = usize>, const BITS: usize>(commands: I) -> usize {
    let mut ones: [usize; BITS] = [0; BITS];
    let total: usize = commands.fold(0, |count, num| {
        for bit in 0..BITS {
            ones[bit] += (num >> bit) & 1;
        }
        count + 1
    });
    // If there are more than `threshold` 1-bits at a certain position, 1 is the
    // most common bit at that position.
    let threshold = total / 2;
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

pub fn part_2(commands: impl Iterator<Item = u32>) -> i32 {
    todo!()
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
        // assert_eq!(part_2(SAMPLE.into_iter()), 900);
    }

    #[test]
    fn test_part_2() {
        // assert_eq!(part_2(input!("day_2_dive", Command)), 1903644897);
    }
}
