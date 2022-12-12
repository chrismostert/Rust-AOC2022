use std::{
    collections::{HashMap, HashSet},
    iter,
};
type Coord = (usize, usize);

fn parse_input(input: &str) -> (HashMap<Coord, usize>, Coord, Coord) {
    input.lines().enumerate().fold(
        (HashMap::new(), Coord::default(), Coord::default()),
        |(mut grid, mut start, mut end), (y, line)| {
            line.as_bytes().iter().enumerate().for_each(|(x, c)| {
                let val = match c {
                    b'S' => {
                        start = (x, y);
                        1
                    }
                    b'E' => {
                        end = (x, y);
                        26
                    }
                    lowercase_char => (lowercase_char - b'a' + 1),
                };
                grid.insert((x, y), val.into());
            });
            (grid, start, end)
        },
    )
}

fn n_steps(grid: &HashMap<Coord, usize>, start: Coord, end: Coord) -> usize {

    let mut frontier: HashSet<Coord> = HashSet::from_iter(iter::once(start));
    let mut visited: HashSet<Coord> = HashSet::from_iter(iter::once(start));
    let mut stepcost = 0;

    let neighs = |(x, y): (usize, usize)| {
        vec![
            (x.saturating_sub(1), y),
            (x + 1, y),
            (x, y.saturating_sub(1)),
            (x, y + 1),
        ]
    };

    while !(frontier.contains(&end)) {
        stepcost += 1;

        for coord in frontier.drain().collect::<Vec<_>>() {
            for coord_neigh in neighs(coord) {
                if !visited.contains(&coord_neigh)
                    && grid.get(&coord_neigh).is_some()
                    && grid[&coord_neigh] <= grid[&coord] + 1
                {
                    visited.insert(coord_neigh);
                    frontier.insert(coord_neigh);
                }
            }
        }
    }

    stepcost
}

fn main() {
    let input = include_str!("../input.txt");
    let (grid, start, end) = parse_input(input);

    let p1 = n_steps(&grid, start, end);
    dbg!(p1);
}
