use std::cmp::Ordering;

use crate::util::load;

#[derive(Debug)]
enum Data {
    Single(u8),
    Multi(Vec<Data>),
}

impl Data {
    fn parse_it<'a>(it: &mut impl Iterator<Item = &'a u8>) -> Data {
        fn push_some(v: &mut Vec<Data>, n: Option<u8>) -> Option<u8> {
            if let Some(val) = n {
                v.push(Data::Single(val));
            }
            None
        }

        let mut v = Vec::new();
        let mut n: Option<u8> = None;
        loop {
            match &it.next() {
                None => return v.remove(0),
                Some(b'[') => v.push(Data::parse_it(it)),
                Some(b']') => {
                    drop(push_some(&mut v, n));
                    return Data::Multi(v);
                }
                Some(b',') => n = push_some(&mut v, n),
                Some(d) => {
                    n = n.or(Some(0)).map(|val| val * 10 + (*d - b'0'));
                }
            }
        }
    }

    fn parse(s: &str) -> Data {
        Data::parse_it(&mut s.as_bytes().into_iter())
    }
}

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        fn to_multi(s: &u8) -> Data {
            Data::Multi(vec![Data::Single(*s)])
        }

        let r = match self {
            Data::Single(s1) => match other {
                Data::Single(s2) => s1.partial_cmp(s2),
                Data::Multi(_m2) => to_multi(s1).partial_cmp(other),
            },
            Data::Multi(m1) => match other {
                Data::Single(s2) => self.partial_cmp(&to_multi(s2)),
                Data::Multi(m2) => {
                    for i in 0..m1.len() {
                        if i >= m2.len() {
                            // all elements equal, and still more in lhs
                            return Some(Ordering::Greater);
                        }
                        match m1[i].partial_cmp(&m2[i]) {
                            Some(Ordering::Less) => return Some(Ordering::Less),
                            Some(Ordering::Greater) => return Some(Ordering::Greater),
                            _ => (), // equal, so check next element
                        }
                    }
                    // less (or equal number of) elements in lhs than rhs
                    m1.len().partial_cmp(&m2.len())
                }
            },
        };
        r
    }
}

impl Ord for Data {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialEq for Data {
    fn eq(&self, other: &Self) -> bool {
        if let Some(Ordering::Equal) = self.partial_cmp(other) {
            true
        } else {
            false
        }
    }
}

impl Eq for Data {}

pub fn part1() -> usize {
    let lines: Vec<String> = load("data/day13.txt");
    let mut sum = 0;
    let mut index = 1;
    loop {
        if index * 3 >= lines.len() {
            break;
        }
        let d1 = Data::parse(&lines[index * 3 - 3]);
        let d2 = Data::parse(&lines[index * 3 - 2]);
        let correct_order = d1 < d2;
        if correct_order {
            sum += index;
        }
        index += 1;
    }
    sum
}

pub fn part2() -> usize {
    let mut packets: Vec<Data> = load::<String>("data/day13.txt")
        .into_iter()
        .filter(|l| !l.is_empty())
        .map(|l| Data::parse(&l))
        .collect();
    packets.push(Data::parse("[[2]]"));
    packets.push(Data::parse("[[6]]"));
    packets.sort();
    // find divider packets and calculate decoder key
    let mut decoder_key = 1;
    let d1 = Data::parse("[[2]]");
    let d2 = Data::parse("[[6]]");
    for (i, p) in packets.into_iter().enumerate() {
        if p == d1 {
            decoder_key *= i + 1;
        } else if p == d2 {
            decoder_key *= i + 1;
        }
    }
    decoder_key
}

mod tests {
    #[test]
    fn unit_test_parse() {
        let d = super::Data::parse("[10]");
        let s = format!("{:?}", d);
        assert_eq!(s, "Multi([Single(10)])");
    }

    #[test]
    fn unit_test_cmp() {
        let d1 = super::Data::parse("[1,1,3,1,1]");
        let d2 = super::Data::parse("[1,1,5,1,1]");
        assert!(d1 < d2); // 1
        let d1 = super::Data::parse("[[1],[2,3,4]]");
        let d2 = super::Data::parse("[[1],4]");
        assert!(d1 < d2); // 2
        let d1 = super::Data::parse("[9]");
        let d2 = super::Data::parse("[[8,7,6]]");
        assert!(!(d1 < d2)); // 3
        let d1 = super::Data::parse("[[4,4],4,4]");
        let d2 = super::Data::parse("[[4,4],4,4,4]");
        assert!(d1 < d2); // 4
        let d1 = super::Data::parse("[7,7,7,7]");
        let d2 = super::Data::parse("[7,7,7]");
        assert!(!(d1 < d2)); // 5
        let d1 = super::Data::parse("[]");
        let d2 = super::Data::parse("[3]");
        assert!(d1 < d2); // 6
        let d1 = super::Data::parse("[[[]]]");
        let d2 = super::Data::parse("[[]]");
        assert!(!(d1 < d2)); // 7
        let d1 = super::Data::parse("[1,[2,[3,[4,[5,6,7]]]],8,9]");
        let d2 = super::Data::parse("[1,[2,[3,[4,[5,6,0]]]],8,9]");
        assert!(!(d1 < d2)); // 8
    }

    #[test]
    fn test_part1() {
        let sum = super::part1();
        println!("Index-sum of right order pairs: {}", sum);
        assert_eq!(sum, 5625);
    }

    #[test]
    fn test_part2() {
        let decoder_key = super::part2();
        println!("Decoder key: {}", decoder_key);
        assert_eq!(decoder_key, 23111);
    }
}
