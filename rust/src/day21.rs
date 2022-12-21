use std::collections::HashMap;

use crate::util::load;

#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operation {
    fn operate(&self, a: u64, b: u64) -> u64 {
        match self {
            Operation::Add => a + b,
            Operation::Sub => a - b,
            Operation::Mul => a * b,
            Operation::Div => a / b,
        }
    }
}

#[derive(Debug)]
enum Monkey {
    Number(u64),
    Op(Operation, String, String),
}

impl Monkey {
    fn deconstruct<'a>(&'a self) -> (&'a Operation, &'a str, &'a str) {
        match self {
            Monkey::Number(_) => panic!(),
            Monkey::Op(o, a, b) => (&o, &a, &b),
        }
    }
}

fn parse_line(s: &str) -> (String, Monkey) {
    let tokens: Vec<&str> = s.split([':', ' ']).collect();
    let name = tokens[0].to_owned();
    let mt = if tokens.len() == 3 {
        Monkey::Number(tokens[2].parse().unwrap())
    } else {
        let op = match tokens[3] {
            "+" => Operation::Add,
            "-" => Operation::Sub,
            "*" => Operation::Mul,
            "/" => Operation::Div,
            _ => panic!("unknown operation"),
        };
        Monkey::Op(op, tokens[2].to_owned(), tokens[4].to_owned())
    };
    (name, mt)
}

type MonkeyMap = HashMap<String, Monkey>;
type NumberMap = HashMap<String, u64>;

fn load_monkeys(filename: &str) -> (NumberMap, MonkeyMap) {
    let monkeys: MonkeyMap = load::<String>(filename)
        .into_iter()
        .map(|s| parse_line(&s))
        .collect();
    let (nm, om): (MonkeyMap, MonkeyMap) = monkeys.into_iter().partition(|(_name, m)| match m {
        Monkey::Number(_) => true,
        Monkey::Op(_, _, _) => false,
    });
    let nm = nm
        .into_iter()
        .map(|(name, m)| (name, if let Monkey::Number(n) = m { n } else { 0 }))
        .collect();
    (nm, om)
}

fn operate(name: &str, m: &Monkey, numbers: &mut NumberMap) -> bool {
    match &m {
        Monkey::Number(_) => true,
        Monkey::Op(op, a, b) => {
            if numbers.contains_key(a) && numbers.contains_key(b) {
                numbers.insert(name.to_owned(), op.operate(numbers[a], numbers[b]));
                false
            } else {
                true
            }
        }
    }
}

fn yell(monkeys: &mut MonkeyMap, numbers: &mut NumberMap) {
    loop {
        let len = monkeys.len();
        monkeys.retain(|n, m| operate(n, m, numbers));
        if len == monkeys.len() {
            break; // no more progress
        }
    }
}

fn reverse_yell(name: &str, num: u64, monkeys: &MonkeyMap, numbers: &NumberMap) -> u64 {
    if name == "humn" {
        return num;
    }
    let (op, a, b) = monkeys[name].deconstruct();
    if numbers.contains_key(a) {
        match op {
            Operation::Add => reverse_yell(b, num - numbers[a], monkeys, numbers),
            Operation::Sub => reverse_yell(b, numbers[a] - num, monkeys, numbers),
            Operation::Mul => reverse_yell(b, num / numbers[a], monkeys, numbers),
            Operation::Div => reverse_yell(b, numbers[a] / num, monkeys, numbers),
        }
    } else if numbers.contains_key(b) {
        match op {
            Operation::Add => reverse_yell(a, num - numbers[b], monkeys, numbers),
            Operation::Sub => reverse_yell(a, num + numbers[b], monkeys, numbers),
            Operation::Mul => reverse_yell(a, num / numbers[b], monkeys, numbers),
            Operation::Div => reverse_yell(a, num * numbers[b], monkeys, numbers),
        }
    } else {
        panic!("deadlock")
    }
}

pub fn part1() -> u64 {
    let (mut numbers, mut monkeys) = load_monkeys("data/day21.txt");
    yell(&mut monkeys, &mut numbers);
    numbers["root"]
}

pub fn part2() -> u64 {
    let (mut numbers, mut monkeys) = load_monkeys("data/day21.txt");
    let _human = numbers.remove("humn").unwrap();
    let root = monkeys.remove("root").unwrap();
    // forward partial solve
    yell(&mut monkeys, &mut numbers);
    // reverse solve rest
    let (_, ma, mb) = root.deconstruct();
    if numbers.contains_key(ma) {
        reverse_yell(mb, numbers[ma], &monkeys, &numbers)
    } else {
        reverse_yell(ma, numbers[mb], &monkeys, &numbers)
    }
}

mod tests {
    #[test]
    fn test_part1() {
        let num = super::part1();
        println!("Number: {}", num);
        assert_eq!(num, 83056452926300);
    }

    #[test]
    fn test_part2() {
        let num = super::part2();
        println!("Number: {}", num);
        assert_eq!(num, 3469704905529);
    }
}
