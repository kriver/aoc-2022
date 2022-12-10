use std::{cell::Cell, collections::HashMap};

use crate::util::load;

#[derive(Debug)]
enum File {
    Plain(usize),
    Folder(HashMap<String, File>, Cell<usize>),
}

fn add_to_folder(folder: &mut File, name: String, entry: File) {
    match folder {
        File::Plain(_) => panic!("cannot add to plain file"),
        File::Folder(ref mut contents, _) => drop(contents.insert(name, entry)),
    }
}

fn parse_lines(it: &mut impl Iterator<Item = String>, folder: &mut File) {
    loop {
        match &it.next() {
            None => return,
            Some(line) => {
                let token: Vec<&str> = line.split(' ').collect();
                match token[0] {
                    "$" => match token[1] {
                        "cd" => match token[2] {
                            ".." => return,
                            name => {
                                if let File::Folder(ref mut map, _) = folder {
                                    if let Some(f) = map.get_mut(name) {
                                        parse_lines(it, f);
                                    }
                                }
                            }
                        },
                        "ls" => (), // nop
                        _ => panic!("unexpected cmd"),
                    },
                    "dir" => add_to_folder(
                        folder,
                        token[1].to_owned(),
                        File::Folder(HashMap::new(), Cell::new(0)),
                    ),
                    sz => add_to_folder(
                        folder,
                        token[1].to_owned(),
                        File::Plain(sz.parse::<usize>().unwrap()),
                    ),
                }
            }
        }
    }
}

fn parse_input() -> File {
    let lines: Vec<String> = load("data/day7.txt");
    let mut fs = File::Folder(HashMap::new(), Cell::new(0));
    parse_lines(&mut lines.into_iter(), &mut fs);
    fs
}

fn calculate_size(fs: &File, limit: usize) -> (usize, usize) {
    let mut total_sz = 0;
    let mut this_sz = 0;
    if let File::Folder(contents, cell) = fs {
        for f in contents.values() {
            match f {
                File::Plain(sz) => this_sz += sz,
                File::Folder(_, _) => {
                    let (total, sub) = calculate_size(f, limit);
                    total_sz += total + (if sub <= limit { sub } else { 0 });
                    this_sz += sub;
                }
            }
        }
        cell.set(this_sz);
    }
    (total_sz, this_sz)
}

fn find_to_delete(
    contents: &HashMap<String, File>,
    at_least: usize,
    mut current_sz: usize,
) -> usize {
    for f in contents.values() {
        if let File::Folder(sub_contents, cell) = f {
            let sz = cell.get();
            if at_least <= sz && sz < current_sz {
                current_sz = sz;
            }
            current_sz = find_to_delete(sub_contents, at_least, current_sz);
        }
    }
    current_sz
}

pub fn part1() -> usize {
    let sz_limit = 100000;
    let fs = parse_input();
    calculate_size(&fs, sz_limit).0
}

pub fn part2() -> usize {
    let sz_limit = 100000;
    let fs = parse_input();
    calculate_size(&fs, sz_limit);
    let disk_sz = 70000000;
    let free_sz_required = 30000000;
    match fs {
        File::Plain(_) => panic!("root folder should not be plain file"),
        File::Folder(contents, sz) => {
            let disk_used_sz = sz.get();
            let min_delete_sz = free_sz_required - (disk_sz - disk_used_sz);
            find_to_delete(&contents, min_delete_sz, disk_sz)
        }
    }
}

mod tests {
    #[test]
    fn test_part1() {
        let sz = super::part1();
        println!("Total size of -100K directories: {}", sz);
        assert_eq!(sz, 1844187);
    }

    #[test]
    fn test_part2() {
        let sz = super::part2();
        println!("Freeing up a directory of size: {}", sz);
        assert_eq!(sz, 4978279);
    }
}
