fn main() {
    let input = include_str!("../input.txt");
    let (p1, p2) =
        input
            .lines()
            .map(|s| s.split_once(' ').unwrap())
            .fold((0, 0), |(mut p1, mut p2), game| {
                match game {
                    ("A", "X") => {p1 += 4; p2 += 3},
                    ("A", "Y") => {p1 += 8; p2 += 4},
                    ("A", "Z") => {p1 += 3; p2 += 8},
                    ("B", "X") => {p1 += 1; p2 += 1},
                    ("B", "Y") => {p1 += 5; p2 += 5},
                    ("B", "Z") => {p1 += 9; p2 += 9},
                    ("C", "X") => {p1 += 7; p2 += 2},
                    ("C", "Y") => {p1 += 2; p2 += 6},
                    ("C", "Z") => {p1 += 6; p2 += 7},
                    _ => unreachable!(),
                };
                (p1, p2)
            });

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}
