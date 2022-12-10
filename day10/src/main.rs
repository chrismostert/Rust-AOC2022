struct Crt {
    xreg: isize,
    cycle: isize,
    drawpos: isize,
    signal_strength: Vec<isize>,
}

impl Crt {
    fn new() -> Self {
        Crt {
            xreg: 1,
            cycle: 0,
            drawpos: 0,
            signal_strength: Vec::new(),
        }
    }

    fn run_instruction(&mut self, instruction: &str) {
        match instruction {
            "noop" => {
                self.increase_cycle();
            }
            add_x_op => {
                self.increase_cycle();
                self.increase_cycle();
                let (_, val) = add_x_op.split_once(' ').unwrap();
                self.xreg += val.parse::<isize>().unwrap();
            }
        }
    }

    fn increase_cycle(&mut self) {
        self.cycle += 1;
        if self.cycle >= 20 && (self.cycle - 20) % 40 == 0 {
            self.signal_strength.push(self.xreg * self.cycle);
        }
        self.draw_to_screen();
    }

    fn draw_to_screen(&mut self) {
        if self.drawpos % 40 == 0 {
            self.drawpos = 0;
            println!();
        }
        if self.drawpos == self.xreg - 1
            || self.drawpos == self.xreg
            || self.drawpos == self.xreg + 1
        {
            print!("⬜");
        } else {
            print!("⬛");
        }
        self.drawpos += 1;
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let mut crt = Crt::new();

    for instruction in input.lines() {
        crt.run_instruction(instruction);
    }

    println!("\n\nPart 1: {}", crt.signal_strength.iter().sum::<isize>());
}
