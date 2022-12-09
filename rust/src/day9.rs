use std::{collections::HashSet, str::FromStr};

use crate::util::load;

enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Dir {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Dir::Up),
            "D" => Ok(Dir::Down),
            "L" => Ok(Dir::Left),
            "R" => Ok(Dir::Right),
            _ => Err(()),
        }
    }
}

struct Move {
    dir: Dir,
    dist: i32,
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.split(' ').collect();
        Ok(Move {
            dir: tokens[0].parse().unwrap(),
            dist: tokens[1].parse().unwrap(),
        })
    }
}

fn delta_with_abs_max_1(d: i32) -> i32 {
    let signum = if d != 0 { d / d.abs() } else { 0 };
    signum * d.abs().min(1)
}
#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn move_1(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }

    /**
     * Either for (x,y) or (y,x)
     *   Delta    Move
     *   (0,0) -> (0,0)
     *   (1,0) -> (0,0)
     *   (1,1) -> (0,0)
     *   (2,0) -> (1,0)
     *   (2,1) -> (1,1)
     */
    fn move_1_closer_to(&mut self, other: &Self) {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        if dx.abs() > 1 || dy.abs() > 1 {
            self.x += delta_with_abs_max_1(dx);
            self.y += delta_with_abs_max_1(dy);
        }
    }
}

struct Rope {
    num_knots: usize,
    knots: Vec<Coord>,
    trail: HashSet<Coord>,
}

impl Rope {
    pub fn new(num_knots: usize) -> Self {
        Self {
            num_knots,
            knots: (0..num_knots).map(|_| Coord { x: 0, y: 0 }).collect(),
            trail: HashSet::from([Coord { x: 0, y: 0 }]),
        }
    }

    fn trail_length(&self) -> usize {
        self.trail.len()
    }

    fn move_head(&mut self, mv: &Move) {
        let (dx, dy) = match mv.dir {
            Dir::Up => (0, 1),
            Dir::Down => (0, -1),
            Dir::Left => (-1, 0),
            Dir::Right => (1, 0),
        };
        for _step in 0..mv.dist {
            self.knots[0].move_1(dx, dy);
            for i in 1..self.num_knots {
                let dst = self.knots[i - 1];
                self.knots[i].move_1_closer_to(&dst);
            }
            self.trail.insert(*self.knots.last().unwrap());
        }
    }
}

fn move_rope(moves: Vec<Move>, num_knots: usize) -> usize {
    let mut rope = Rope::new(num_knots);
    for m in moves {
        rope.move_head(&m);
    }
    rope.trail_length()
}

pub fn part1(filename: &str) -> usize {
    let moves: Vec<Move> = load(filename);
    move_rope(moves, 2)
}

pub fn part2(filename: &str) -> usize {
    let moves: Vec<Move> = load(filename);
    move_rope(moves, 10)
}

mod tests {
    #[test]
    fn test_part1() {
        let num = super::part1("data/day9.txt");
        println!("Number of visited positions 2-knot-rope: {}", num);
        assert_eq!(num, 6098);
    }

    #[test]
    fn test_part2() {
        let num = super::part2("data/day9.txt");
        println!("Number of visited positions 10-knot-rope: {}", num);
        assert_eq!(num, 2597);
    }
}
