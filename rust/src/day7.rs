use std::collections::HashMap;

use crate::util::load;

enum FileType {
    Plain,
    Folder,
}

struct File {
    ftype: FileType,
    contents: Option<HashMap<String, File>>,
    size: usize,
}

fn add_to_folder(folder: &mut File, name: String, entry: File) {
    match folder.ftype {
        FileType::Plain => panic!("cannot add to plain file"),
        FileType::Folder => drop(folder.contents.unwrap().insert(name, entry)),
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
                        if let Some(ref mut c) = folder.contents {
                            let mut f = c.get(name).unwrap();
                            i = parse_lines(lines, i, f);
                        }
                    }
                },
                "ls" => (), // nop
                _ => panic!("unexpected cmd"),
            },
            "dir" => add_to_folder(
                folder,
                token[1].to_owned(),
                File {
                    ftype: FileType::Folder,
                    contents: Some(HashMap::new()),
                    size: 0,
                },
            ),
            sz => add_to_folder(
                folder,
                token[1].to_owned(),
                File {
                    ftype: FileType::Plain,
                    contents: None,
                    size: sz.parse::<usize>().unwrap(),
                },
            ),
        };
    }
}

fn parse_input() -> File {
    let lines: Vec<String> = load("data/day7.txt");
    let mut fs = File {
        ftype: FileType::Folder,
        contents: Some(HashMap::new()),
        size: 0,
    };
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
    }
}
