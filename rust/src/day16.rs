use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

use crate::util::load;

#[derive(Debug)]
struct QueueItem<'a> {
    time_left: [u32; 2],
    name: [&'a str; 2],
    next: usize,
    pressure: u32,
    visited: u64,
}

impl PartialOrd for QueueItem<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for QueueItem<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.pressure.cmp(&other.pressure) {
            Ordering::Equal => self.time_left.cmp(&other.time_left),
            ord => ord,
        }
    }
}
impl PartialEq for QueueItem<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for QueueItem<'_> {}

#[derive(Debug)]
struct Room {
    id: usize,
    name: String,
    rate: u32,
    tunnels: Vec<String>,
}

impl Room {
    fn load(s: &str, id: usize) -> Self {
        let tokens: Vec<&str> = s.split([' ', '=', ';', ',']).collect();
        let name = tokens[1].to_owned();
        let rate = tokens[5].parse::<u32>().unwrap();
        let tunnels = tokens[11..]
            .into_iter()
            .filter(|t| !t.is_empty())
            .map(|&t| t.to_owned())
            .collect();
        Room {
            id,
            name,
            rate,
            tunnels,
        }
    }
}

#[derive(Debug)]
struct Rooms {
    rooms: HashMap<String, Room>,
    valves: Vec<String>,
    distances: Vec<Vec<u32>>,
}

impl Rooms {
    fn load(filename: &str) -> Self {
        let rooms: HashMap<String, Room> = load::<String>(filename)
            .into_iter()
            .enumerate()
            .map(|(i, l)| Room::load(&l, i))
            .map(|r| (r.name.to_owned(), r))
            .collect();
        let valves = rooms
            .iter()
            .filter(|(_, r)| r.rate > 0)
            .map(|(_, r)| r.name.to_owned())
            .collect();
        Rooms {
            rooms,
            valves,
            distances: Vec::new(),
        }
    }

    // Floyd-Warshall
    fn init_distances(&mut self) {
        let n = self.rooms.len();
        let mut m = vec![vec![1000; n]; n];
        for (_, r) in &self.rooms {
            m[r.id][r.id] = 0;
            for t in &r.tunnels {
                let other = self.rooms[t].id;
                m[r.id][other] = 1;
                m[other][r.id] = 1;
            }
        }
        for k in 0..n {
            let mut new = vec![vec![0u32; n]; n];
            for i in 0..n {
                for j in 0..n {
                    new[i][j] = m[i][j].min(m[i][k] + m[k][j])
                }
            }
            m = new;
        }
        self.distances = m;
    }

    fn find_max_pressure<'a>(&'a self, q: &mut BinaryHeap<QueueItem<'a>>, num: usize) -> u32 {
        let mut max = 0;
        loop {
            println!("{} -- {}", max, q.len());
            match q.pop() {
                None => return max,
                Some(qi) => {
                    max = max.max(qi.pressure);
                    let current: &Room = &self.rooms[qi.name[qi.next]];
                    for v in self.valves.iter() {
                        let other = &self.rooms[v];
                        if qi.visited & (1 << other.id) > 0 {
                            continue;
                        }
                        let cost = self.distances[current.id][other.id] + 1;
                        let mut tl = qi.time_left[qi.next];
                        if tl >= cost {
                            tl -= cost;
                            let mut time_left = qi.time_left.clone();
                            time_left[qi.next] = tl;
                            let mut name = qi.name.clone();
                            name[qi.next] = &other.name;
                            q.push(QueueItem {
                                time_left,
                                name,
                                next: (qi.next + 1) % num,
                                pressure: qi.pressure + tl * other.rate,
                                visited: qi.visited | (1 << other.id),
                            });
                        }
                    }
                }
            }
        }
    }
}

pub fn part1() -> u32 {
    let mut rooms = Rooms::load("data/day16.txt");
    rooms.init_distances();
    let mut queue = BinaryHeap::new();
    queue.push(QueueItem {
        time_left: [30, 0],
        name: ["AA", ""],
        next: 0,
        pressure: 0,
        visited: 0,
    });
    rooms.find_max_pressure(&mut queue, 1)
}

pub fn part2() -> u32 {
    let mut rooms = Rooms::load("data/day16.txt");
    rooms.init_distances();
    let mut queue = BinaryHeap::new();
    queue.push(QueueItem {
        time_left: [26, 26],
        name: ["AA", "AA"],
        next: 0,
        pressure: 0,
        visited: 0,
    });
    rooms.find_max_pressure(&mut queue, 2)
}

mod tests {
    #[test]
    fn test_part1() {
        let pressure = super::part1();
        println!("Pressure: {}", pressure);
        assert_eq!(pressure, 1789);
    }

    #[test]
    fn test_part2() {
        let pressure = super::part2();
        println!("Pressure: {}", pressure);
        assert_eq!(pressure, 2496);
    }
}
