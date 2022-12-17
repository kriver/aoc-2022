use std::{collections::HashMap, str::FromStr};

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

    fn pressure_for(&self, valves: &[String], distances: &Vec<Vec<u32>>) -> u32 {
        let mut tl = 30; // time left
        let mut pos = self.name2id["AA"];
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

    fn max_pressure(&mut self, distances: &Vec<Vec<u32>>) -> u32 {
        // loop over all permutations of rooms with valves (Heap's algorithm)
        let n = self.valves.len();
        let mut c = vec![0; n];
        let mut max_pressure = self.pressure_for(&self.valves, distances);
        let mut i = 1;
        let mut cnt = 0;
        while i < n {
            if c[i] < i {
                if i % 2 == 0 {
                    self.valves.swap(0, i);
                } else {
                    self.valves.swap(c[i], i);
                }
                let pressure = self.pressure_for(&self.valves, distances);
                max_pressure = max_pressure.max(pressure);
                c[i] += 1;
                i = 1;
            } else {
                c[i] = 0;
                i += 1;
            }
            cnt += 1;
            if cnt % 1000000 == 0 {
                println!("{:8} - {:5} - {:?}", cnt, max_pressure, self.valves);
            }
        }
        max_pressure
    }
}

pub fn part1() -> u32 {
    let mut rooms = Rooms::load("data/day16.txt");
    let d = rooms.distances();
    rooms.max_pressure(&d)
}

mod tests {
    #[test]
    fn test_part1() {
        let pressure = super::part1();
        println!("Pressure: {}", pressure);
        assert_eq!(pressure, 0);
    }

    #[test]
    fn test_part2() {}
}
