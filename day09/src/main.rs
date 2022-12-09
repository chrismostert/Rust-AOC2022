use std::collections::HashSet;

fn simulate_rope(input: &str, rope_length: usize) -> (usize, usize) {
    let mut rope = vec![(0, 0); rope_length];

    let mut visited_second: HashSet<(isize, isize)> = HashSet::new();
    let mut visited_tail: HashSet<(isize, isize)> = HashSet::new();

    for line in input.lines() {
        let (dir, amount) = line.split_once(' ').unwrap();

        for _ in 0..amount.parse().unwrap() {
            // Move head
            match dir {
                "U" => rope[0].1 += 1,
                "D" => rope[0].1 -= 1,
                "L" => rope[0].0 -= 1,
                "R" => rope[0].0 += 1,
                _ => unreachable!(),
            }

            for i in 1..rope.len() {
                // Update tail
                match (rope[i - 1].0 - rope[i].0, rope[i - 1].1 - rope[i].1) {
                    (0, 2) => rope[i].1 += 1,
                    (0, -2) => rope[i].1 -= 1,
                    (-2, 0) => rope[i].0 -= 1,
                    (2, 0) => rope[i].0 += 1,
                    (1, 2) | (2, 1) | (2, 2) => {
                        rope[i].0 += 1;
                        rope[i].1 += 1
                    }
                    (1, -2) | (2, -1) | (2, -2) => {
                        rope[i].0 += 1;
                        rope[i].1 -= 1;
                    }
                    (-1, -2) | (-2, -1) | (-2, -2) => {
                        rope[i].0 -= 1;
                        rope[i].1 -= 1;
                    }
                    (-1, 2) | (-2, 1) | (-2, 2) => {
                        rope[i].0 -= 1;
                        rope[i].1 += 1;
                    }
                    _ => {}
                }
            }

            visited_second.insert(*rope.get(1).unwrap());
            visited_tail.insert(*rope.last().unwrap());
        }
    }
    (visited_second.len(), visited_tail.len())
}

fn main() {
    let input = include_str!("../input.txt");
    let (p1, p2) = simulate_rope(input, 10);

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}
