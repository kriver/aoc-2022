use std::fmt::{Display, Formatter, Result};

use crate::util::load;

#[derive(Clone, Copy)]
struct ComSys {
    cycle: usize,
    sprite: i32,
    crt: [char; 240],
}

impl ComSys {
    fn new() -> Self {
        Self {
            cycle: 0,
            sprite: 1,
            crt: ['.'; 6 * 40],
        }
    }

    fn update_crt(&mut self) {
        let is_visible = ((self.cycle as i32) % 40 - self.sprite).abs() < 2;
        self.crt[self.cycle % 240] = if is_visible { '#' } else { '.' };
    }

    fn process(&mut self, cmd: &str) {
        let tokens: Vec<&str> = cmd.split(' ').collect();
        self.update_crt();
        match tokens[0] {
            "noop" => self.cycle += 1,
            "addx" => {
                self.cycle += 1;
                self.update_crt();
                self.sprite += tokens[1].parse::<i32>().unwrap();
                self.cycle += 1;
            }
            _ => panic!("unexpected command"),
        }
    }
}

impl Display for ComSys {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for line in self.crt.chunks(40) {
            writeln!(f, "{}", line.iter().collect::<String>())?;
        }
        Ok(())
    }
}

pub fn part1and2() -> i32 {
    let lines: Vec<String> = load("data/day10.txt");
    let mut sum = 0;
    let mut signal_cycle = 20;
    let mut cs = ComSys::new();
    for line in lines {
        let old = cs;
        cs.process(&line);
        if cs.cycle >= signal_cycle {
            let signal = old.sprite
                * (if cs.cycle == signal_cycle {
                    cs.cycle
                } else {
                    cs.cycle - 1
                }) as i32;
            sum += signal;
            signal_cycle += 40;
        }
    }
    println!("{}", cs);
    sum
}

mod tests {
    #[test]
    fn test_part1and2() {
        let sum = super::part1and2();
        println!("Signal strength sum: {}", sum);
        assert_eq!(sum, 14420);
    }
}
