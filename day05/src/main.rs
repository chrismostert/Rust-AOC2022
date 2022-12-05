use std::fmt;
#[derive(Clone)]
struct TowerPuzzle {
    towers: Vec<Vec<char>>,
}

impl From<&str> for TowerPuzzle {
    fn from(blueprint: &str) -> Self {
        let mut layers = blueprint.lines().rev();
        let n_towers = layers.next().unwrap().split("  ").count();
        let mut towers = vec![Vec::new(); n_towers];

        for layer in layers {
            let chars: Vec<char> = layer.chars().collect();
            for (tower_idx, tower) in towers.iter_mut().enumerate() {
                let idx = 1 + tower_idx * 4;
                if chars[idx] != ' ' {
                    tower.push(chars[idx])
                };
            }
        }

        TowerPuzzle { towers }
    }
}

impl fmt::Display for TowerPuzzle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let top: String = self
            .towers
            .iter()
            .map(|tower| tower.last().unwrap())
            .collect();
        write!(f, "{top}")
    }
}

impl TowerPuzzle {
    fn move_blocks(&mut self, amount: usize, from_idx: usize, to_idx: usize) {
        for _ in 0..amount {
            let elem = self.towers[from_idx - 1].pop().unwrap();
            self.towers[to_idx - 1].push(elem);
        }
    }

    fn move_many_blocks(&mut self, amount: usize, from_idx: usize, to_idx: usize) {
        let tower_len = self.towers[from_idx - 1].len();
        let elems: Vec<char> = self.towers[from_idx - 1]
            .drain(tower_len - amount..)
            .collect();
        self.towers[to_idx - 1].extend(elems);
    }
}

fn main() {
    let (blueprint, instructions) = include_str!("../input.txt").split_once("\n\n").unwrap();
    let mut towers = TowerPuzzle::from(blueprint);
    let mut towers_p2 = towers.clone();

    instructions
        .lines()
        .map(|line| {
            let mut parts = line.split(' ');
            (
                parts.nth(1).unwrap().parse().unwrap(),
                parts.nth(1).unwrap().parse().unwrap(),
                parts.nth(1).unwrap().parse().unwrap(),
            )
        })
        .for_each(|(amount, from_idx, to_idx)| {
            towers.move_blocks(amount, from_idx, to_idx);
            towers_p2.move_many_blocks(amount, from_idx, to_idx);
        });

    println!("Part 1: {}", towers);
    println!("Part 2: {}", towers_p2);
}
