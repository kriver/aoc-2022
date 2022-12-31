use std::{collections::HashMap, str::FromStr};

use crate::util::load;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Coord {
    x: i32,
    y: i32,
}

impl FromStr for Coord {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let token: Vec<i32> = s.split(',').map(|n| n.parse().unwrap()).collect();
        Ok(Coord {
            x: token[0],
            y: token[1],
        })
    }
}

impl Coord {
    pub fn new(x: i32, y: i32) -> Self {
        Coord { x, y }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    fn move_down(&mut self) {
        self.y += 1;
    }
    fn move_down_left(&mut self) {
        self.y += 1;
        self.x -= 1
    }
    fn move_down_right(&mut self) {
        self.y += 1;
        self.x += 1;
    }
}

pub enum Type {
    Rock,
    Sand,
}

pub enum Action {
    Falling(Coord),
    Settled(Coord, Coord),
    Done,
}

pub struct Cave {
    grid: HashMap<Coord, Type>,
    lowest: i32,
    floor: bool,
    units: usize,
}

impl Cave {
    pub fn load(floor: bool) -> Self {
        let lines: Vec<String> = load("data/day14.txt");
        let mut grid = HashMap::new();
        let mut lowest = 0;
        for line in lines {
            let mut prev: Option<Coord> = None;
            for point in line.split(" -> ") {
                let to: Coord = point.parse().unwrap();
                lowest = lowest.max(to.y);
                if let Some(from) = prev {
                    if from.x == to.x {
                        for y in from.y.min(to.y)..=to.y.max(from.y) {
                            grid.insert(
                                Coord {
                                    x: from.x,
                                    y: y as i32,
                                },
                                Type::Rock,
                            );
                        }
                    } else {
                        for x in from.x.min(to.x)..=to.x.max(from.x) {
                            grid.insert(
                                Coord {
                                    x: x as i32,
                                    y: from.y,
                                },
                                Type::Rock,
                            );
                        }
                    }
                }
                prev = Some(to);
            }
        }
        Cave {
            grid,
            lowest,
            floor,
            units: 0,
        }
    }

    pub fn grid_at(&self, x: i32, y: i32) -> Option<&Type> {
        self.grid.get(&Coord { x, y })
    }

    pub fn lowest(&self) -> i32 {
        self.lowest
    }

    fn free_below(&self, Coord { x, y }: &Coord) -> Vec<bool> {
        ((x - 1)..=(x + 1))
            .into_iter()
            .map(|x| Coord { x, y: y + 1 })
            .map(|c| !self.grid.contains_key(&c))
            .collect()
    }

    fn display(&self, y: usize) {
        let mut c = [['.'; 65]; 190];
        for (Coord { x, y }, t) in &self.grid {
            match t {
                Type::Rock => c[*y as usize][(*x - 460) as usize] = '#',
                Type::Sand => c[*y as usize][(*x - 460) as usize] = 'o',
            }
        }
        c[0][500 - 460] = '@';
        for (i, l) in c[y - 10..y + 10].into_iter().enumerate() {
            println!(
                "{:3}/{:3} - {}",
                y - 10 + i,
                self.lowest,
                l.iter().collect::<String>()
            );
        }
    }

    fn insert(&mut self, sand: &Coord) -> Option<Coord> {
        self.units += 1;
        let y = sand.y;
        self.grid.insert(sand.clone(), Type::Sand);

        if self.floor && y == 0 {
            None
        } else {
            Some(Coord { x: 500, y: 0 })
        }
    }

    pub fn sandfall_single_step(&mut self, mut sand: Coord) -> Action {
        if sand.y > self.lowest {
            if !self.floor {
                return Action::Done;
            } else {
                return match self.insert(&sand) {
                    None => Action::Done,
                    Some(s) => Action::Settled(sand, s),
                };
            }
        }
        let free = self.free_below(&sand);
        if free[1] {
            sand.move_down();
        } else if free[0] {
            sand.move_down_left();
        } else if free[2] {
            sand.move_down_right();
        } else {
            return match self.insert(&sand) {
                None => Action::Done,
                Some(s) => Action::Settled(sand, s),
            };
        }
        Action::Falling(sand)
    }

    fn sandfall(&mut self, show: bool) {
        let mut sand = Coord { x: 500, y: 0 };
        loop {
            match self.sandfall_single_step(sand) {
                Action::Done => break,
                Action::Falling(s) | Action::Settled(_, s) => {
                    sand = s;
                    if show {
                        self.display(sand.y as usize);
                    }
                }
            }
        }
    }
}

pub fn part1() -> usize {
    let mut cave = Cave::load(false);
    cave.sandfall(false);
    cave.units
}

pub fn part2() -> usize {
    let mut cave = Cave::load(true);
    cave.sandfall(false);
    cave.units
}

mod tests {
    #[test]
    fn test_part1() {
        let units = super::part1();
        println!("Units of sand a rest: {}", units);
        assert_eq!(units, 592);
    }

    #[test]
    fn test_part2() {
        let units = super::part2();
        println!("Units of sand a rest: {}", units);
        assert_eq!(units, 30367);
    }
}
