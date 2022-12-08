use std::collections::hash_map::Entry;
use std::collections::HashMap;

use crate::util::load;

pub fn input() -> Vec<String> {
    load("data/day6.txt")
}

fn find_marker_end(s: &[u8], sz: usize) -> usize {
    let mut h: HashMap<u8, usize> = HashMap::new();
    for i in 0..s.len() {
        if i > sz - 1 {
            let e = h.entry(s[i - sz]).and_modify(|cnt| *cnt -= 1);
            if let Entry::Occupied(o) = e {
                if o.get() == &0 {
                    o.remove_entry();
                }
            }
        }
        h.entry(s[i]).and_modify(|cnt| *cnt += 1).or_insert(1);
        if h.len() == sz {
            return i;
        }
    }
    0
}

pub fn part1(stream: &str) -> usize {
    find_marker_end(stream.as_bytes(), 4) + 1 // 1-based indexing
}

pub fn part2(stream: &str) -> usize {
    find_marker_end(stream.as_bytes(), 14) + 1 // 1-based indexing
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);

        let lines = input();
        let start = part1(&lines[0]);
        println!("Start-of-packet marker at {}", start);
        assert_eq!(start, 1275);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(part2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);

        let lines = input();
        let start = part2(&lines[0]);
        println!("Start-of-message marker at {}", start);
        assert_eq!(start, 3605);
    }
}
