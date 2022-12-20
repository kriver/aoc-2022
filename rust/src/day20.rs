use std::str::FromStr;

use crate::util::load;

#[derive(Debug)]
struct Item {
    v: i64,
    moved: bool,
}

impl FromStr for Item {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Item {
            v: s.parse().unwrap(),
            moved: false,
        })
    }
}

fn mod_add(a: i64, b: i64, m: i64) -> (i64, i64) {
    let mut sum = a + b;
    let mut wrap = sum / m;
    sum %= m;
    if sum < 0 {
        while sum < 0 {
            sum += m;
            wrap -= 1;
        }
    } else {
        while sum >= m {
            sum -= m;
            wrap += 1;
        }
    }
    (wrap, sum)
}

fn mix(items: &mut Vec<Item>) {
    let len = items.len() as i64;
    let mut moved = 0;
    let mut idx = 0;
    while moved < len {
        if items[idx].v == 0 {
            items[idx].moved = true;
            moved += 1;
        }
        if items[idx].moved {
            idx = (idx + 1) % len as usize;
            continue;
        }
        let mut item = items.remove(idx);
        let (wrap, mut new_idx) = mod_add(idx as i64, item.v, len);
        if wrap < 0 {
            // wrap-around left
            if new_idx == 0 {
                (_, new_idx) = mod_add(new_idx, wrap - 1, len);
            } else {
                (_, new_idx) = mod_add(new_idx, wrap, len);
            }
        }
        if wrap > 0 {
            // wrap-around right
            if new_idx == len - 1 {
                (_, new_idx) = mod_add(new_idx, wrap + 1, len);
            } else {
                (_, new_idx) = mod_add(new_idx, wrap, len);
            }
        }
        item.moved = true;
        moved += 1;
        items.insert(new_idx as usize, item);
    }
}

fn score(items: &Vec<Item>) -> i64 {
    let len = items.len();
    let p0 = items.iter().position(|item| item.v == 0).unwrap();
    let p1 = (p0 + 1000) % len;
    let p2 = (p0 + 2000) % len;
    let p3 = (p0 + 3000) % len;
    items[p1].v + items[p2].v + items[p3].v
}

pub fn part1() -> i64 {
    let mut numbers: Vec<Item> = load("data/day20.txt");
    mix(&mut numbers);
    score(&numbers)
}

pub fn part2() {
    // let mut numbers: Vec<Item> = vec!["1", "2", "-3", "3", "-2", "0", "4"]
    // .into_iter()
    // .map(|s| s.parse().unwrap())
    // .collect();
}

pub fn do_mix(data: &[i64]) -> Vec<i64> {
    let mut data: Vec<Item> = data
        .into_iter()
        .map(|i| format!("{}", i))
        .map(|s| s.parse().unwrap())
        .collect();
    mix(&mut data);
    data.into_iter().map(|i| i.v).collect()
}

mod tests {
    #[test]
    fn test_mix() {
        assert_eq!(super::do_mix(&[0, 0, 0]), vec![0, 0, 0]);
        assert_eq!(super::do_mix(&[1, 0, 0]), vec![0, 1, 0]);
        assert_eq!(super::do_mix(&[2, 0, 0]), vec![0, 0, 2]);
        assert_eq!(super::do_mix(&[3, 0, 0]), vec![0, 3, 0]);
        assert_eq!(super::do_mix(&[0, 1, 0]), vec![0, 0, 1]);
        assert_eq!(super::do_mix(&[0, 2, 0]), vec![0, 2, 0]);
        assert_eq!(super::do_mix(&[0, 3, 0]), vec![0, 0, 3]);
        assert_eq!(super::do_mix(&[0, 0, 1]), vec![0, 1, 0]);
        assert_eq!(super::do_mix(&[0, 0, 2]), vec![0, 0, 2]);
        assert_eq!(super::do_mix(&[0, 0, 3]), vec![0, 3, 0]);
        assert_eq!(super::do_mix(&[0, 0, 4]), vec![0, 0, 4]);
        assert_eq!(super::do_mix(&[-1, 0, 0]), vec![0, -1, 0]);
        assert_eq!(super::do_mix(&[-2, 0, 0]), vec![-2, 0, 0]);
        assert_eq!(super::do_mix(&[-3, 0, 0]), vec![0, -3, 0]);
        assert_eq!(super::do_mix(&[-4, 0, 0]), vec![-4, 0, 0]);
        assert_eq!(super::do_mix(&[0, -1, 0]), vec![-1, 0, 0]);
        assert_eq!(super::do_mix(&[0, -2, 0]), vec![0, -2, 0]);
        assert_eq!(super::do_mix(&[0, -3, 0]), vec![-3, 0, 0]);
        assert_eq!(super::do_mix(&[0, 0, -1]), vec![0, -1, 0]);
        assert_eq!(super::do_mix(&[0, 0, -2]), vec![-2, 0, 0]);
        assert_eq!(super::do_mix(&[0, 0, -3]), vec![0, -3, 0]);
    }

    #[test]
    fn test_part1() {
        let sum = super::part1();
        println!("Sum: {}", sum);
        assert_eq!(sum, 11123);
    }

    #[test]
    fn test_part2() {}
}
