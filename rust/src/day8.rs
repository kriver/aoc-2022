use std::{cmp::max, collections::HashSet};

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

fn is_visible(
    trees: &Vec<Vec<u8>>,
    visible: &mut HashSet<Coord>,
    x: usize,
    y: usize,
    mut max: u8,
) -> u8 {
    if trees[y][x] > max {
        visible.insert(Coord { x, y });
        max = trees[y][x]
    }
    max
}

pub fn part1() -> usize {
    let trees = input();
    let mut visible: HashSet<Coord> = HashSet::new();
    let (h, w) = (trees.len(), trees[0].len());
    // horizontal
    for y in 1..h - 1 {
        // look right
        let mut max = trees[y][0];
        let mut x = 1;
        while x < w - 1 {
            max = is_visible(&trees, &mut visible, x, y, max);
            if max == 9 {
                break;
            }
            x += 1;
        }
        // look left
        let mut max = trees[y][w - 1];
        let mut x = w - 2;
        while x > 0 {
            max = is_visible(&trees, &mut visible, x, y, max);
            if max == 9 {
                break;
            }
            x -= 1;
        }
    }
    for x in 1..w - 1 {
        // look down
        let mut max = trees[0][x];
        let mut y = 1;
        while y < h - 1 {
            max = is_visible(&trees, &mut visible, x, y, max);
            if max == 9 {
                break;
            }
            y += 1;
        }
        // look up
        let mut max = trees[h - 1][x];
        let mut y = h - 2;
        while y > 0 {
            max = is_visible(&trees, &mut visible, x, y, max);
            if max == 9 {
                break;
            }
            y -= 1;
        }
    }
    visible.len() + 2 * (w + h - 2) // the border
}

fn tree_score(trees: &Vec<Vec<u8>>, x: usize, y: usize) -> usize {
    let (h, w) = (trees.len(), trees[0].len());
    let max = trees[y][x];
    let mut score = 1;
    if score > 0 {
        // look left
        let mut cnt = 0;
        for ix in (0..x).rev() {
            cnt += 1;
            if trees[y][ix] >= max {
                break;
            }
        }
        score *= cnt;
    }
    if score > 0 {
        // look right
        let mut cnt = 0;
        for ix in (x + 1)..w {
            cnt += 1;
            if trees[y][ix] >= max {
                break;
            }
        }
        score *= cnt;
    }
    if score > 0 {
        // look up
        let mut cnt = 0;
        for iy in (0..y).rev() {
            cnt += 1;
            if trees[iy][x] >= max {
                break;
            }
        }
        score *= cnt;
    }
    if score > 0 {
        // look down
        let mut cnt = 0;
        for iy in (y + 1)..h {
            cnt += 1;
            if trees[iy][x] >= max {
                break;
            }
        }
        score *= cnt;
    }
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
    use super::*;

    #[test]
    fn test_part1() {
        let num = part1();
        println!("Number of visible trees: {}", num);
        assert_eq!(num, 1703);
    }

    #[test]
    fn test_part2() {
        let score = part2();
        println!("Highest scenic score: {}", score);
        assert_eq!(score, 496650);
    }
}
