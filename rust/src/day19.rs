use std::{
    collections::HashMap,
    ops::{Add, Sub},
    str::FromStr,
};

use crate::util::load;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}
impl FromStr for Resource {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ore" => Ok(Resource::Ore),
            "clay" => Ok(Resource::Clay),
            "obsidian" => Ok(Resource::Obsidian),
            "geode" => Ok(Resource::Geode),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Inventory {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
}

impl From<&[&str]> for Inventory {
    fn from(s: &[&str]) -> Self {
        let mut inv = Inventory::new();
        let mut i = 0;
        while i < s.len() {
            let n: u32 = s[i].parse().unwrap();
            let r: Resource = s[i + 1].parse().unwrap();
            match r {
                Resource::Ore => inv.ore = n,
                Resource::Clay => inv.clay = n,
                Resource::Obsidian => inv.obsidian = n,
                _ => panic!("unexpected resource"),
            }
            i += 3;
        }
        inv
    }
}

impl<'a> Sub for &'a Inventory {
    type Output = Inventory;

    fn sub(self, rhs: Self) -> Self::Output {
        Inventory {
            ore: self.ore - rhs.ore,
            clay: self.clay - rhs.clay,
            obsidian: self.obsidian - rhs.obsidian,
            geode: self.geode - rhs.geode,
        }
    }
}

impl<'a> Add for &'a Inventory {
    type Output = Inventory;

    fn add(self, rhs: Self) -> Self::Output {
        Inventory {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
            geode: self.geode + rhs.geode,
        }
    }
}

impl Inventory {
    fn new() -> Self {
        Inventory {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }
}

#[derive(Debug)]
struct Robot {
    generates: Resource,
    needs: Inventory,
}

impl FromStr for Robot {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.split_whitespace().collect();
        let generates: Resource = tokens[1].parse().unwrap();
        let needs: Inventory = Inventory::from(&tokens[4..]);
        Ok(Robot { generates, needs })
    }
}

impl Robot {
    fn can_produce(&self, inv: &Inventory) -> bool {
        inv.ore >= self.needs.ore
            && inv.clay >= self.needs.clay
            && inv.obsidian >= self.needs.obsidian
    }
}

#[derive(Debug)]
struct State {
    rec: u32,                   // recursion level
    tl: u32,                    // time left
    rc: HashMap<Resource, u32>, // robot counts
    inv: Inventory,
}

impl State {
    fn new_with(&self, robot: &Robot, production: &Inventory) -> Self {
        let mut rc = self.rc.clone();
        rc.entry(robot.generates)
            .and_modify(|c| *c += 1)
            .or_insert(1);
        State {
            rec: self.rec + 1,
            tl: self.tl - 1,
            rc,
            inv: &(&self.inv - &robot.needs) + production,
        }
    }

    fn produce(&mut self) {
        self.tl -= 1;
        self.inv = &self.inv + &self.production();
    }

    fn production(&self) -> Inventory {
        let mut inv = Inventory::new();
        for (r, c) in &self.rc {
            match r {
                Resource::Ore => inv.ore = *c,
                Resource::Clay => inv.clay = *c,
                Resource::Obsidian => inv.obsidian = *c,
                Resource::Geode => inv.geode = *c,
            }
        }
        inv
    }
}

#[derive(Debug)]
struct Blueprint {
    id: u32,
    rules: Vec<Robot>,
}

impl FromStr for Blueprint {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.split([':', '.']).collect();
        let id = tokens[0].split(' ').last().unwrap().parse().unwrap();
        let rules: Vec<Robot> = tokens[1..]
            .into_iter()
            .filter(|r| r.len() != 0)
            .map(|r| r.parse().unwrap())
            .collect();
        Ok(Blueprint { id, rules })
    }
}

impl Blueprint {
    fn produce(&self, mut state: State) -> u32 {
        if state.tl == 0 {
            state.inv.geode
        } else {
            // determine how much we will produce this cycle
            let production = state.production();
            // create (or not) new robots
            let mut next_states: Vec<State> = vec![];
            for robot in &self.rules {
                if robot.can_produce(&state.inv) {
                    next_states.push(state.new_with(&robot, &production));
                }
            }
            // println!("({:2}) {:?} - {:?}", next_states.len(), state, next_states);
            // original state is also a candidate (no new robots created)
            state.produce();
            next_states.push(state);
            // recurse using new states
            next_states
                .into_iter()
                .map(|s| self.produce(s))
                .max()
                .unwrap()
        }
        // FIXME need to mulitply by ID
    }
}

pub fn part1() -> u32 {
    let blueprints: Vec<Blueprint> = load("data/day19-test.txt");
    let state = State {
        rec: 0,
        tl: 24,
        rc: HashMap::from([(Resource::Ore, 1)]),
        inv: Inventory::new(),
    };
    println!("{}", blueprints[0].produce(state));
    0
}

// TODO use linear programming ??

mod tests {
    #[test]
    fn test_part1() {
        let quality = super::part1();
        println!("Total quality: {}", quality);
        assert_eq!(quality, 0);
    }

    #[test]
    fn test_part2() {}
}
