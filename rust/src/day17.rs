use std::collections::HashSet;

use crate::util::load;

enum Dir {
    Left,
    Right,
    Down,
}

struct Rock {
    move_left: Vec<(i64, i64)>,
    move_right: Vec<(i64, i64)>,
    move_down: Vec<(i64, i64)>,
    shape: Vec<(i64, i64)>,
}

fn rocks() -> Vec<Rock> {
    vec![
        // ####
        Rock {
            move_left: vec![(-1, 0)],
            move_right: vec![(4, 0)],
            move_down: vec![(0, -1), (1, -1), (2, -1), (3, -1)],
            shape: vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        },
        //  #
        // ###
        //  #
        Rock {
            move_left: vec![(0, 0), (-1, 1), (0, 2)],
            move_right: vec![(2, 0), (3, 1), (2, 2)],
            move_down: vec![(0, 0), (1, -1), (2, 0)],
            shape: vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
        },
        //   #
        //   #
        // ###
        Rock {
            move_left: vec![(-1, 0), (1, 1), (1, 2)],
            move_right: vec![(3, 0), (3, 1), (3, 2)],
            move_down: vec![(0, -1), (1, -1), (2, -1)],
            shape: vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        },
        // #
        // #
        // #
        // #
        Rock {
            move_left: vec![(-1, 0), (-1, 1), (-1, 2), (-1, 3)],
            move_right: vec![(1, 0), (1, 1), (1, 2), (1, 3)],
            move_down: vec![(0, -1)],
            shape: vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        },
        // ##
        // ##
        Rock {
            move_left: vec![(-1, 0), (-1, 1)],
            move_right: vec![(2, 0), (2, 1)],
            move_down: vec![(0, -1), (1, -1)],
            shape: vec![(0, 0), (1, 0), (0, 1), (1, 1)],
        },
    ]
}

type Cave = HashSet<i64>;

fn coord(x: i64, y: i64) -> i64 {
    (y << 3) + x
}

fn can_move(rock: &Rock, x: i64, y: i64, dir: Dir, cave: &Cave) -> bool {
    match dir {
        Dir::Left => &rock.move_left,
        Dir::Right => &rock.move_right,
        Dir::Down => &rock.move_down,
    }
    .iter()
    .fold(true, |acc, (dx, dy)| {
        let (nx, ny) = (x + dx, y + dy);
        acc && (nx >= 0) && (nx <= 6) && (ny >= 0) && !cave.contains(&coord(nx, ny))
    })
}

// fn display(cave: &Cave, y: i64) {
//     for cy in (y - 2..y + 5).rev() {
//         print!("{:3}  ", cy);
//         for x in 0..7 {
//             if cy < 0 {
//                 print!("-");
//             } else if cave.contains(&coord(x, cy)) {
//                 print!("#");
//             } else {
//                 print!(".");
//             }
//         }
//         println!();
//     }
// }

pub fn rock_fall(num: u64) -> i64 {
    let jet: Vec<char> = load::<String>("data/day17.txt")[0].chars().collect();
    let rocks = rocks();
    let mut gas = 0;
    let mut cave = HashSet::new();
    let mut shape = 0;
    let mut max_y = 3;
    let (mut x, mut y) = (2, 3);
    for _i in 0..num {
        // if _i % 100000 == 0 {
        //     println!("{:013} -> {}", _i, max_y);
        // }
        loop {
            match jet[gas] {
                '>' => {
                    if can_move(&rocks[shape], x, y, Dir::Right, &cave) {
                        x += 1;
                    }
                }
                '<' => {
                    if can_move(&rocks[shape], x, y, Dir::Left, &cave) {
                        x -= 1
                    }
                }
                _ => panic!("invalid gas"),
            }
            gas = (gas + 1) % jet.len();
            if can_move(&rocks[shape], x, y, Dir::Down, &cave) {
                y -= 1;
            } else {
                // settle rock
                let s = &rocks[shape].shape;
                for (dx, dy) in s {
                    cave.insert(coord(x + dx, y + dy));
                }
                // display(&cave, y);
                max_y = max_y.max(y + s.last().unwrap().1 + 4);
                (x, y) = (2, max_y);
                shape = (shape + 1) % 5;
                break;
            }
        }
    }
    max_y - 3
}

pub fn part1() -> i64 {
    rock_fall(2022)
}

pub fn part2() -> i64 {
    rock_fall(1000000000000)
}

mod tests {
    #[test]
    fn test_part1() {
        let height = super::part1();
        println!("Tower height: {}", height);
        assert_eq!(height, 3133);
    }

    #[test]
    fn test_part2() {
        let height = super::part2();
        println!("Tower height: {}", height);
        assert_eq!(height, 0);
    }
}
