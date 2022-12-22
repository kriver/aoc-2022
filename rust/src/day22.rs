use std::collections::HashMap;

use crate::util::load;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Tile {
    Void,
    Open,
    Wall,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Map {
    warps: Warps,
    grid: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
    pos: Coord,
    dir: usize,
}

enum StepResult {
    Stepped,
    HitWall,
    Nothing,
}

type Warps = HashMap<(Coord, usize), (Coord, usize)>;

impl Map {
    // Cube connecting edges:
    // .-------.  .----.
    // | .----[#][#]-. |
    // | |  .-[#]-'  | |
    // | '-[#][#]----' |
    // '---[#]-'       |
    //      `----------'
    fn warps(cube: bool) -> Warps {
        match cube {
            true => {
                let mut warps = HashMap::new();
                for y in 0..50 {
                    warps.insert((Coord { x: 50, y }, 2), (Coord { x: 0, y: 149 - y }, 0));
                    warps.insert((Coord { x: 149, y }, 0), (Coord { x: 99, y: 149 - y }, 2));
                }
                for y in 50..100 {
                    warps.insert((Coord { x: 50, y }, 2), (Coord { x: y - 50, y: 100 }, 1));
                    warps.insert((Coord { x: 99, y }, 0), (Coord { x: y + 50, y: 49 }, 3));
                }
                for y in 100..150 {
                    warps.insert((Coord { x: 0, y }, 2), (Coord { x: 50, y: 149 - y }, 0));
                    warps.insert((Coord { x: 99, y }, 0), (Coord { x: 149, y: 149 - y }, 2));
                }
                for y in 150..200 {
                    warps.insert((Coord { x: 0, y }, 2), (Coord { x: y - 100, y: 0 }, 1));
                    warps.insert((Coord { x: 49, y }, 0), (Coord { x: y - 100, y: 149 }, 3));
                }
                for x in 0..50 {
                    warps.insert((Coord { x, y: 100 }, 3), (Coord { x: 50, y: x + 50 }, 0));
                    warps.insert((Coord { x, y: 199 }, 1), (Coord { x: x + 100, y: 0 }, 1));
                }
                for x in 50..100 {
                    warps.insert((Coord { x, y: 0 }, 3), (Coord { x: 0, y: x + 100 }, 0));
                    warps.insert((Coord { x, y: 149 }, 1), (Coord { x: 49, y: x + 100 }, 2));
                }
                for x in 100..150 {
                    warps.insert((Coord { x, y: 0 }, 3), (Coord { x: x - 100, y: 199 }, 3));
                    warps.insert((Coord { x, y: 49 }, 1), (Coord { x: 99, y: x - 50 }, 2));
                }
                warps
            }
            false => HashMap::new(),
        }
    }

    fn load(lines: &[String], cube: bool) -> Self {
        let mut grid: Vec<Vec<Tile>> = lines
            .iter()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        ' ' => Tile::Void,
                        '.' => Tile::Open,

                        '#' => Tile::Wall,
                        _ => panic!("invalid tile '{}'", c),
                    })
                    .collect()
            })
            .collect();
        let width = grid[0].len();
        // make sure all rows are as long as the first one (luckily the longest)
        for row in grid.iter_mut() {
            if row.len() < width {
                row.append(&mut vec![Tile::Void; width - row.len()]);
            }
        }
        let x = grid[0].iter().position(|t| *t == Tile::Open).unwrap();
        let warps = Map::warps(cube);
        Map {
            warps,
            width,
            height: grid.len(),
            grid,
            pos: Coord { x, y: 0 },
            dir: 0,
        }
    }

    fn turn_right(&mut self) {
        self.dir = (self.dir + 1) % 4;
    }

    fn turn_left(&mut self) {
        self.dir = (self.dir + 3) % 4;
    }

    fn delta_step(&self) -> (usize, usize) {
        match self.dir {
            0 => (1, 0),
            1 => (0, 1),
            2 => (self.width - 1, 0),
            3 => (0, self.height - 1),
            _ => unreachable!(),
        }
    }

    fn step_one(&mut self, Coord { x, y }: Coord) -> Coord {
        let (dx, dy) = self.delta_step();
        match self.move_void() {
            Some(p) => p,
            None => {
                let nx = (x + dx) % self.width;
                let ny = (y + dy) % self.height;
                Coord { x: nx, y: ny }
            }
        }
    }

    fn move_void(&mut self) -> Option<Coord> {
        let key = (self.pos, self.dir);
        match self.warps.contains_key(&key) {
            true => {
                let w = self.warps[&key];
                if let Tile::Open = self.grid[w.0.y][w.0.x] {
                    self.dir = w.1; // only turn if not hitting a wall
                }
                Some(w.0)
            }
            false => None,
        }
    }

    // fn display(&self, trail: &HashMap<Coord, usize>) {
    //     for y in 0..self.height {
    //         for x in 0..self.width {
    //             let pos = Coord { x, y };
    //             if trail.contains_key(&pos) {
    //                 match trail[&pos] {
    //                     0 => print!(">"),
    //                     1 => print!("v"),
    //                     2 => print!("<"),
    //                     3 => print!("^"),
    //                     _ => panic!(),
    //                 }
    //             } else {
    //                 match self.grid[pos.y][pos.x] {
    //                     Tile::Void => print!(" "),
    //                     Tile::Open => print!("."),
    //                     Tile::Wall => print!("#"),
    //                 }
    //             }
    //         }
    //         println!("");
    //     }
    //     println!("\n");
    // }

    fn check_pos(&mut self, pos: &mut Coord, trail: &mut HashMap<Coord, usize>) -> StepResult {
        match self.grid[pos.y][pos.x] {
            Tile::Open => {
                self.pos = *pos;
                trail.insert(*pos, self.dir);
                StepResult::Stepped
            }
            Tile::Wall => StepResult::HitWall, // stop moving
            Tile::Void => match self.move_void() {
                Some(p) => {
                    (pos.x, pos.y) = (p.x, p.y);
                    self.check_pos(pos, trail)
                }
                None => StepResult::Nothing,
            },
        }
    }

    fn move_steps(&mut self, steps: u8, trail: &mut HashMap<Coord, usize>) {
        let mut pos = self.pos; // running coords
        let mut step = 0;
        while step < steps {
            pos = self.step_one(pos);
            match self.check_pos(&mut pos, trail) {
                StepResult::Stepped => step += 1,
                StepResult::HitWall => break,
                StepResult::Nothing => (),
            }
        }
    }

    fn password(&self) -> usize {
        1000 * (self.pos.y + 1) + 4 * (self.pos.x + 1) + self.dir
    }
}

#[derive(Debug)]
struct Notes {
    map: Map,
    path: String,
}

impl Notes {
    fn load(filename: &str, cube: bool) -> Self {
        let lines = load::<String>(filename);
        let last = lines.len() - 1;
        let path = lines[last].clone();
        let map = Map::load(&lines[0..last - 1], cube);
        Notes { map, path }
    }

    fn walk(&mut self) {
        let mut steps = 0;
        let mut trail = HashMap::new();
        for c in self.path.chars() {
            match c {
                '0'..='9' => steps = steps * 10 + (c as u8) - b'0',
                'R' => {
                    self.map.move_steps(steps, &mut trail);
                    steps = 0;
                    self.map.turn_right();
                }
                'L' => {
                    self.map.move_steps(steps, &mut trail);
                    steps = 0;
                    self.map.turn_left();
                }
                _ => panic!("invalid instruction"),
            }
        }
        if steps > 0 {
            self.map.move_steps(steps, &mut trail);
        }
    }
}

pub fn part1() -> usize {
    let mut notes = Notes::load("data/day22.txt", false);
    notes.walk();
    notes.map.password()
}

pub fn part2() -> usize {
    let mut notes = Notes::load("data/day22.txt", true);
    notes.walk();
    notes.map.password()
}

mod tests {
    #[test]
    fn test_part1() {
        let password = super::part1();
        println!("Password: {}", password);
        assert_eq!(password, 88226);
    }

    #[test]
    fn test_part2() {
        let password = super::part2();
        println!("Password: {}", password);
        assert_eq!(password, 57305);
    }
}
