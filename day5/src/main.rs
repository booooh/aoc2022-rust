use std::{collections::VecDeque, fs, str::FromStr, string::ParseError};

#[derive(Debug, Default)]
struct CrateStack {
    crates: VecDeque<char>,
}

#[derive(Debug)]
struct CargoLoad {
    stacks: Vec<CrateStack>,
}

#[derive(Debug)]
struct Instruction {
    source_stack: usize,
    dest_stack: usize,
    count: usize,
}

impl FromStr for CargoLoad {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let first_line = lines.next().unwrap();
        let num_stacks = first_line.split(" ").filter(|x| !x.is_empty()).count();
        let mut stacks = Vec::<CrateStack>::new();
        stacks.resize_with(num_stacks, Default::default);
        for line in lines {
            let chunks = line
                .as_bytes()
                .chunks(4)
                .map(|chunk| chunk[1] as char)
                .enumerate()
                .filter(|(idx, item)| item != &' ');
            for (idx, contents) in chunks {
                stacks[idx].crates.push_back(contents)
            }
        }
        return Ok(CargoLoad { stacks });
    }
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" ").collect::<Vec<_>>();

        Ok(Instruction {
            source_stack: parts[3].parse::<usize>().unwrap() - 1,
            dest_stack: parts[5].parse::<usize>().unwrap() - 1,
            count: parts[1].parse().unwrap(),
        })
    }
}

impl CargoLoad {
    fn apply_instruction(&mut self, instruction: &Instruction, part2: bool) {
        let stacks = &mut self.stacks;

        let source_stack = stacks.get_mut(instruction.source_stack).unwrap();
        let start_index = source_stack.crates.len() - instruction.count;
        let mut items = source_stack
            .crates
            .drain(start_index..)
            .collect::<VecDeque<_>>();
        if !part2 {
            items = items.into_iter().rev().collect::<VecDeque<char>>();
        }
        stacks
            .get_mut(instruction.dest_stack)
            .unwrap()
            .crates
            .append(&mut items);
    }
}

fn main() {
    let contents = fs::read_to_string("input/day5.txt").unwrap();
    let mut iter = contents.lines();
    let cargo = iter
        .by_ref()
        .take_while(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect::<Vec<_>>()
        .join("\n");
    let instructions = iter
        .map(|s| s.parse::<Instruction>().unwrap())
        .collect::<Vec<_>>();
    let mut cargo_load: CargoLoad = cargo.parse().unwrap();
    println!("{:?}", cargo_load);
    for inst in instructions {
        cargo_load.apply_instruction(&inst, true);
    }
    println!(
        "{:?}",
        cargo_load
            .stacks
            .iter()
            .map(|stack| stack.crates.back().unwrap())
            .collect::<String>()
    );
}
