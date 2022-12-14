use itertools::Itertools;

fn find_packet_index(input: &str, size: usize) -> usize {
    input
        .as_bytes()
        .windows(size)
        .find_position(|chars| chars.iter().all_unique())
        .unwrap()
        .0
        + size
}

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", find_packet_index(input, 4));
    println!("Part 2: {}", find_packet_index(input, 14));
}
