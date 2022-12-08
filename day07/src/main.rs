use std::collections::HashMap;

fn find_sizes(input: &str) -> HashMap<Vec<&str>, usize> {
    let mut path: Vec<&str> = Vec::new();

    input.lines().fold(HashMap::new(), |mut sizes, line| {
        let mut parts = line.split_whitespace();
        match (parts.next(), parts.next(), parts.next()) {
            (Some("$"), Some("cd"), Some("..")) => {
                path.pop();
                sizes
            }
            (Some("$"), Some("cd"), Some(dir)) => {
                path.push(dir);
                sizes
            }
            (Some("$"), _, _) => sizes,
            (Some("dir"), _, _) => sizes,
            (Some(size), _, _) => {
                for subpath in (0..path.len()).map(|i| path[0..=i].to_vec()) {
                    *sizes.entry(subpath).or_insert(0) += size.parse::<usize>().unwrap();
                }
                sizes
            }
            _ => sizes,
        }
    })
}

fn main() {
    let input = include_str!("../input.txt");
    let sizes = find_sizes(input);

    let p1: usize = sizes.values().filter(|&&v| v <= 100000).sum();
    let p2 = sizes
        .values()
        .filter(|&&v| v >= sizes[&vec!["/"]] - 40000000)
        .min()
        .unwrap();

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}
