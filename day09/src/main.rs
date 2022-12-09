use std::collections::HashSet;

fn simulate_rope(input: &str, rope_length: usize) -> Vec<HashSet<(isize, isize)>> {
    let mut rope: Vec<(isize, isize)> = vec![(0, 0); rope_length];
    input
        .lines()
        .fold(vec![HashSet::new(); rope_length], |mut seen, line| {
            let (dir, amount) = line.split_once(' ').unwrap();
            for _ in 0..amount.parse().unwrap() {
                match dir {
                    "U" => rope[0].1 += 1,
                    "D" => rope[0].1 -= 1,
                    "L" => rope[0].0 -= 1,
                    "R" => rope[0].0 += 1,
                    _ => unreachable!(),
                }

                for i in 1..rope.len() {
                    if (rope[i - 1].0 - rope[i].0).abs() == 2
                        || (rope[i - 1].1 - rope[i].1).abs() == 2
                    {
                        rope[i].0 += (rope[i - 1].0 - rope[i].0).signum();
                        rope[i].1 += (rope[i - 1].1 - rope[i].1).signum();
                    }
                    seen[i].insert(*rope.get(i).unwrap());
                }
            }
            seen
        })
}

fn main() {
    let input = include_str!("../input.txt");
    let seen = simulate_rope(input, 10);

    println!("Part 1: {}", seen[1].len());
    println!("Part 2: {}", seen.last().unwrap().len());
}
