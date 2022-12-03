use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

#[derive(Default, Debug)]
struct Elf {
    inventory: Vec<i64>,
}

impl Elf {
    fn add_calories(&mut self, calories: i64) {
        self.inventory.push(calories)
    }
    fn total_calories(&self) -> i64 {
        self.inventory.iter().sum::<i64>()
    }
}

enum ParseStatus {
    ItemComplete,
    ItemIncomplete,
}

trait Parsable<P: Default> {
    fn parse(lines: io::Lines<io::BufReader<File>>) -> Vec<P> {
        let mut items = Vec::new();
        let mut curr_item = P::default();
        for line in lines {
            if let Ok(line_val) = line {
                let parse_res = Self::parse_line(&line_val, &mut curr_item);
                curr_item = match parse_res {
                    ParseStatus::ItemComplete => {
                        items.push(curr_item);
                        P::default()
                    }
                    ParseStatus::ItemIncomplete => curr_item,
                }
            }
        }
        // add the last item
        items.push(curr_item);
        return items;
    }
    fn parse_line(line: &str, curr_item: &mut P) -> ParseStatus;
    fn parse_file<PathType>(filename: PathType) -> Vec<P>
    where
        PathType: AsRef<Path>,
    {
        let file = File::open(filename).unwrap();
        Self::parse(io::BufReader::new(file).lines())
    }
}

impl Parsable<Elf> for Elf {
    fn parse_line(line: &str, curr_item: &mut Elf) -> ParseStatus {
        if line.is_empty() {
            return ParseStatus::ItemComplete;
        }
        curr_item.add_calories(line.parse::<i64>().unwrap());
        return ParseStatus::ItemIncomplete;
    }
}

fn main() {
    let mut elves = Elf::parse_file("input/day1.txt");
    elves.sort_by(|a, b| b.total_calories().cmp(&a.total_calories()));
    println!("{:?}", elves.first().unwrap().total_calories());
    println!(
        "{:?}",
        elves[0..3].iter().map(|e| e.total_calories()).sum::<i64>()
    );
}
