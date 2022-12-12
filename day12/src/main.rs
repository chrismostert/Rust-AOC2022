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

fn n_steps(grid: &HashMap<Coord, usize>, start: Coord, end: Option<Coord>) -> Option<usize> {
    let mut frontier: HashSet<Coord> = HashSet::from_iter(iter::once(start));
    let mut visited: HashSet<Coord> = HashSet::from_iter(iter::once(start));
    let mut stepcost = 0;
    let mut end_coord = (0, 0);

    if let Some(v) = end {
        end_coord = v;
    }

    let neighs = |(x, y): (usize, usize)| {
        vec![
            (x.saturating_sub(1), y),
            (x + 1, y),
            (x, y.saturating_sub(1)),
            (x, y + 1),
        ]
    };

    while !frontier.is_empty() {
        stepcost += 1;

        for coord_from in frontier.drain().collect::<Vec<_>>() {
            for coord_to in neighs(coord_from) {
                if !visited.contains(&coord_to)
                    && grid.get(&coord_to).is_some()
                    && ((end.is_some() && grid[&coord_to] <= grid[&coord_from] + 1)
                        || (end.is_none() && grid[&coord_to] >= grid[&coord_from] - 1))
                {
                    if (end.is_some() && coord_to == end_coord)
                        || (end.is_none() && grid[&coord_to] == 1)
                    {
                        return Some(stepcost);
                    }
                    visited.insert(coord_to);
                    frontier.insert(coord_to);
                }
            }
        }
    }
    None
}

fn main() {
    let input = include_str!("../input.txt");
    let (grid, start, end) = parse_input(input);

    let p1 = n_steps(&grid, start, Some(end));
    let p2 = n_steps(&grid, end, None);

    println!("Part 1: {}", p1.unwrap());
    println!("Part 2: {}", p2.unwrap());
}
