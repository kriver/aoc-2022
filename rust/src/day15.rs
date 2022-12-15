use std::str::FromStr;

use crate::util::load;

#[derive(Debug, Hash, PartialEq, Eq)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn new(xs: &str, ys: &str) -> Self {
        Coord {
            x: xs.parse().unwrap(),
            y: ys.parse().unwrap(),
        }
    }

    fn dist(&self, other: &Coord) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Debug)]
struct Sensor {
    loc: Coord,
    beacon: Coord,
    dist: i32,
}

impl FromStr for Sensor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.split([' ', ',', '=', ':']).collect();
        let loc = Coord::new(tokens[3], tokens[6]);
        let beacon = Coord::new(tokens[13], tokens[16]);
        let dist = loc.dist(&beacon);
        Ok(Sensor { loc, beacon, dist })
    }
}

#[derive(Debug)]
struct Regions {
    list: Vec<(i32, i32)>,
}

impl Regions {
    fn add(&mut self, from: i32, to: i32) {
        let overlaps = self.list.drain_filter(|(x1,x2)|
        // full overlap
        (from <= *x1 && *x2 <= to) ||
        // partial overlap
        (*x1 <= from && from <= *x2) || (*x1 <= to && to <= *x2));
        let (mut nx1, mut nx2) = (from, to);
        for (x1, x2) in overlaps {
            nx1 = nx1.min(x1);
            nx2 = nx2.max(x2);
        }
        self.list.push((nx1, nx2));
    }

    fn ignore(&mut self, x: i32) {
        let overlaps: Vec<(i32, i32)> = self
            .list
            .drain_filter(|(x1, x2)| *x1 <= x && x <= *x2)
            .collect();
        for (x1, x2) in overlaps {
            if x1 != x2 {
                if x1 == x {
                    self.list.push((x + 1, x2));
                } else if x == x2 {
                    self.list.push((x1, x - 1));
                } else {
                    self.list.push((x1, x - 1));
                    self.list.push((x + 1, x2));
                }
            }
        }
    }

    fn len(&self) -> i32 {
        self.list.iter().map(|(x1, x2)| x2 - x1 + 1).sum()
    }
}

fn used_regions(sensors: &Vec<Sensor>, row: i32, ignore_beacon: bool) -> Regions {
    let mut regions = Regions { list: vec![] };
    for s in sensors {
        // add sensor sweep
        let d = (s.loc.y - row).abs();
        if d <= s.dist {
            let d = (d - s.dist).abs();
            if d >= 0 {
                regions.add(s.loc.x - d, s.loc.x + d);
            }
        }
        // ignore beacon on row
        if ignore_beacon && s.beacon.y == row {
            regions.ignore(s.beacon.x);
        }
    }
    regions
}

pub fn part1() -> i32 {
    let row = 2000000;
    let sensors: Vec<Sensor> = load("data/day15.txt");
    used_regions(&sensors, row, true).len()
}

pub fn part2() -> i64 {
    let max = 4000000;
    let sensors: Vec<Sensor> = load("data/day15.txt");
    for y in 0..max {
        let regions = used_regions(&sensors, y, false);
        if regions.list.len() == 2 {
            let (x11, x12) = regions.list[0];
            let (x21, x22) = regions.list[1];
            if x21 - x12 == 2 {
                return max as i64 * (x12 as i64 + 1) + y as i64;
            } else if x11 - x22 == 2 {
                return max as i64 * (x22 as i64 + 1) + y as i64;
            }
        }
    }
    panic!("no result");
}

mod tests {
    #[test]
    fn test_part1() {
        let num = super::part1();
        println!("Number of positions not containing a beacon: {}", num);
        assert_eq!(num, 4748135);
    }

    #[test]
    fn test_part2() {
        let freq = super::part2();
        println!("Tuning frequency: {}", freq);
        assert_eq!(freq, 13743542639657);
    }
}
