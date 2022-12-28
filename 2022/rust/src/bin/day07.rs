use std::fs::read_to_string;

#[derive(Debug)]
struct Directory {
    name: String,
    directories: Vec<Directory>,
    size: u32,
}

impl Directory {
    fn size(&self) -> u32 {
        self.size + self.directories.iter().map(|dir| dir.size()).sum::<u32>()
    }
}

fn main() {
    let input = read_to_string("src/inputs/07.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &String) -> () {
    let root = build_directories(input);
    let sizes: Vec<u32> = to_sizes(&root);
    let total: u32 = sizes.iter().filter(|size| **size <= 100_000).sum();
    println!("{}", total);
}

fn part2(input: &String) -> () {
    let total_diskspace = 70_000_000;
    let needed_unused = 30_000_000;
    let root = build_directories(input);
    let total_used = root.size();
    let current_unused = total_diskspace - total_used;
    let need_to_delete = needed_unused - current_unused;
    let mut sizes: Vec<u32> = to_sizes(&root);
    sizes.sort();
    let size = sizes
        .iter()
        .filter(|size| **size >= need_to_delete)
        .next()
        .unwrap();
    println!("{}", size);
}

fn build_directories(input: &String) -> Directory {
    let mut root_dir = Directory {
        name: String::from("/"),
        directories: Vec::new(),
        size: 0,
    };
    let mut stack = vec![];
    for line in input.lines() {
        if line.starts_with("$ cd") {
            let dir_name = line.replace("$ cd ", "");
            if dir_name == ".." {
                stack.pop();
            } else {
                stack.push(dir_name);
            }
        } else if line == "$ ls" {
            // do nothing
        } else {
            let mut current = &mut root_dir;
            // [1..] here because we already have the root directory so we ignore that
            // in the stack
            current = stack[1..].iter().fold(current, |dir, curr_name| {
                dir.directories
                    .iter_mut()
                    .find(|directory| directory.name == *curr_name)
                    .unwrap()
            });
            if line.starts_with("dir") {
                let dir_name = line[4..].to_string();
                current.directories.push(Directory {
                    name: dir_name,
                    directories: Vec::new(),
                    size: 0,
                });
            } else {
                let raw_size = line.split_once(" ").unwrap().0;
                let size: u32 = raw_size.parse().unwrap();
                current.size += size;
            }
        }
    }
    root_dir
}

fn to_sizes(directory: &Directory) -> Vec<u32> {
    let mut sizes = vec![directory.size()];
    for directory in directory.directories.iter() {
        sizes.append(&mut to_sizes(&directory));
    }
    sizes
}
