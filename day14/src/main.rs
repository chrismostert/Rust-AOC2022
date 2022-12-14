use fxhash::FxHashSet;
use std::cmp::Ordering;
type Coord = (isize, isize);

fn make_cave(input: &str) -> (FxHashSet<Coord>, isize) {
    let mut cave = FxHashSet::default();
    let mut ymax = 0;
    input
        .lines()
        .map(|line| line.split(" -> "))
        .map(|linepieces| {
            linepieces
                .map(|piece| {
                    let (x, y) = piece.split_once(',').unwrap();
                    let (x, y) = (x.parse().unwrap(), y.parse().unwrap());
                    if y > ymax {
                        ymax = y;
                    }
                    (x, y)
                })
                .collect::<Vec<_>>()
        })
        .for_each(|line| {
            line.windows(2).for_each(|window| {
                add_rocks(&mut cave, window[0], window[1]);
            })
        });
    (cave, ymax)
}

fn add_rocks(cave: &mut FxHashSet<Coord>, (x1, y1): (isize, isize), (x2, y2): (isize, isize)) {
    match (x1.cmp(&x2), y1.cmp(&y2)) {
        (Ordering::Equal, _) => {
            for y in y1.min(y2)..=y1.max(y2) {
                cave.insert((x1, y));
            }
        }
        (_, std::cmp::Ordering::Equal) => {
            for x in x1.min(x2)..=x1.max(x2) {
                cave.insert((x, y1));
            }
        }
        _ => unreachable!(),
    }
}

fn drop_sand(
    cave: &mut FxHashSet<Coord>,
    (mut x, mut y): (isize, isize),
    ymax: isize,
    floor: bool,
) -> bool {
    let is_blocked = |coord: Coord| {
        if coord.1 == ymax + 2 {
            true
        } else {
            cave.get(&coord).is_some()
        }
    };

    match cave.get(&(x, y)) {
        Some(_) => false,
        None => loop {
            if !floor && y == ymax {
                return false;
            }
            match (
                is_blocked((x, y + 1)),
                is_blocked((x - 1, y + 1)),
                is_blocked((x + 1, y + 1)),
            ) {
                (false, _, _) => y += 1,
                (_, false, _) => {
                    x -= 1;
                    y += 1
                }
                (_, _, false) => {
                    x += 1;
                    y += 1
                }
                (true, true, true) => {
                    cave.insert((x, y));
                    return true;
                }
            }
        },
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let (mut cave, ymax) = make_cave(input);
    let mut cave_2 = cave.clone();

    let mut p1 = 0;
    while drop_sand(&mut cave, (500, 0), ymax, false) {
        p1 += 1;
    }

    let mut p2 = 0;
    while drop_sand(&mut cave_2, (500, 0), ymax, true) {
        p2 += 1;
    }

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}
