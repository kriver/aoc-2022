use std::{collections::VecDeque, ops::Add};

use crate::util::load;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
}

impl Add for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Coord {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Node {
    pos: Coord,
    elev: u8,
    prev: Option<Coord>,
    dist: usize,
}

impl Node {
    fn unvisited(coord: &Coord, elev: u8) -> Self {
        Node {
            pos: *coord,
            elev: elev,
            prev: None,
            dist: usize::MAX,
        }
    }
}

struct Grid {
    grid: Vec<Vec<Node>>,
    width: usize,
    height: usize,
}

impl Grid {
    fn load(lines: Vec<String>) -> (Self, Coord, Coord, Vec<Coord>) {
        let mut low_points = vec![];
        let mut start = Coord { x: 0, y: 0 };
        let mut top = Coord { x: 0, y: 0 };
        let grid: Vec<Vec<Node>> = lines
            .into_iter()
            .enumerate()
            .map(|(y, l)| {
                l.as_bytes()
                    .into_iter()
                    .enumerate()
                    .map(|(x, elevation)| {
                        let c = Coord {
                            x: x as i32,
                            y: y as i32,
                        };
                        let e = match *elevation {
                            b'S' => {
                                start = c;
                                b'a'
                            }
                            b'E' => {
                                top = c;
                                b'z'
                            }
                            b'a' => {
                                low_points.push(c);
                                b'a'
                            }
                            b => b,
                        };
                        Node::unvisited(&c, e)
                    })
                    .collect()
            })
            .collect();
        let height = grid.len();
        let width = grid[0].len();
        (
            Grid {
                grid,
                width,
                height,
            },
            start,
            top,
            low_points,
        )
    }

    fn update(&mut self, Coord { x, y }: &Coord, prev: Option<Coord>, dist: usize) {
        self.grid[*y as usize][*x as usize].dist = dist;
        self.grid[*y as usize][*x as usize].prev = prev;
    }

    fn reset(&mut self) {
        for y in 0..self.height as i32 {
            for x in 0..self.width as i32 {
                self.update(&Coord { x, y }, None, usize::MAX);
            }
        }
    }

    fn node<'a>(&'a self, Coord { x, y }: &Coord) -> &'a Node {
        &self.grid[*y as usize][*x as usize]
    }

    fn elevation(&self, coord: &Coord) -> i16 {
        self.node(coord).elev as i16
    }

    fn is_valid(&self, coord: &Coord) -> bool {
        (coord.x >= 0)
            && (coord.x < self.width as i32)
            && (coord.y >= 0)
            && (coord.y < self.height as i32)
    }

    fn can_move(&self, from: &Coord, to: &Coord) -> bool {
        self.is_valid(to) && (self.elevation(to) - self.elevation(from) <= 1)
    }

    /* Dijkstra */
    fn find_top(&mut self, start: &Coord, top: &Coord) -> usize {
        let neighbours = [
            Coord { x: -1, y: 0 },
            Coord { x: 1, y: 0 },
            Coord { x: 0, y: -1 },
            Coord { x: 0, y: 1 },
        ];
        let mut queue = VecDeque::from([*start]);
        self.update(&queue[0], None, 0);
        loop {
            if queue.is_empty() {
                return usize::MAX;
            }
            let pos = queue.pop_front().unwrap();
            let prev_dist = self.node(&pos).dist;
            if pos == *top {
                return prev_dist;
            }
            for delta in neighbours {
                let new_pos = pos + delta;
                if self.can_move(&pos, &new_pos) {
                    let dist = prev_dist + 1;
                    if dist < self.node(&new_pos).dist {
                        self.update(&new_pos, Some(pos), dist);
                        queue.push_back(new_pos);
                    }
                }
            }
        }
    }
}

struct Map {
    grid: Grid,
    low_points: Vec<Coord>,
    start: Coord,
    top: Coord,
}

impl Map {
    fn load(filename: &str) -> Self {
        let lines: Vec<String> = load(filename);
        let (grid, start, top, low_points) = Grid::load(lines);
        Self {
            grid,
            low_points,
            start,
            top,
        }
    }
}

pub fn part1() -> usize {
    let mut map = Map::load("data/day12.txt");
    let start = map.start;
    let top = map.top;
    map.grid.find_top(&start, &top)
}

pub fn part2() -> usize {
    let mut map = Map::load("data/day12.txt");
    let top = map.top;
    let mut shortest = usize::MAX;
    for start in map.low_points.iter() {
        map.grid.reset();
        shortest = shortest.min(map.grid.find_top(start, &top));
    }
    shortest
}

mod tests {
    #[test]
    fn test_part1() {
        let steps = super::part1();
        println!("Number of steps: {}", steps);
        assert_eq!(steps, 394);
    }

    #[test]
    fn test_part2() {
        let steps = super::part2();
        println!("Number of steps: {}", steps);
        assert_eq!(steps, 388);
    }
}
