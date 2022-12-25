use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

use crate::util::load;

type Coord = (usize, usize);
type Blizzards = HashMap<Coord, Vec<u8>>;
type BitMap = Vec<Vec<u64>>;

#[derive(Debug)]
struct Position {
    minutes: usize,
    coord: Coord,
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Position {
    // furthest first, least amount of time if equal
    fn cmp(&self, other: &Self) -> Ordering {
        let d1 = self.coord.0 + self.coord.1;
        let d2 = other.coord.0 + other.coord.1;
        match d1.cmp(&d2) {
            Ordering::Equal => self.minutes.cmp(&other.minutes).reverse(),
            ord => ord,
        }
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Position {}

#[derive(Debug)]
struct Map {
    blizzards: Blizzards,
    width: usize,  // of space occupied by blizzards
    height: usize, // of space occupied by blizzards
}

impl Map {
    fn load(filename: &str) -> Self {
        let lines = load::<String>(filename);
        let width = lines[0].len() - 2;
        let height = lines.len() - 2;
        let mut blizzards = HashMap::new();
        for (y, line) in lines.into_iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let dir = match c {
                    '#' | '.' => continue,
                    '<' | '>' | '^' | 'v' => c as u8,
                    _ => panic!(),
                };
                blizzards.insert((x - 1, y - 1), vec![dir]);
            }
        }
        Map {
            blizzards,
            width,
            height,
        }
    }

    fn move_blizzards(&mut self) {
        let mut blizzards: Blizzards = HashMap::new();
        for (c, dirs) in self.blizzards.iter() {
            for d in dirs.into_iter() {
                let coord = match d {
                    b'<' => ((c.0 + self.width - 1) % self.width, c.1),
                    b'>' => ((c.0 + 1) % self.width, c.1),
                    b'^' => (c.0, (c.1 + self.height - 1) % self.height),
                    b'v' => (c.0, (c.1 + 1) % self.height),
                    _ => unreachable!(),
                };
                blizzards
                    .entry(coord)
                    .and_modify(|v| v.push(*d))
                    .or_insert(vec![*d]);
            }
        }
        self.blizzards = blizzards;
    }
}

#[derive(Debug)]
struct MultiMap {
    grids: Vec<BitMap>,
    width: usize,  // of space occupied by blizzards
    height: usize, // of space occupied by blizzards
}

impl MultiMap {
    fn to_bit(x: usize) -> (usize, u64) {
        (x / 64, 1 << (x % 64))
    }

    fn build(mut map: Map, iterations: usize) -> Self {
        fn to_bitmap(map: &Map) -> BitMap {
            let sz = (map.width as f32 / 64.).ceil() as usize;
            let mut bm = vec![vec![0; sz]; map.height + 2];
            // first row is border with entry point, last with exit
            for w in 0..sz {
                bm[0][w] = u64::MAX;
                bm[map.height + 1][w] = u64::MAX;
            }
            bm[0][0] &= u64::MAX - 1;
            bm[map.height + 1][sz - 1] &= u64::MAX - (1 << (map.width - (sz - 1) * 64 - 1));
            // actual rows
            for ((x, y), _) in &map.blizzards {
                let (w, b) = MultiMap::to_bit(*x);
                bm[*y + 1][w] |= b;
            }
            bm
        }

        let mut grids: Vec<BitMap> = Vec::with_capacity(iterations);
        for _ in 0..iterations {
            map.move_blizzards();
            grids.push(to_bitmap(&map));
        }
        MultiMap {
            grids: grids,
            width: map.width,
            height: map.height,
        }
    }

    fn display(&self, x: usize, y: usize, minutes: usize, min: usize, sz: usize) {
        let g = minutes % self.grids.len();
        let grid = &self.grids[g];
        let offset = 64 * grid[0].len() - x;
        println!(
            " {:>offset$}     minutes passed: {} ({}), pos: {},{}, #q = {}",
            'v', minutes, min, x, y, sz
        );
        for (r, row) in grid.iter().enumerate() {
            print!("{}", if r == y { '>' } else { ' ' });
            for word in row.iter().rev() {
                print!("{:064b}", word);
            }
            println!()
        }
    }

    fn is_empty(&self, x: usize, y: usize, minutes: usize) -> bool {
        let g = minutes % self.grids.len();
        let (w, b) = MultiMap::to_bit(x);
        (self.grids[g][y][w] & b) == 0
    }

    fn possible_moves(&self, (x, y): Coord, minutes: usize) -> Vec<Coord> {
        let mut moves = vec![];
        if self.is_empty(x, y, minutes) {
            // don't move
            moves.push((x, y));
        }
        if x > 0 && self.is_empty(x - 1, y, minutes) {
            moves.push((x - 1, y));
        }
        if x < self.width - 1 && self.is_empty(x + 1, y, minutes) {
            moves.push((x + 1, y));
        }
        if y > 1 && self.is_empty(x, y - 1, minutes) {
            moves.push((x, y - 1));
        }
        if y < self.height + 1 && self.is_empty(x, y + 1, minutes) {
            moves.push((x, y + 1));
        }
        moves
    }

    fn find_path(&mut self) -> usize {
        let start = (0, 0);
        let finish = (self.width - 1, self.height + 1);
        let mut q = BinaryHeap::from([Position {
            minutes: 0,
            coord: start,
        }]);
        let mut minimum = usize::MAX;
        loop {
            match q.pop() {
                None => break,
                Some(Position {
                    minutes,
                    coord: (x, y),
                }) => {
                    if minutes + (self.width - 1 - x) + (self.height + 1 - y) >= minimum {
                        // all possible further paths from this will be too long
                        continue;
                    }
                    let moves = self.possible_moves((x, y), minutes);
                    if q.len() % 200000 == 0 {
                        self.display(x, y, minutes, minimum, q.len());
                        println!("Possible move: {:?}", moves);
                    }
                    for m in moves {
                        if m == finish {
                            // found a path
                            minimum = minimum.min(minutes + 1);
                        } else {
                            q.push(Position {
                                minutes: minutes + 1,
                                coord: m,
                            })
                        }
                    }
                }
            }
        }
        minimum
    }
}

pub fn part1() -> usize {
    // Grid repeats every 12 iterations for test
    // let mut mm = MultiMap::build(Map::load("data/day24-test.txt"), 12);
    // Grid repeats every 600 iterations for actual
    let mut mm = MultiMap::build(Map::load("data/day24.txt"), 600);
    mm.find_path()
}

mod tests {
    #[test]
    fn test_part1() {
        let minutes = super::part1();
        println!("Minutes needed: {}", minutes);
        assert_eq!(minutes, 0);
    }

    #[test]
    fn test_part2() {}
}
