use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");

    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    let (mut x_h, mut y_h) = (0, 0);
    let (mut x_t, mut y_t) = (0, 0);

    for line in input.lines() {
        let (dir, amount) = line.split_once(' ').unwrap();

        for _ in 0..amount.parse().unwrap() {
            // Move head
            match dir {
                "U" => y_h += 1,
                "D" => y_h -= 1,
                "L" => x_h -= 1,
                "R" => x_h += 1,
                _ => unreachable!(),
            }

            // Update tail
            match (x_h - x_t, y_h - y_t) {
                (0, 2) => y_t += 1,
                (0, -2) => y_t -= 1,
                (-2, 0) => x_t -= 1,
                (2, 0) => x_t += 1,
                (1, 2) | (2, 1) => {
                    x_t += 1;
                    y_t += 1
                }
                (1, -2) | (2, -1) => {
                    x_t += 1;
                    y_t -= 1;
                }
                (-1, -2) | (-2, -1) => {
                    x_t -= 1;
                    y_t -= 1;
                }
                (-1, 2) | (-2, 1) => {
                    x_t -= 1;
                    y_t += 1;
                }
                _ => {}
            }

            // Add tail pos
            visited.insert((x_t, y_t));
        }
    }

    let p1 = visited.len();
    dbg!(p1);
}
