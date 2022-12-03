use std::collections::HashSet;

use std::str::FromStr;

use crate::util::load;

struct Rucksack {
    comp1: HashSet<char>,
    comp2: HashSet<char>,
}

impl FromStr for Rucksack {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut comp1 = HashSet::new();
        let mut comp2 = HashSet::new();
        for (i, c) in s.chars().enumerate() {
            if i < s.len() / 2 {
                comp1.insert(c);
            } else {
                comp2.insert(c);
            }
        }
        Ok(Rucksack { comp1, comp2 })
    }
}

pub fn priority(c: &char) -> u32 {
    match c {
        'A'..='Z' => Into::<u32>::into(*c) - Into::<u32>::into('A') + 27,
        'a'..='z' => Into::<u32>::into(*c) - Into::<u32>::into('a') + 1,
        _ => 0,
    }
}

pub fn part1() -> u32 {
    let rucksacks: Vec<Rucksack> = load("data/day3.txt");
    rucksacks
        .into_iter()
        .map(|r| {
            r.comp1
                .intersection(&r.comp2)
                .map(|c| priority(c))
                .sum::<u32>()
        })
        .sum()
}

pub fn part2() -> u32 {
    let rucksacks: Vec<Rucksack> = load("data/day3.txt");
    rucksacks
        .chunks(3)
        .map(|chunk| {
            chunk
                .into_iter()
                .map(|r| r.comp1.union(&r.comp2).copied().collect())
                .reduce(|acc: HashSet<char>, r| acc.intersection(&r).copied().collect())
                .unwrap()
                .into_iter()
                .map(|c| priority(&c))
                .sum::<u32>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let priority_sum = part1();
        println!("Priority sum is {}", priority_sum);
        assert_eq!(priority_sum, 7674);
    }

    #[test]
    fn test_part2() {
        let priority_sum = part2();
        println!("Priority sum is {}", priority_sum);
        assert_eq!(priority_sum, 2805);
    }
}
