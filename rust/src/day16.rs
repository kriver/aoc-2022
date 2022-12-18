use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use crate::util::load;

#[derive(Debug)]
struct Room {
    name: String,
    rate: u32,
    tunnels: Vec<String>,
}

impl FromStr for Room {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.split([' ', '=', ';', ',']).collect();
        let name = tokens[1].to_owned();
        let rate = tokens[5].parse::<u32>().unwrap();
        let tunnels = tokens[11..]
            .into_iter()
            .filter(|t| !t.is_empty())
            .map(|&t| t.to_owned())
            .collect();
        Ok(Room {
            name,
            rate,
            tunnels,
        })
    }
}

struct Rooms {
    rooms: Vec<Room>,
    name2id: HashMap<String, usize>,
    valves: Vec<String>,
}

impl Rooms {
    fn load(filename: &str) -> Self {
        let rooms: Vec<Room> = load(filename);
        let map = rooms
            .iter()
            .enumerate()
            .map(|(id, v)| (v.name.to_owned(), id))
            .collect();
        let valves = rooms
            .iter()
            .filter(|r| r.rate > 0)
            .map(|r| r.name.to_owned())
            .collect();
        Rooms {
            rooms,
            name2id: map,
            valves,
        }
    }

    // Floyd-Warshall
    fn distances(&self) -> Vec<Vec<u32>> {
        let n = self.rooms.len();
        let mut m = vec![vec![1000; n]; n];
        for (id, v) in self.rooms.iter().enumerate() {
            m[id][id] = 0;
            for t in &v.tunnels {
                let other = self.name2id[t];
                m[id][other] = 1;
                m[other][id] = 1;
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
        m
    }

    fn pressure_for(
        &self,
        time_left: u32,
        start: &str,
        valves: &Vec<String>,
        distances: &Vec<Vec<u32>>,
    ) -> u32 {
        let mut tl = time_left;
        let mut pos = self.name2id[start];
        let mut pressure = 0;
        for v in valves {
            let next = self.name2id[v];
            let d = distances[pos][next] + 1; // one extra for opening the valve
            if d > tl {
                break;
            }
            tl -= d;
            pressure += tl * self.rooms[next].rate;
            pos = next;
        }
        pressure
    }

    fn max_pressure(
        &self,
        time_left: u32,
        start: &str,
        valves: &mut Vec<String>,
        distances: &Vec<Vec<u32>>,
    ) -> u32 {
        // loop over all permutations of rooms with valves (Heap's algorithm)
        let n = valves.len();
        let mut c = vec![0; n];
        let mut max_pressure = self.pressure_for(time_left, start, valves, distances);
        let mut i = 1;
        let mut cnt = 0;
        while i < n {
            if c[i] < i {
                if i % 2 == 0 {
                    valves.swap(0, i);
                } else {
                    valves.swap(c[i], i);
                }
                let pressure = self.pressure_for(time_left, start, valves, distances);
                max_pressure = max_pressure.max(pressure);
                c[i] += 1;
                i = 1;
            } else {
                c[i] = 0;
                i += 1;
            }
            cnt += 1;
            if cnt % 1000000 == 0 {
                println!("{:8} - {:5} - {:?}", cnt, max_pressure, valves);
            }
        }
        max_pressure
    }

    fn with_look_ahead(&self, look_ahead: usize) -> u32 {
        let distances = self.distances();
        let mut pressure = 0;
        let mut visited: HashSet<String> = HashSet::new();
        let mut tl = 30;
        let mut prev = "AA".to_owned();
        while self.valves.len() - visited.len() >= 4 {
            let mut best_p = 0; // pressure
            let mut best_n = None; // next
            let mut best_d = 0; // delta
            let mut iterators = vec![self.valves.iter(); look_ahead];
            let mut valves = iterators
                .iter_mut()
                .map(|it| it.next().unwrap().to_owned())
                .collect::<Vec<_>>();
            'outer: loop {
                let current: HashSet<String> = HashSet::from_iter(valves.iter().map(|v| v.clone()));
                if current.len() == look_ahead && visited.intersection(&current).count() == 0 {
                    // all different valves, and none yet visited
                    let p = self.pressure_for(tl, &prev, &mut valves, &distances);
                    if p > best_p {
                        best_p = p;
                        best_n = Some(valves[0].to_owned());
                        best_d = distances[self.name2id[&prev]][self.name2id[&valves[0]]] + 1;
                    }
                }
                for i in 0..look_ahead {
                    match iterators[i].next() {
                        Some(v) => {
                            valves[i] = v.to_owned();
                            break;
                        }
                        None => {
                            if i == look_ahead - 1 {
                                break 'outer;
                            }
                            iterators[i] = self.valves.iter();
                            valves[i] = iterators[i].next().unwrap().to_owned();
                        }
                    }
                }
            }
            match best_n {
                None => break,
                Some(n) => {
                    prev = n.clone();
                    tl -= best_d;
                    println!("{:2} Best = {} with {}", tl, n, best_p);
                    pressure += tl * self.rooms[self.name2id[&n]].rate;
                    println!("   Total pressure now at {}", pressure);
                    visited.insert(n);
                    println!("   Visited {}/{}", visited.len(), self.valves.len());
                }
            }
        }
        let mut remaining: Vec<String> = HashSet::from_iter(self.valves.iter().map(|v| v.clone()))
            .difference(&visited)
            .map(|s| s.to_owned())
            .collect();
        println!("Remains {:?}", remaining);
        let p = self.max_pressure(tl, &prev, &mut remaining, &distances);
        println!("Remaining pressure is {}", p);
        pressure + p
    }
}

pub fn part1() -> u32 {
    let rooms = Rooms::load("data/day16.txt");
    rooms.with_look_ahead(6)
}

mod tests {
    #[test]
    fn test_part1() {
        let pressure = super::part1();
        println!("Pressure: {}", pressure);
        assert_eq!(pressure, 1789);
    }

    #[test]
    fn test_part2() {}
}
