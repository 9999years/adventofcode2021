use crate::*;

pub fn part_1(depths: impl Iterator<Item = u16>) -> usize {
    depths.tuple_windows().filter(|(a, b)| a < b).count()
}

pub fn part_2(depths: impl Iterator<Item = u16>) -> usize {
    depths
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .filter(|(prev, next)| prev < next)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE: [u16; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    #[test]
    fn test_part_1_sample() {
        assert_eq!(part_1(SAMPLE.into_iter()), 7);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(input!("day_1_sonar_sweep", u16)), 1215);
    }

    #[test]
    fn test_part_2_sample() {
        assert_eq!(part_2(SAMPLE.into_iter()), 5);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(input!("day_1_sonar_sweep", u16)), 1150);
    }
}
