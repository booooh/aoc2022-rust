use std::collections::HashSet;

use common::{Parsable, ParseStatus};

#[derive(Debug)]
struct Group {
    rucksacks: Vec<String>,
}

impl Parsable for Group {
    fn parse_line(line: &str, curr_item: &mut Option<Group>) -> ParseStatus {
        if curr_item.is_none() {
            curr_item.replace(Group {
                rucksacks: Vec::new(),
            });
        }
        let mut_curr_item = curr_item.as_mut().unwrap();

        mut_curr_item.rucksacks.push(line.to_owned());
        if mut_curr_item.rucksacks.len() == 3 {
            return ParseStatus::ItemComplete;
        }
        return ParseStatus::ItemIncomplete;
    }
}

impl Group {
    fn badge(&self) -> char {
        let rucksack_sets = self
            .rucksacks
            .iter()
            .map(|items| HashSet::from_iter(items.chars()));
        let res = rucksack_sets
            .reduce(|accum: HashSet<char>, item| accum.intersection(&item).map(|x| *x).collect())
            .unwrap()
            .iter()
            .next()
            .unwrap()
            .to_owned();
        return res;
    }

    fn priority(&self) -> i64 {
        let val = self.badge();
        let ord_val = val as i64;

        if val >= 'a' && val <= 'z' {
            return ord_val - ('a' as i64) + 1;
        } else {
            return ord_val - ('A' as i64) + 27;
        }
    }
}

fn main() {
    let mut groups = Group::parse_file("input/day3.txt");
    println!("len groups {:}", groups.len());
    println!(
        "groups priority {:?}",
        groups
            .iter()
            .map(|rucksack| rucksack.priority())
            .sum::<i64>()
    );
}
