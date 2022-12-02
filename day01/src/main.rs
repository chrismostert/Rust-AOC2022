fn main() {
    let input = include_str!("../input.txt");

    let mut sol: Vec<usize> = input
        .split("\n\n")
        .map(|elf| elf.lines().map(|x| x.parse::<usize>().unwrap()).sum())
        .collect();
    sol.sort_by(|a, b| b.cmp(a));

    println!("Part 1: {}", sol[0]);
    println!("Part 2: {}", sol[0] + sol[1] + sol[2]);
}
