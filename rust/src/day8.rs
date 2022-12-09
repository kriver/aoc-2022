use std::{cmp::max, collections::HashSet, iter::repeat};

use crate::util::load;

#[derive(Eq, Hash, PartialEq)]
struct Coord {
    x: usize,
    y: usize,
}

fn input() -> Vec<Vec<u8>> {
    let lines: Vec<String> = load("data/day8.txt");
    lines
        .into_iter()
        .map(|line| line.into_bytes().into_iter().map(|b| b - b'0').collect())
        .collect()
}

pub fn part1() -> usize {
    fn look(
        trees: &Vec<Vec<u8>>,
        visible: &mut HashSet<Coord>,
        mut max: u8,
        coords: impl Iterator<Item = (usize, usize)>,
    ) {
        for (x, y) in coords {
            max = if trees[y][x] > max {
                visible.insert(Coord { x, y });
                trees[y][x]
            } else {
                max
            };
            if max == 9 {
                break;
            }
        }
    }

    let trees = input();
    let mut visible: HashSet<Coord> = HashSet::new();
    let (h, w) = (trees.len(), trees[0].len());
    // horizontal
    for y in 1..h - 1 {
        // look right
        let max = trees[y][0];
        look(&trees, &mut visible, max, (1..(w - 1)).zip(repeat(y)));
        // look left
        let max = trees[y][w - 1];
        look(&trees, &mut visible, max, (1..(w - 1)).rev().zip(repeat(y)));
    }
    for x in 1..w - 1 {
        // look down
        let max = trees[0][x];
        look(&trees, &mut visible, max, repeat(x).zip(1..(h - 1)));
        // look up
        let max = trees[h - 1][x];
        look(&trees, &mut visible, max, repeat(x).zip((1..(h - 1)).rev()));
    }
    visible.len() + 2 * (w + h - 2) // the border
}

fn tree_score(trees: &Vec<Vec<u8>>, x: usize, y: usize) -> usize {
    fn look(trees: &Vec<Vec<u8>>, max: u8, coords: impl Iterator<Item = (usize, usize)>) -> usize {
        let mut cnt = 0;
        for (x, y) in coords {
            cnt += 1;
            if trees[y][x] >= max {
                break;
            }
        }
        cnt
    }

    let (h, w) = (trees.len(), trees[0].len());
    let max = trees[y][x];
    let mut score = 1;
    // look left
    score *= look(trees, max, (0..x).rev().zip(repeat(y)));
    // look right
    score *= look(trees, max, ((x + 1)..w).zip(repeat(y)));
    // look up
    score *= look(trees, max, repeat(x).zip((0..y).rev()));
    // look down
    score *= look(trees, max, repeat(x).zip((y + 1)..h));
    score
}

pub fn part2() -> usize {
    let trees = input();
    let (h, w) = (trees.len(), trees[0].len());
    let mut score = 0;
    for y in 0..h {
        for x in 0..w {
            let s = tree_score(&trees, x, y);
            score = max(s, score);
        }
    }
    score
}

mod tests {
    #[test]
    fn test_part1() {
        let num = super::part1();
        println!("Number of visible trees: {}", num);
        assert_eq!(num, 1703);
    }

    #[test]
    fn test_part2() {
        let score = super::part2();
        println!("Highest scenic score: {}", score);
        assert_eq!(score, 496650);
    }
}
