use std::{collections::HashMap, str::FromStr};

use crate::util::load;

#[derive(Debug, Hash, PartialEq, Eq)]
struct Coord {
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

enum Type {
    Rock,
    Sand,
}

struct Cave {
    grid: HashMap<Coord, Type>,
    lowest: i32,
    floor: bool,
    units: usize,
}

fn load_cave(floor: bool) -> Cave {
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

impl Cave {
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

    fn insert(&mut self, sand: Coord, show: bool) -> Option<Coord> {
        self.units += 1;
        let y = sand.y;
        self.grid.insert(sand, Type::Sand);
        if show {
            self.display(y as usize);
        }
        if self.floor && y == 0 {
            None
        } else {
            Some(Coord { x: 500, y: 0 })
        }
    }

    fn sandfall(&mut self, show: bool) {
        let mut sand = Coord { x: 500, y: 0 };
        loop {
            if sand.y > self.lowest {
                if !self.floor {
                    break;
                } else {
                    sand = match self.insert(sand, show) {
                        None => break,
                        Some(s) => s,
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
                sand = match self.insert(sand, show) {
                    None => break,
                    Some(s) => s,
                };
            }
        }
    }
}
pub fn part1() -> usize {
    let mut cave = load_cave(false);
    cave.sandfall(false);
    cave.units
}

pub fn part2() -> usize {
    let mut cave = load_cave(true);
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
