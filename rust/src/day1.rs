use std::collections::BinaryHeap;

use crate::util::load;

pub fn input() -> Vec<String> {
    let calories: Vec<String> = load("data/day1.txt");
    calories
}

fn sorted_sums(calories: Vec<String>) -> Vec<u32> {
    let (mut sums, last) = calories.iter().fold(
        (BinaryHeap::new(), 0),
        |(mut sums, current), cal| match cal.as_str() {
            "" => {
                sums.push(current);
                (sums, 0)
            }
            s => (sums, current + s.parse::<u32>().unwrap()),
        },
    );
    sums.push(last);
    let mut v = sums.into_sorted_vec();
    v.reverse();
    v
}

pub fn part1(calories: Vec<String>) -> u32 {
    sorted_sums(calories)[0]
}

pub fn part2(calories: Vec<String>) -> u32 {
    sorted_sums(calories)[0..3].iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::day1::{input, part1, part2};

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 72017);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 212520);
    }
}
