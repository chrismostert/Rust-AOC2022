fn main() {
    let input: Vec<((usize, usize), (usize, usize))> = include_str!("../input.txt")
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(',').unwrap();
            let (a1, a2) = a.split_once('-').unwrap();
            let (b1, b2) = b.split_once('-').unwrap();
            (
                (a1.parse().unwrap(), a2.parse().unwrap()),
                (b1.parse().unwrap(), b2.parse().unwrap()),
            )
        })
        .collect();

    let p1 = input
        .iter()
        .filter(|((a1, a2), (b1, b2))| (b1 >= a1 && b2 <= a2) || (a1 >= b1 && a2 <= b2))
        .count();

    let p2 = input
        .iter()
        .filter(|((a1, a2), (b1, b2))| (b2 >= a2 && b1 <= a2) || (a2 >= b2 && a1 <= b2))
        .count();

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}
