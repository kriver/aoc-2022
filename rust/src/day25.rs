use crate::util::load;

fn snafu(s: &str) -> i64 {
    let mut num = 0;
    let mut multiplier = 1;
    for c in s.chars().rev() {
        match c {
            '2' => num += 2 * multiplier,
            '1' => num += 1 * multiplier,
            '0' => num += 0 * multiplier,
            '-' => num += -1 * multiplier,
            '=' => num += -2 * multiplier,
            _ => unreachable!(),
        }
        multiplier *= 5;
    }
    num
}

fn ufans(mut n: i64) -> String {
    fn push(s: &mut Vec<char>, n: i64) -> i64 {
        match n % 5 {
            0 => {
                s.push('0');
                0
            }
            1 => {
                s.push('1');
                0
            }
            2 => {
                s.push('2');
                0
            }
            3 => {
                s.push('=');
                1
            }
            4 => {
                s.push('-');
                1
            }
            _ => unreachable!(),
        }
    }
    let mut s = Vec::new();
    let mut overflow = 0;
    while n > 0 {
        overflow = push(&mut s, n + overflow);
        n /= 5;
    }
    if overflow != 0 {
        push(&mut s, overflow);
    }
    s.iter().rev().collect()
}

pub fn part1() -> String {
    let n = load::<String>("data/day25.txt")
        .into_iter()
        .map(|s| snafu(&s))
        .sum();
    ufans(n)
}

mod tests {
    #[test]
    fn test_part1() {
        let num = super::part1();
        println!("SNAFU number: {}", num);
        assert_eq!(num, "2-21=02=1-121-2-11-0");
    }
}
