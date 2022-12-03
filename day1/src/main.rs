use common::{Parsable, ParseStatus};

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

impl Parsable for Elf {
    fn parse_line(line: &str, curr_item: &mut Option<Elf>) -> ParseStatus {
        if line.is_empty() {
            return ParseStatus::ItemComplete;
        }
        if curr_item.is_none() {
            curr_item.replace(Elf::default());
        }
        curr_item
            .as_mut()
            .unwrap()
            .add_calories(line.parse::<i64>().unwrap());
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
