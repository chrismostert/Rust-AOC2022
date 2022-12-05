#[derive(Clone)]
struct TowerPuzzle {
    towers: Vec<Vec<char>>,
}

impl TowerPuzzle {
    fn create_from(blueprint: &str) -> Self {
        let mut layers = blueprint.lines().rev();
        let n_towers = layers.next().unwrap().split("  ").count();
        let mut towers = vec![Vec::new(); n_towers];

        for layer in layers {
            let chars: Vec<char> = layer.chars().collect();
            for tower_no in 0..n_towers {
                let idx = 1 + tower_no * 4;
                if chars[idx] != ' ' {
                    towers[tower_no].push(chars[1 + tower_no * 4])
                };
            }
        }

        TowerPuzzle { towers }
    }

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

    fn print_top(self) {
        for tower in &self.towers {
            print!("{}", tower.last().unwrap());
        }
        println!();
    }
}

fn main() {
    let (blueprint, instructions) = include_str!("../input.txt").split_once("\n\n").unwrap();
    let mut towers = TowerPuzzle::create_from(blueprint);
    let mut towers_p2 = towers.clone();

    for line in instructions.lines() {
        let mut parts = line.split(' ');
        let amount = parts.nth(1).unwrap().parse().unwrap();
        let from_idx = parts.nth(1).unwrap().parse().unwrap();
        let to_idx = parts.nth(1).unwrap().parse().unwrap();
        
        towers.move_blocks(amount, from_idx, to_idx);
        towers_p2.move_many_blocks(amount, from_idx, to_idx);
    }
    
    print!("Part 1: ");
    towers.print_top();

    print!("Part 2: ");
    towers_p2.print_top();
}
