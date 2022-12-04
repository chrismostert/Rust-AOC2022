use itertools::Itertools;

fn priority(c: &u8) -> usize {
    match c {
        b'a'..=b'z' => (c - b'a' + 1).into(),
        b'A'..=b'Z' => (c - b'A' + 27).into(),
        _ => unreachable!(),
    }
}

fn matching_chars(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter().copied().filter(|c| b.contains(c)).collect()
}

fn main() {
    let input: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|line| line.as_bytes())
        .collect();

    let p1: usize = input
        .iter()
        .map(|sack| matching_chars(&sack[..sack.len() / 2], &sack[sack.len() / 2..]))
        .map(|overlap| priority(&overlap[0]))
        .sum();

    let p2: usize = input
        .iter()
        .tuples()
        .map(|(a, b, c)| matching_chars(a, &matching_chars(b, c)))
        .map(|overlap| priority(&overlap[0]))
        .sum();

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}
