use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, Clone)]
enum Material {
    Rock,
    Sand,
}

type Cave = HashMap<(isize, isize), Material>;

fn make_cave(input: &str) -> (Cave, isize) {
    let mut cave: Cave = HashMap::new();
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

fn add_rocks(cave: &mut Cave, (x1, y1): (isize, isize), (x2, y2): (isize, isize)) {
    match (x1.cmp(&x2), y1.cmp(&y2)) {
        (Ordering::Equal, _) => {
            for y in y1.min(y2)..=y1.max(y2) {
                cave.insert((x1, y), Material::Rock);
            }
        }
        (_, std::cmp::Ordering::Equal) => {
            for x in x1.min(x2)..=x1.max(x2) {
                cave.insert((x, y1), Material::Rock);
            }
        }
        _ => unreachable!(),
    }
}

fn drop_sand(cave: &mut Cave, (mut x, mut y): (isize, isize), ymax: isize) -> bool {
    match cave.get(&(x, y)) {
        Some(_) => false,
        None => loop {
            if y == ymax {
                return false;
            }
            match (
                cave.get(&(x, y + 1)),
                cave.get(&(x - 1, y + 1)),
                cave.get(&(x + 1, y + 1)),
            ) {
                (None, _, _) => y += 1,
                (_, None, _) => {
                    x -= 1;
                    y += 1
                }
                (_, _, None) => {
                    x += 1;
                    y += 1
                }
                (Some(_), Some(_), Some(_)) => {
                    cave.insert((x, y), Material::Sand);
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

    for x in 0..1000 {
        cave_2.insert((x, ymax + 2), Material::Rock);
    }

    let mut i = 0;
    while drop_sand(&mut cave, (500, 0), ymax) {
        i += 1;
    }

    let mut i_2 = 0;
    while drop_sand(&mut cave_2, (500, 0), ymax + 100) {
        i_2 += 1;
    }

    dbg!(i);
    dbg!(i_2);
}
