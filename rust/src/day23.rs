use std::{
    collections::{HashMap, HashSet},
    ops::Add,
};

use crate::util::load;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
}

impl Add for Coord {
    type Output = Coord;

    fn add(self, rhs: Self) -> Self::Output {
        Coord {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug)]
struct Grid {
    tl: Coord, // top left corner
    br: Coord, // bottom right corner
    elves: HashSet<Coord>,
    look_idx: usize,
    look: HashMap<Coord, Vec<usize>>,
}

impl Grid {
    fn load(filename: &str) -> Self {
        let lines = load::<String>(filename);
        let mut tl = Coord { x: 0, y: 0 };
        let mut br = Coord { x: 0, y: 0 };
        let mut elves = HashSet::new();
        for (y, line) in lines.into_iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => {
                        tl.x = tl.x.min(x as i32);
                        br.x = br.x.max(x as i32);
                        tl.y = tl.y.min(y as i32);
                        br.y = br.y.max(y as i32);
                        elves.insert(Coord {
                            x: x as i32,
                            y: y as i32,
                        });
                    }
                    _ => (),
                }
            }
        }
        let look = HashMap::from([
            (Coord { x: -1, y: -1 }, vec![0, 2]),
            (Coord { x: 0, y: -1 }, vec![0]),
            (Coord { x: 1, y: -1 }, vec![0, 3]),
            (Coord { x: 1, y: 0 }, vec![3]),
            (Coord { x: 1, y: 1 }, vec![1, 3]),
            (Coord { x: 0, y: 1 }, vec![1]),
            (Coord { x: -1, y: 1 }, vec![1, 2]),
            (Coord { x: -1, y: 0 }, vec![2]),
        ]);
        Grid {
            tl,
            br,
            elves,
            look_idx: 0,
            look,
        }
    }

    fn empty(&self) -> usize {
        let size = (self.br.x - self.tl.x + 1) * (self.br.y - self.tl.y + 1);
        size as usize - self.elves.len()
    }

    // fn display(&self) {
    //     let width = self.br.x - self.tl.x + 1;
    //     let height = self.br.y - self.tl.y + 1;
    //     let mut lines = vec![vec!['.'; width as usize]; height as usize];
    //     for (Coord { x, y }, _elf) in &self.elves {
    //         let row = y - self.tl.y;
    //         let column = x - self.tl.x;
    //         lines[row as usize][column as usize] = '#';
    //     }
    //     for line in lines {
    //         println!("{}", line.into_iter().collect::<String>());
    //     }
    // }

    fn neighbours(&self, pos: &Coord) -> [usize; 4] {
        let mut counts = [0; 4];
        for (delta, indices) in &self.look {
            let np = *pos + *delta;
            if self.elves.contains(&np) {
                for index in indices {
                    counts[*index] += 1;
                }
            }
        }
        counts
    }

    fn new_coord(pos: &Coord, dir: usize) -> Coord {
        match dir {
            0 => Coord {
                x: pos.x,
                y: pos.y - 1,
            },
            1 => Coord {
                x: pos.x,
                y: pos.y + 1,
            },
            2 => Coord {
                x: pos.x - 1,
                y: pos.y,
            },
            3 => Coord {
                x: pos.x + 1,
                y: pos.y,
            },
            _ => unreachable!(),
        }
    }

    fn move_apart_once(&mut self) -> usize {
        let mut proposals: HashMap<Coord, Vec<Coord>> = HashMap::new();
        for coord in &self.elves {
            let nb = self.neighbours(coord);
            if nb.iter().sum::<usize>() == 0 {
                continue; // lonely elf
            }
            for l in 0..4 {
                let dir = (self.look_idx + l) % 4;
                if nb[dir] == 0 {
                    let np = Self::new_coord(coord, dir);
                    match proposals.contains_key(&np) {
                        true => drop(proposals.entry(np).and_modify(|v| v.push(*coord))),
                        false => drop(proposals.insert(np, vec![*coord])),
                    }
                    break; // found proposal
                }
            }
        }
        let mut moves = 0;
        for (dst, sources) in proposals {
            if sources.len() == 1 {
                self.elves.remove(&sources[0]);
                self.tl.x = self.tl.x.min(dst.x as i32);
                self.br.x = self.br.x.max(dst.x as i32);
                self.tl.y = self.tl.y.min(dst.y as i32);
                self.br.y = self.br.y.max(dst.y as i32);
                self.elves.insert(dst);
                moves += 1;
            }
        }
        self.look_idx = (self.look_idx + 1) % 4;
        moves
    }

    fn move_rounds(&mut self, rounds: usize) {
        for _ in 0..rounds {
            self.move_apart_once();
        }
    }

    fn move_expand(&mut self) -> usize {
        let mut rounds = 0;
        loop {
            rounds += 1;
            if 0 == self.move_apart_once() {
                break;
            }
        }
        rounds
    }
}

pub fn part1() -> usize {
    let mut grid = Grid::load("data/day23.txt");
    grid.move_rounds(10);
    grid.empty()
}

pub fn part2() -> usize {
    let mut grid = Grid::load("data/day23.txt");
    let rounds = grid.move_expand();
    rounds
}

mod tests {
    #[test]
    fn test_part1() {
        let num = super::part1();
        println!("Number of empty ground tiles: {}", num);
        assert_eq!(num, 4025);
    }

    #[test]
    fn test_part2() {
        let rounds = super::part2();
        println!("Number of rounds: {}", rounds);
        assert_eq!(rounds, 935);
    }
}
