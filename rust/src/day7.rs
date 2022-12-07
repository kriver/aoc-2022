use std::collections::HashMap;

use crate::util::load;

enum File {
    Plain(usize),
    Folder(HashMap<String, File>),
}

fn add_to_folder(folder: &mut File, name: String, entry: File) {
    match folder {
        File::Plain(_) => panic!("cannot add to plain file"),
        File::Folder(ref mut contents) => drop(contents.insert(name, entry)),
    }
}

fn parse_lines(lines: &[String], mut i: usize, folder: &mut File) -> usize {
    loop {
        if i >= lines.len() {
            return i;
        }
        let line = &lines[i];
        i += 1;
        let token: Vec<&str> = line.split(' ').collect();
        match token[0] {
            "$" => match token[1] {
                "cd" => match token[2] {
                    ".." => return i,
                    name => {
                        if let File::Folder(ref mut map) = folder {
                            if let Some(ref mut f) = map.get(name) {
                                i = parse_lines(lines, i, f);
                            }
                        }
                    }
                },
                "ls" => (), // nop
                _ => panic!("unexpected cmd"),
            },
            "dir" => add_to_folder(folder, token[1].to_owned(), File::Folder(HashMap::new())),
            sz => add_to_folder(
                folder,
                token[1].to_owned(),
                File::Plain(sz.parse::<usize>().unwrap()),
            ),
        };
    }
}

fn parse_input() -> File {
    let lines: Vec<String> = load("data/day7.txt");
    let mut fs = File::Folder(HashMap::new());
    parse_lines(&lines, 0, &mut fs);
    fs
}

pub fn part1() -> usize {
    0
}

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let sz = part1();
        println!("Total size of -100K directories: {}", sz);
        assert_eq!(sz, 1844187);
    }

    #[test]
    fn test_part2() {
        // let count = part2();
        // println!("Overlap count {}", count);
        // assert_eq!(count, 924);
    }
}
