use std::collections::HashSet;

use common::{Parsable, ParseStatus};

#[derive(Debug)]
struct Rucksack {
    compartments: [String; 2],
}

impl Parsable for Rucksack {
    fn parse_line(line: &str, curr_item: &mut Option<Rucksack>) -> ParseStatus {
        let line_len = line.len();
        let compartments = [
            (&line[0..line_len / 2]).to_owned(),
            (&line[line_len / 2..line_len]).to_owned(),
        ];
        curr_item.replace(Rucksack { compartments });
        return ParseStatus::ItemComplete;
    }
}

impl Rucksack {
    fn in_both_compartments(&self) -> char {
        let compartment_set: Vec<HashSet<char>> = self
            .compartments
            .iter()
            .map(|items| HashSet::from_iter(items.chars()))
            .collect::<Vec<_>>();
        return compartment_set[0]
            .intersection(&compartment_set[1])
            .next()
            .unwrap()
            .to_owned();
    }

    fn priority(&self) -> i64 {
        let val = self.in_both_compartments();
        let ord_val = val as i64;

        if val >= 'a' && val <= 'z' {
            return ord_val - ('a' as i64) + 1;
        } else {
            return ord_val - ('A' as i64) + 27;
        }
    }
}

fn main() {
    let mut rucksacks = Rucksack::parse_file("input/day3.txt");
    println!("len rucksacks {:}", rucksacks.len());
    println!(
        "rucksacks priority {:?}",
        rucksacks
            .iter()
            .map(|rucksack| rucksack.priority())
            .sum::<i64>()
    );
}
