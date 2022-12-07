use std::collections::HashMap;

fn find_sizes(input: &str) -> HashMap<Vec<&str>, usize> {
    let mut path: Vec<&str> = Vec::new();

    input.lines().fold(HashMap::new(), |mut sizes, line| {
        let mut parts = line.split_whitespace();
        match (parts.next(), parts.next(), parts.next()) {
            (Some("$"), Some("cd"), Some("..")) => {
                path.pop();
            }
            (Some("$"), Some("cd"), Some(dir)) => {
                path.push(dir);
            }
            (Some("$"), _, _) => return sizes,
            (Some("dir"), _, _) => return sizes,
            (Some(size), _, _) => {
                (0..path.len())
                    .map(|i| path[0..=i].to_vec())
                    .for_each(|subpath| {
                        *sizes.entry(subpath).or_insert(0) += size.parse::<usize>().unwrap();
                    });
            }
            _ => return sizes,
        };
        sizes
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
