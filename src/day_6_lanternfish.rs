use crate::*;

const MAX_LANTERNFISH_TIMER: usize = 8;

// A school of fish is a 9-element array where the element at index `i` is the
// number of fish with a timer of `i`.
pub struct School([usize; MAX_LANTERNFISH_TIMER + 1]);

impl FromIterator<usize> for School {
    fn from_iter<T: IntoIterator<Item = usize>>(iter: T) -> Self {
        let mut ret = Self::new();
        for time in iter {
            if time > MAX_LANTERNFISH_TIMER {
                panic!("Invalid lanternfish time (>8): {}", time);
            }
            ret.0[time] += 1;
        }
        ret
    }
}

impl School {
    pub fn new() -> Self {
        Self([0; MAX_LANTERNFISH_TIMER + 1])
    }

    pub fn tick(&mut self) {
        let new_fish = self.0[0];
        self.0.rotate_left(1);
        self.0[6] += new_fish;
    }

    pub fn total_fish(&self) -> usize {
        self.0.into_iter().sum()
    }
}

pub fn part_1(mut school: School) -> usize {
    for _ in 0..80 {
        school.tick();
    }
    school.total_fish()
}

pub fn part_2(mut school: School) -> usize {
    for _ in 0..256 {
        school.tick();
    }
    school.total_fish()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> School {
        [3, 4, 3, 1, 2].into_iter().collect()
    }

    fn input() -> School {
        input!("day_6_lanternfish")
            .trim()
            .split(',')
            .map(|time| {
                time.parse()
                    .map_err(|err| format!("{}: {:?}", err, time))
                    .unwrap()
            })
            .collect()
    }

    #[test]
    fn test_part_1_sample() {
        assert_eq!(part_1(sample()), 5934);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(input()), 389726);
    }

    #[test]
    fn test_part_2_sample() {
        assert_eq!(part_2(sample()), 26984457539);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(input()), 1743335992042);
    }
}
