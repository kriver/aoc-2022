use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use crate::util::load;

// Create graph of nodes:
//   cat input |
//   sed 's/Valve //;s/has.* rate=//;s/;.*valves*//' |
//   tr -d ',' |
//   awk 'BEGIN {
//     printf "digraph G {\n"
//   };
//   {
//     printf "%s [label=\"%s - %s\"];\n",$1,$1,$2;
//     for(i=2;i<=NF;i++) {
//       printf "%s -> %s\n",$1,$i
//     }
//   };
//   END {
//     printf "}\n"
//   }' |
//   dot -Tsvg > graph.svg

type Id = u32;

fn to_id(name: &str) -> u32 {
    let b = name.as_bytes();
    (((b[0] - b'A' + 1) as u32) << 8) + ((b[1] - b'A' + 1) as u32)
}

fn to_name(id: Id) -> String {
    format!(
        "{}{}",
        ((id >> 8) as u8 + b'A' - 1) as char,
        ((id & 0xff) as u8 + b'A' - 1) as char
    )
}

#[derive(Debug)]
struct Valve {
    id: Id,
    rate: u32,
    tunnels: Vec<Id>,
}

impl FromStr for Valve {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.split([' ', '=', ';', ',']).collect();
        let id = to_id(tokens[1]);
        let rate = tokens[5].parse::<u32>().unwrap();
        let tunnels = tokens[11..]
            .iter()
            .filter(|t| !t.is_empty())
            .map(|s| to_id(s))
            .collect();
        Ok(Valve { id, rate, tunnels })
    }
}

type Valves = HashMap<Id, Valve>;
type Visited = HashSet<u32>;
type Opened = Vec<Id>;

fn load_valves(filename: &str) -> Valves {
    let valves: Vec<Valve> = load(filename);
    valves.into_iter().map(|v| (v.id, v)).collect()
}

fn move_through_tunnels(
    valves: &Valves,
    visited: &mut Visited,
    opened: &mut Opened,
    current: Id,
    time_left: u32,
) -> u32 {
    match time_left {
        0..=1 => 0, // no more time to (open a valve and) move
        tl => {
            // can move and possibly open new valve
            valves[&current]
                .tunnels
                .iter()
                .map(|t| {
                    let path = (current << 16) + t;
                    if !visited.contains(&path) {
                        visited.insert(path);
                        let p = open_or_not(valves, visited, opened, *t, tl - 1);
                        visited.remove(&path);
                        p
                    } else {
                        0
                    }
                })
                .max()
                .unwrap()
        }
    }
}

fn open_or_not(
    valves: &Valves,
    visited: &mut Visited,
    opened: &mut Opened,
    current: Id,
    time_left: u32,
) -> u32 {
    match time_left {
        0..=1 => 0, // no more time (or if opening, no further release)
        tl => {
            // don't open
            let p1 = move_through_tunnels(valves, visited, opened, current, tl);
            // try open
            let p2 = if valves[&current].rate > 0 && !opened.contains(&current) {
                opened.push(current);
                let p = (&valves[&current].rate * (tl - 1))
                    + move_through_tunnels(valves, visited, opened, current, tl - 1);
                opened.pop();
                p
            } else {
                0
            };
            p1.max(p2)
        }
    }
}

pub fn part1() -> u32 {
    let valves = load_valves("data/day16-test.txt");
    open_or_not(
        &valves,
        &mut HashSet::new(),
        &mut Vec::new(),
        to_id("AA"),
        30,
    )
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
