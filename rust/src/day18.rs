use std::{collections::HashMap, ops::Add, str::FromStr};

use crate::util::load;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Coord {
    x: i8,
    y: i8,
    z: i8,
}

impl FromStr for Coord {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let n: Vec<&str> = s.split(',').collect();
        Ok(Coord {
            x: n[0].parse().unwrap(),
            y: n[1].parse().unwrap(),
            z: n[2].parse().unwrap(),
        })
    }
}

impl Add for &Coord {
    type Output = Coord;

    fn add(self, rhs: Self) -> Self::Output {
        Coord {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Type {
    Lava,
    Air,
}

type Scan = HashMap<Coord, Type>;

fn load_scan(filename: &str) -> Scan {
    let scan: Vec<Coord> = load(filename);
    scan.into_iter().map(|c| (c, Type::Lava)).collect()
}

const NEIGHBOURS: [Coord; 6] = [
    Coord { x: -1, y: 0, z: 0 },
    Coord { x: 1, y: 0, z: 0 },
    Coord { x: 0, y: -1, z: 0 },
    Coord { x: 0, y: 1, z: 0 },
    Coord { x: 0, y: 0, z: -1 },
    Coord { x: 0, y: 0, z: 1 },
];

// fn show_plane(scan: &Scan, z: usize) {
//     let mut grid = [['.'; 22]; 22];
//     for y in 0..22 {
//         for x in 0..22 {
//             grid[y as usize][x as usize] = match scan.get(&Coord { x, y, z: z as i8 }) {
//                 None => '.',
//                 Some(Type::Lava) => '#',
//                 Some(Type::Air) => '~',
//             };
//         }
//     }
//     for y in 0..22 {
//         println!("{:2} {}", y, grid[y].iter().collect::<String>())
//     }
// }

fn flood_fill(scan: &mut Scan, c: &Coord) {
    scan.insert(*c, Type::Air);
    for n in &NEIGHBOURS {
        let nc = c + n;
        if nc.x >= -1 && nc.y >= -1 && nc.z >= -1 && nc.x <= 22 && nc.y <= 22 && nc.z <= 22 {
            if !scan.contains_key(&nc) {
                flood_fill(scan, &nc);
            }
        }
    }
}

pub fn part1() -> u32 {
    let scan = load_scan("data/day18.txt");
    let mut surface = 0;
    for (c, _) in &scan {
        let mut surf = 6;
        for n in &NEIGHBOURS {
            if scan.contains_key(&(c + n)) {
                surf -= 1;
            }
        }
        surface += surf
    }
    surface
}

pub fn part2() -> u32 {
    let mut scan = load_scan("data/day18.txt");
    flood_fill(&mut scan, &Coord { x: 0, y: 0, z: 0 });
    let mut surface = 0;
    for (c, t) in &scan {
        if *t != Type::Lava {
            continue;
        }
        let mut surf = 0;
        for n in &NEIGHBOURS {
            let s = scan.get(&(c + n));
            if let Some(nt) = s {
                if *nt == Type::Air {
                    surf += 1;
                }
            }
        }
        surface += surf
    }
    surface
}

mod tests {
    #[test]
    fn test_part1() {
        let area = super::part1();
        println!("Surface area: {}", area);
        assert_eq!(area, 4400);
    }

    #[test]
    fn test_part2() {
        let area = super::part2();
        println!("Exterior surface area: {}", area);
        assert_eq!(area, 2522);
    }
}
