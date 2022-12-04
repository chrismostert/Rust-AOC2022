use std::collections::HashSet;

fn priority(c: &char) -> usize {
    if c.is_lowercase() {
        *c as usize - 'a' as usize + 1
    } else {
        *c as usize - 'A' as usize + 27
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let sacks: Vec<(HashSet<char>, HashSet<char>)> = input
        .lines()
        .map(|line| {
            let (lhs, rhs) = line.split_at(line.len() / 2);
            (lhs.chars().collect(), rhs.chars().collect())
        })
        .collect();

    let isx: Vec<&char> = sacks
        .iter()
        .map(|(lhs, rhs)| lhs.intersection(rhs).next().unwrap())
        .collect();

    println!(
        "Part 1: {}",
        isx.iter().map(|&c| priority(c)).sum::<usize>()
    );

    let p2: Vec<HashSet<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let p2_sol: usize = p2
        .chunks_exact(3)
        .map(|chunk| {
            let ab: HashSet<char> = chunk[0].intersection(&chunk[1]).copied().collect();
            let abc: Vec<&char> = ab.intersection(&chunk[2]).collect();
            priority(abc[0])
        })
        .sum();

    dbg!(p2_sol);
}
