use std::collections::HashMap;
#[derive(Debug)]
enum Command {
    DirectoryUp,
    Directory(&'static str),
    File(usize),
    None,
}

fn parse_input(input: &'static str) -> Vec<Command> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            match (parts.next(), parts.next()) {
                (Some("$"), Some("cd")) => {
                    if let Some(dirname) = parts.next() {
                        if dirname == ".." {
                            return Command::DirectoryUp;
                        } else {
                            return Command::Directory(dirname);
                        }
                    }
                    Command::None
                }
                (Some(maybe_num), _) => {
                    if let Ok(num) = maybe_num.parse() {
                        return Command::File(num);
                    }
                    Command::None
                }
                _ => Command::None,
            }
        })
        .collect()
}

fn find_sizes(commands: &[Command]) -> HashMap<Vec<&str>, usize> {
    let mut path: Vec<&str> = Vec::new();

    commands.iter().fold(HashMap::new(), |mut sizes, command| {
        match command {
            Command::DirectoryUp => {
                path.pop();
            }
            Command::Directory(dir) => {
                path.push(dir);
            }
            Command::File(size) => {
                (0..path.len())
                    .map(|i| path[0..=i].to_vec())
                    .for_each(|subpath| {
                        *sizes.entry(subpath).or_insert(0) += size;
                    });
            }
            _ => {}
        }
        sizes
    })
}

fn main() {
    let commands = parse_input(include_str!("../input.txt"));
    let sizes = find_sizes(&commands);

    let p1: usize = sizes.values().filter(|&&v| v <= 100000).sum();

    let total_size = 70000000;
    let size_free = total_size - sizes.get(&vec!["/"]).unwrap();
    let size_needed = 30000000 - size_free;

    let p2 = sizes.values().filter(|&&v| v >= size_needed).min().unwrap();

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}
