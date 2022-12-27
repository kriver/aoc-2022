use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
    fmt::{Display, Formatter},
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

#[derive(Debug, Clone)]
struct Counters {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
}

impl From<&[&str]> for Counters {
    fn from(s: &[&str]) -> Self {
        let mut inv = Counters::new();
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

impl<'a> Sub for &'a Counters {
    type Output = Counters;

    fn sub(self, rhs: Self) -> Self::Output {
        Counters {
            ore: self.ore - rhs.ore,
            clay: self.clay - rhs.clay,
            obsidian: self.obsidian - rhs.obsidian,
            geode: self.geode - rhs.geode,
        }
    }
}

impl<'a> Add for &'a Counters {
    type Output = Counters;

    fn add(self, rhs: Self) -> Self::Output {
        Counters {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
            geode: self.geode + rhs.geode,
        }
    }
}

impl Counters {
    fn new() -> Self {
        Counters {
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
    needs: Counters,
}

impl FromStr for Robot {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.split_whitespace().collect();
        let generates: Resource = tokens[1].parse().unwrap();
        let needs: Counters = Counters::from(&tokens[4..]);
        Ok(Robot { generates, needs })
    }
}

impl Robot {
    fn can_produce(&self, inv: &Counters) -> bool {
        inv.ore >= self.needs.ore
            && inv.clay >= self.needs.clay
            && inv.obsidian >= self.needs.obsidian
    }
}

struct State<'a> {
    tl: u32, // time left
    robots: Counters,
    inv: Counters,
    bp: &'a Blueprint,
}

impl Display for State<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "State[{}, Robots {:?}, {:?}]",
            self.tl, self.robots, self.inv
        )
    }
}

impl PartialOrd for State<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        // ordered by number of robots, advanced to simple
        match self.robots.geode.cmp(&other.robots.geode) {
            Ordering::Equal => match self.robots.obsidian.cmp(&other.robots.obsidian) {
                Ordering::Equal => match self.robots.clay.cmp(&other.robots.clay) {
                    Ordering::Equal => self.robots.ore.cmp(&other.robots.ore),
                    ord => ord,
                },
                ord => ord,
            },
            ord => ord,
        }
    }
}

impl PartialEq for State<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for State<'_> {}

impl<'a> State<'a> {
    fn new_with(&self, robot: &Robot, production: &Counters) -> Self {
        let mut robots = self.robots.clone();
        match robot.generates {
            Resource::Ore => robots.ore += 1,
            Resource::Clay => robots.clay += 1,
            Resource::Obsidian => robots.obsidian += 1,
            Resource::Geode => robots.geode += 1,
        }
        State {
            tl: self.tl - 1,
            robots,
            inv: &(&self.inv - &robot.needs) + production,
            bp: self.bp,
        }
    }

    fn key(&self) -> (u64, u64, u32) {
        let robots = ((self.robots.ore as u64) << 48)
            | ((self.robots.clay as u64) << 32)
            | ((self.robots.obsidian as u64) << 16)
            | ((self.robots.geode as u64) << 0);
        let inventory = ((self.inv.ore as u64) << 48)
            | ((self.inv.clay as u64) << 32)
            | ((self.inv.obsidian as u64) << 16)
            | ((self.inv.geode as u64) << 0);
        (robots as u64, inventory as u64, self.tl)
    }

    fn produce(&mut self) {
        self.tl -= 1;
        self.inv = &self.inv + &self.production();
    }

    fn production(&self) -> Counters {
        self.robots.clone()
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
    fn produce(&self) -> u32 {
        // not ideal yet
        fn should_add(s: &State, best: u32) -> bool {
            // assuming we can make a geode robot for each `tl` left and generate geodes along the way....
            let tl = s.tl as f32;
            if ((tl * (tl - 1.) / 2.).round() as u32 + s.tl * s.robots.geode + s.inv.geode) <= best
            {
                return false;
            }
            // more inventory than needed for creating robots... we should have created at least one instead
            if s.inv.ore > 3 * s.bp.rules.iter().map(|r| r.needs.ore).sum::<u32>() {
                return false;
            }
            if s.inv.clay > 3 * s.bp.rules[2].needs.clay {
                return false;
            }
            if s.inv.obsidian > 3 * s.bp.rules[3].needs.obsidian {
                return false;
            }
            // all ok
            true
        }
        let mut q = BinaryHeap::from([State {
            tl: 24,
            robots: Counters {
                ore: 1,
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
            inv: Counters::new(),
            bp: &self,
        }]);
        let mut visited = HashSet::new();
        let mut max = 0;
        loop {
            match q.pop() {
                None => break,
                Some(mut s) => {
                    // println!("Best = {}, #q = {}/{}", max, q.len(), visited.len());
                    if s.tl == 0 {
                        max = max.max(s.inv.geode);
                        continue;
                    }
                    // determine how much we will produce this cycle
                    let production = s.production();
                    // create (or not) new robots
                    for robot in &self.rules {
                        if robot.can_produce(&s.inv) {
                            let ns = s.new_with(&robot, &production);
                            if should_add(&ns, max) & !visited.contains(&ns.key()) {
                                // println!("Adding extra {:?} robot {}", robot.generates, ns);
                                visited.insert(ns.key());
                                q.push(ns);
                            }
                        }
                    }
                    // original state is also a candidate (no new robots created)
                    s.produce();
                    if should_add(&s, max) & !visited.contains(&s.key()) {
                        // println!("Adding {}", s);
                        visited.insert(s.key());
                        q.push(s);
                    }
                }
            }
        }
        // println!("{}: {}", self.id, max);
        self.id * max as u32
    }
}

pub fn part1() -> u32 {
    let blueprints: Vec<Blueprint> = load("data/day19.txt");
    blueprints.into_iter().map(|bp| bp.produce()).sum()
}

mod tests {
    #[test]
    fn test_part1() {
        let quality = super::part1();
        println!("Total quality: {}", quality);
        assert_eq!(quality, 1466);
    }

    #[test]
    fn test_part2() {}
}
