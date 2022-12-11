use std::collections::VecDeque;

use crate::util::load;

#[derive(Debug)]
enum Op {
    Nop,
    MulOld,
    MulNum(u64),
    AddOld,
    AddNum(u64),
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u64>,
    op: Op,
    inspect_count: u64,
    divisibility: u64,
    monkey_true: usize,
    monkey_false: usize,
}

impl Monkey {
    fn new() -> Self {
        Monkey {
            items: VecDeque::new(),
            op: Op::Nop,
            inspect_count: 0,
            divisibility: 0,
            monkey_true: 0,
            monkey_false: 0,
        }
    }

    fn inspect_item(&mut self, item: u64) -> u64 {
        self.inspect_count += 1;
        match self.op {
            Op::Nop => item,
            Op::MulOld => item * item,
            Op::MulNum(n) => item * n,
            Op::AddOld => item + item,
            Op::AddNum(n) => item + n,
        }
    }

    fn catch(&mut self, item: u64) {
        self.items.push_back(item);
    }
}

fn parse_input(filename: &str) -> Vec<Monkey> {
    enum ParseState {
        MONKEY,
        ITEMS,
        OPERATION,
        TEST,
        TRUE,
        FALSE,
        END,
    }

    let mut monkeys = Vec::new();
    let mut state = ParseState::MONKEY;

    let lines: Vec<String> = load(filename);
    let mut monkey = Monkey::new();
    for line in lines {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        state = match state {
            ParseState::MONKEY => {
                assert_eq!(tokens[0], "Monkey");
                ParseState::ITEMS
            }
            ParseState::ITEMS => {
                assert_eq!(tokens[0], "Starting");
                monkey.items = tokens[2..]
                    .into_iter()
                    .map(|s| s.split(',').next().unwrap().parse::<u64>().unwrap())
                    .collect::<VecDeque<u64>>();
                ParseState::OPERATION
            }
            ParseState::OPERATION => {
                assert_eq!(tokens[0], "Operation:");
                monkey.op = match tokens[5].parse::<u64>() {
                    Ok(rhs) => match tokens[4] {
                        "+" => Op::AddNum(rhs),
                        "*" => Op::MulNum(rhs),
                        _ => panic!("unknown operation"),
                    },
                    Err(_) => match tokens[4] {
                        "+" => Op::AddOld,
                        "*" => Op::MulOld,
                        _ => panic!("unknown operation"),
                    },
                };
                ParseState::TEST
            }
            ParseState::TEST => {
                assert_eq!(tokens[0], "Test:");
                monkey.divisibility = tokens[3].parse().unwrap();
                ParseState::TRUE
            }
            ParseState::TRUE => {
                assert_eq!(tokens[1], "true:");
                monkey.monkey_true = tokens[5].parse().unwrap();
                ParseState::FALSE
            }
            ParseState::FALSE => {
                assert_eq!(tokens[1], "false:");
                monkey.monkey_false = tokens[5].parse().unwrap();
                ParseState::END
            }
            ParseState::END => {
                assert!(tokens.len() == 0);
                monkeys.push(monkey);
                monkey = Monkey::new();
                ParseState::MONKEY
            }
        };
    }
    monkeys.push(monkey);
    monkeys
}

fn throw_to_monkey(target: usize, item: u64, ml: &mut [Monkey], mid: usize, mr: &mut [Monkey]) {
    assert_ne!(mid, target);
    let limited = item % (2 * 3 * 5 * 7 * 11 * 13 * 17 * 19);
    if target < mid {
        ml[target].catch(limited);
    } else {
        mr[target - mid - 1].catch(limited);
    }
}

fn play_round<F>(monkeys: &mut Vec<Monkey>, relief: F)
where
    F: Fn(u64) -> u64,
{
    for m in 0..monkeys.len() {
        let (ml, mr) = monkeys.split_at_mut(m);
        let (monkey, mr) = mr.split_first_mut().unwrap();
        for _ in 0..monkey.items.len() {
            let item = monkey.items.pop_front().unwrap();
            let level = relief(monkey.inspect_item(item));
            if level % monkey.divisibility == 0 {
                throw_to_monkey(monkey.monkey_true, level, ml, m, mr);
            } else {
                throw_to_monkey(monkey.monkey_false, level, ml, m, mr);
            }
        }
    }
}
fn play<F>(num_rounds: u64, relief: F) -> u64
where
    F: Fn(u64) -> u64,
{
    let mut monkeys = parse_input("data/day11.txt");
    for _ in 0..num_rounds {
        play_round(&mut monkeys, &relief);
    }
    let mut counts: Vec<u64> = monkeys.into_iter().map(|m| m.inspect_count).collect();
    counts.sort();
    counts.reverse();
    counts[0] * counts[1]
}

pub fn part1() -> u64 {
    play(20, |x| x / 3)
}

pub fn part2() -> u64 {
    play(10000, |x| x)
}

mod tests {
    #[test]
    fn test_part1() {
        let level = super::part1();
        println!("Monkey level: {}", level);
        assert_eq!(level, 78678);
    }

    #[test]
    fn test_part2() {
        let level = super::part2();
        println!("Monkey level: {}", level);
        assert_eq!(level, 15333249714);
    }
}
