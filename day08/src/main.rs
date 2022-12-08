use std::{cmp, collections::HashMap};
type Grid = HashMap<(isize, isize), usize>;

enum CheckDir {
    Left,
    Right,
    Up,
    Down,
}

fn parse_input(input: &str) -> Grid {
    input
        .lines()
        .enumerate()
        .fold(HashMap::new(), |mut grid, (line_idx, line)| {
            line.chars().enumerate().for_each(|(col_idx, elem)| {
                grid.insert(
                    (col_idx as isize, line_idx as isize),
                    elem.to_digit(10).unwrap() as usize,
                );
            });
            grid
        })
}

fn check_dir(grid: &Grid, mut x: isize, mut y: isize, dir: &CheckDir) -> (bool, usize) {
    let tree_size = grid[&(x, y)];
    let mut view_dist = 0;

    loop {
        match dir {
            CheckDir::Left => x -= 1,
            CheckDir::Right => x += 1,
            CheckDir::Up => y -= 1,
            CheckDir::Down => y += 1,
        }

        let check = grid.get(&(x, y));
        if let Some(&v) = check {
            view_dist += 1;
            if v >= tree_size {
                break;
            }
        }
        if check.is_none() {
            return (true, view_dist);
        }
    }
    (false, view_dist)
}

fn tree_score(grid: &Grid, x: &isize, y: &isize) -> (bool, usize) {
    [
        CheckDir::Left,
        CheckDir::Right,
        CheckDir::Up,
        CheckDir::Down,
    ]
    .iter()
    .fold((false, 1), |(was_visible, scenic_score), dir| {
        let (visible, view_dist) = check_dir(grid, *x, *y, dir);
        (was_visible || visible, scenic_score * view_dist)
    })
}

fn main() {
    let input = include_str!("../input.txt");
    let grid = parse_input(input);

    let (n_visible, highest_score) = grid.keys().map(|(x, y)| tree_score(&grid, x, y)).fold(
        (0, 0),
        |(n_visible, highest_score), (visible, score)| {
            (n_visible + visible as usize, cmp::max(highest_score, score))
        },
    );

    println!("Part 1: {n_visible}");
    println!("Part 2: {highest_score}");
}
