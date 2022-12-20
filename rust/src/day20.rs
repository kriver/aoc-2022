use std::str::FromStr;

use crate::util::load;

#[derive(Debug)]
struct Item {
    v: i64,
    id: i64,
}

impl FromStr for Item {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Item {
            v: s.parse().unwrap(),
            id: 0,
        })
    }
}

fn mod_add(a: i64, b: i64, l: i64) -> i64 {
    let modulo = l - 1;
    if b > 0 {
        1 + (a + b - 1) % modulo
    } else {
        (modulo + 1 + (a + b - 1) % modulo) % modulo
    }
}

fn mix(items: &mut Vec<Item>) {
    let len = items.len() as i64;
    let mut id = 0;
    let mut idx = 0;
    while id < len {
        while items[idx].id != id {
            idx = (idx + 1) % len as usize;
        }
        id += 1;
        if items[idx].v == 0 {
            idx = (idx + 1) % len as usize;
            continue;
        }
        let item = items.remove(idx);
        let new_idx = mod_add(idx as i64, item.v, len);
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

fn load_items(filename: &str, multiplier: i64) -> Vec<Item> {
    let mut items = load::<Item>(filename);
    for (id, item) in items.iter_mut().enumerate() {
        item.id = id as i64;
        item.v *= multiplier;
    }
    items
}

pub fn part1() -> i64 {
    let mut numbers: Vec<Item> = load_items("data/day20.txt", 1);
    mix(&mut numbers);
    score(&numbers)
}

pub fn part2() -> i64 {
    let multiplier = 811589153;
    let mut numbers = load_items("data/day20.txt", multiplier);
    for _ in 0..10 {
        mix(&mut numbers);
    }
    score(&numbers)
}

pub fn do_mix(data: &[i64]) -> Vec<i64> {
    let mut data: Vec<Item> = data
        .into_iter()
        .map(|i| format!("{}", i))
        .map(|s| s.parse::<Item>().unwrap())
        .enumerate()
        .map(|(id, mut item)| {
            item.id = id as i64;
            item
        })
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
        assert_eq!(super::do_mix(&[5, 0, 0]), vec![0, 5, 0]);
        assert_eq!(super::do_mix(&[0, 5, 0]), vec![0, 0, 5]);
        assert_eq!(super::do_mix(&[0, 0, 5]), vec![0, 5, 0]);
        assert_eq!(super::do_mix(&[0, 0, 6]), vec![0, 0, 6]);
        assert_eq!(super::do_mix(&[0, 0, 15]), vec![0, 15, 0]);
        assert_eq!(super::do_mix(&[-1, 0, 0]), vec![0, -1, 0]);
        assert_eq!(super::do_mix(&[-2, 0, 0]), vec![-2, 0, 0]);
        assert_eq!(super::do_mix(&[-3, 0, 0]), vec![0, -3, 0]);
        assert_eq!(super::do_mix(&[-4, 0, 0]), vec![-4, 0, 0]);
        assert_eq!(super::do_mix(&[-15, 0, 0]), vec![0, -15, 0]);
        assert_eq!(super::do_mix(&[0, -1, 0]), vec![-1, 0, 0]);
        assert_eq!(super::do_mix(&[0, -2, 0]), vec![0, -2, 0]);
        assert_eq!(super::do_mix(&[0, -3, 0]), vec![-3, 0, 0]);
        assert_eq!(super::do_mix(&[0, 0, -1]), vec![0, -1, 0]);
        assert_eq!(super::do_mix(&[0, 0, -2]), vec![-2, 0, 0]);
        assert_eq!(super::do_mix(&[0, 0, -3]), vec![0, -3, 0]);
        assert_eq!(super::do_mix(&[0, 0, -4]), vec![-4, 0, 0]);
        assert_eq!(super::do_mix(&[0, 0, -5]), vec![0, -5, 0]);
        assert_eq!(super::do_mix(&[0, 0, -6]), vec![-6, 0, 0]);
        assert_eq!(super::do_mix(&[0, 0, -15]), vec![0, -15, 0]);
    }

    #[test]
    fn test_part1() {
        let sum = super::part1();
        println!("Sum: {}", sum);
        assert_eq!(sum, 11123);
    }

    #[test]
    fn test_part2() {
        let sum = super::part2();
        println!("Sum: {}", sum);
        assert_eq!(sum, 4248669215955);
    }
}
