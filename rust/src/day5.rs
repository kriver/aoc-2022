use std::str::FromStr;

use crate::util::load;

enum State {
    STACKS,
    MOVES,
}

type Crates = [Vec<char>; 9];

#[derive(Debug)]
struct Move {
    num: usize,
    from: usize,
    to: usize,
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.split_whitespace().collect();
        Ok(Move {
            num: tokens[1].parse().unwrap(),
            from: tokens[3].parse::<usize>().unwrap() - 1,
            to: tokens[5].parse::<usize>().unwrap() - 1,
        })
    }
}

fn load_and_parse() -> (Crates, Vec<Move>) {
    fn add_crates(crates: &mut Crates, line: &[u8]) {
        for i in 0..9 {
            let idx = 4 * i + 1;
            let c = line[idx];
            if idx < line.len() && c >= b'A' && c <= b'Z' {
                crates[i].insert(0, c as char);
            }
        }
    }

    let lines: Vec<String> = load("data/day5.txt");
    let mut crates: Crates = Default::default();
    let mut moves: Vec<Move> = Vec::new();
    let mut state = State::STACKS;
    for line in lines.into_iter() {
        if line.len() == 0 {
            state = State::MOVES;
            continue;
        }
        match state {
            State::STACKS => add_crates(&mut crates, line.as_bytes()),
            State::MOVES => moves.push(line.parse().unwrap()),
        }
    }
    (crates, moves)
}

fn top_crates(crates: &mut Crates) -> String {
    crates
        .into_iter()
        .map(|stack| stack.pop().unwrap())
        .collect()
}

pub fn part1() -> String {
    let (mut crates, moves) = load_and_parse();
    for m in moves.into_iter() {
        for _ in 0..m.num {
            let c = crates[m.from].pop().unwrap();
            crates[m.to].push(c);
        }
    }
    top_crates(&mut crates)
}

pub fn part2() -> String {
    let (mut crates, moves) = load_and_parse();
    for m in moves.into_iter() {
        let l = crates[m.from].len();
        let c = crates[m.from].split_off(l - m.num);
        crates[m.to].extend_from_slice(&c);
    }
    top_crates(&mut crates)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let top = part1();
        println!("Top crates are {}", top);
        assert_eq!(top, "VPCDMSLWJ");
    }

    #[test]
    fn test_part2() {
        let top = part2();
        println!("Top crates are {}", top);
        assert_eq!(top, "TPWCGNCCG");
    }
}
