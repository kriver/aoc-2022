use std::collections::HashSet;

use crate::util::load;

enum Dir {
    Left,
    Right,
    Down,
}

struct Rock {
    move_left: Vec<(i64, i64)>,
    move_right: Vec<(i64, i64)>,
    move_down: Vec<(i64, i64)>,
    shape: Vec<(i64, i64)>,
}

fn rocks() -> Vec<Rock> {
    vec![
        // ####
        Rock {
            move_left: vec![(-1, 0)],
            move_right: vec![(4, 0)],
            move_down: vec![(0, -1), (1, -1), (2, -1), (3, -1)],
            shape: vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        },
        //  #
        // ###
        //  #
        Rock {
            move_left: vec![(0, 0), (-1, 1), (0, 2)],
            move_right: vec![(2, 0), (3, 1), (2, 2)],
            move_down: vec![(0, 0), (1, -1), (2, 0)],
            shape: vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
        },
        //   #
        //   #
        // ###
        Rock {
            move_left: vec![(-1, 0), (1, 1), (1, 2)],
            move_right: vec![(3, 0), (3, 1), (3, 2)],
            move_down: vec![(0, -1), (1, -1), (2, -1)],
            shape: vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        },
        // #
        // #
        // #
        // #
        Rock {
            move_left: vec![(-1, 0), (-1, 1), (-1, 2), (-1, 3)],
            move_right: vec![(1, 0), (1, 1), (1, 2), (1, 3)],
            move_down: vec![(0, -1)],
            shape: vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        },
        // ##
        // ##
        Rock {
            move_left: vec![(-1, 0), (-1, 1)],
            move_right: vec![(2, 0), (2, 1)],
            move_down: vec![(0, -1), (1, -1)],
            shape: vec![(0, 0), (1, 0), (0, 1), (1, 1)],
        },
    ]
}

struct Cave {
    jet: Vec<char>,
    rocks: HashSet<i64>,
    gas: usize,
    shape: usize,
    max_y: i64,
}

impl Cave {
    fn load(filename: &str) -> Self {
        let jet: Vec<char> = load::<String>(filename)[0].chars().collect();
        Cave {
            jet,
            rocks: HashSet::new(),
            gas: 0,
            shape: 0,
            max_y: 3,
        }
    }

    fn coord(x: i64, y: i64) -> i64 {
        (y << 3) + x
    }

    fn can_move(&self, rock: &Rock, x: i64, y: i64, dir: Dir) -> bool {
        match dir {
            Dir::Left => &rock.move_left,
            Dir::Right => &rock.move_right,
            Dir::Down => &rock.move_down,
        }
        .iter()
        .fold(true, |acc, (dx, dy)| {
            let (nx, ny) = (x + dx, y + dy);
            acc && (nx >= 0) && (nx <= 6) && (ny >= 0) && !self.rocks.contains(&Self::coord(nx, ny))
        })
    }

    // fn display(&self, y: i64) {
    //     for cy in (y - 2..y + 5).rev() {
    //         print!("{:3}  ", cy);
    //         for x in 0..7 {
    //             if cy < 0 {
    //                 print!("-");
    //             } else if self.rocks.contains(&Self::coord(x, cy)) {
    //                 print!("#");
    //             } else {
    //                 print!(".");
    //             }
    //         }
    //         println!();
    //     }
    // }

    pub fn rock_fall(&mut self, num: usize) -> i64 {
        let rocks = rocks();
        for _ in 0..num {
            let (mut x, mut y) = (2, self.max_y);
            loop {
                match self.jet[self.gas] {
                    '>' => {
                        if self.can_move(&rocks[self.shape], x, y, Dir::Right) {
                            x += 1;
                        }
                    }
                    '<' => {
                        if self.can_move(&rocks[self.shape], x, y, Dir::Left) {
                            x -= 1
                        }
                    }
                    _ => panic!("invalid gas"),
                }
                self.gas = (self.gas + 1) % self.jet.len();
                if self.can_move(&rocks[self.shape], x, y, Dir::Down) {
                    y -= 1;
                } else {
                    // settle rock
                    let s = &rocks[self.shape].shape;
                    for (dx, dy) in s {
                        self.rocks.insert(Self::coord(x + dx, y + dy));
                    }
                    self.max_y = self.max_y.max(y + s.last().unwrap().1 + 4);
                    self.shape = (self.shape + 1) % 5;
                    break;
                }
            }
        }
        self.max_y - 3
    }
}

pub fn part1() -> i64 {
    let mut cave = Cave::load("data/day17.txt");
    cave.rock_fall(2022)
}

pub fn part2() -> usize {
    fn find_pps(deltas: &Vec<usize>, cycle: usize, delta: usize) -> (Option<usize>, usize) {
        let mut pps = None;
        let mut prs = 0;
        let mut i: usize = cycle;
        while i > 1 {
            i -= 1;
            if deltas[i] == delta {
                pps = Some(i);
                prs = cycle;
                break;
            }
        }
        (pps, prs)
    }

    let mut cave = Cave::load("data/day17.txt");
    let mut deltas = Vec::new();
    let mut prev = 0;
    let times = 10; // expect at least this number of repeats
    let mut pps = None; // possible pattern start
    let mut prs = 0; // possible repeat start
    let mut cycle = 0;
    loop {
        // let rocks fall in multiples of 5 (one of each kind)
        cave.rock_fall(5);
        let d = (cave.max_y - prev) as usize;
        deltas.push(d);
        match pps {
            None => (pps, prs) = find_pps(&deltas, cycle, d),
            Some(start) => {
                let offset = cycle - prs;
                if deltas[start + offset] != d {
                    (pps, prs) = find_pps(&deltas, cycle, d);
                }
            }
        }
        // println!(
        //     "{:2} delta max = {:2}, pps = {:?}, prs = {}",
        //     cycle, d, pps, prs
        // );
        cycle += 1;
        if let Some(s) = pps {
            let len = prs - s; // pattern length
            if cycle == s + (times + 1) * len {
                break;
            }
        }
        prev = cave.max_y;
    }
    let start = pps.unwrap();
    println!(
        "Found {} repeats of {}..{} : {:?}",
        times,
        start,
        prs,
        &deltas[start..prs]
    );
    let rocks = 1000000000000 / 5; // max is recorded per 5 rocks
    let len = prs - start;
    let repeats = (rocks - start) / len;
    let remainder = rocks - start - repeats * len;
    println!(
        "Prefix = {}, repeats = {} * {} (with increase {}), suffix = {}",
        start,
        repeats,
        len * 5,
        deltas[start..prs].iter().sum::<usize>(),
        remainder
    );
    deltas[0..start].iter().sum::<usize>()
        + repeats * deltas[start..prs].iter().sum::<usize>()
        + deltas[start..(start + remainder)].iter().sum::<usize>()
        - 3 // why??
}

mod tests {
    #[test]
    fn test_part1() {
        let height = super::part1();
        println!("Tower height: {}", height);
        assert_eq!(height, 3133);
    }

    #[test]
    fn test_part2() {
        let height = super::part2();
        println!("Tower height: {}", height);
        assert_eq!(height, 1547953216393);
    }
}
