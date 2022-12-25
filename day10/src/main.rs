use std::{collections::VecDeque, fs, str::FromStr, string::ParseError};

#[derive(Debug, Clone)]
enum Instruction {
    NOOP,
    ADDX(isize),
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "noop" {
            return Ok(Instruction::NOOP);
        } else {
            let val = s.split(" ").nth(1).unwrap().parse().unwrap();
            return Ok(Instruction::ADDX(val));
        }
    }
}

struct Program {
    instructions: VecDeque<Instruction>,
    curr_value: isize,
    history_values: Vec<isize>,
    crt: [char; 240],
}

impl Program {
    fn new(instructions: &VecDeque<Instruction>) -> Self {
        let crt = ['.'; 240];
        let mut res = Self {
            instructions: instructions.clone(),
            curr_value: 1,
            history_values: vec![1],
            crt,
        };

        res.update_crt();

        return res;
    }

    fn update_crt(&mut self) {
        // update the crt
        let curr_cycle = self.history_values.len();
        let drawn_pixel = ((curr_cycle - 1) % 40) as isize;

        let sprite_position = self.curr_value - 1..self.curr_value + 2;
        dbg!(&curr_cycle, &drawn_pixel, &sprite_position);

        // if the pixel being drawn is within the sprite position
        if sprite_position.contains(&drawn_pixel) {
            self.crt[curr_cycle - 1] = '#'
        }
    }

    fn display(&self) {
        for row in 0..6 {
            println!("{}", String::from_iter(&self.crt[row * 40..(row + 1) * 40]));
        }
    }

    fn run_instruction(&mut self) {
        let instruction = &self.instructions.pop_front().unwrap();
        match instruction {
            Instruction::NOOP => {
                // the value remains the same, so record that and increment the cycle count
                self.history_values.push(self.curr_value);
                self.update_crt()
            }
            Instruction::ADDX(value) => {
                // the value remains the same for one cycle, and then incremented
                self.history_values.push(self.curr_value);
                self.update_crt();
                self.curr_value += value;
                self.history_values.push(self.curr_value);
                self.update_crt();
            }
        }
    }
}

impl FromStr for Program {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instructions = s
            .lines()
            .map(|s| s.parse().unwrap())
            .collect::<VecDeque<_>>();
        return Ok(Self::new(&instructions));
    }
}

fn main() {
    let contents = fs::read_to_string("input/day10.txt").unwrap();
    let mut program: Program = contents.parse().unwrap();
    while !program.instructions.is_empty() {
        program.run_instruction();
    }

    // dbg!(program.crt);
    program.display();

    let val = program
        .history_values
        .iter()
        .enumerate()
        .map(|(idx, value)| (idx + 1, value))
        .filter(|(idx, _)| idx % 40 == 20)
        .map(|(idx, value)| idx as isize * value)
        .sum::<isize>();
    dbg!(val);
    // .for_each(|x| {
    //     println!("{}: {}", x.0, x.1);
    // });
}
