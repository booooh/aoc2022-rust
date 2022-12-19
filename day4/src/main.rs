use common::{Parsable, ParseStatus};

#[derive(Debug)]
struct CleanupPair {
    range_elf1: (i64, i64),
    range_elf2: (i64, i64),
}

impl Parsable for CleanupPair {
    fn parse_line(line: &str, curr_item: &mut Option<CleanupPair>) -> ParseStatus {
        let indices = line
            .split(&[',', '-'])
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        curr_item.replace(CleanupPair {
            range_elf1: (indices[0], indices[1]),
            range_elf2: (indices[2], indices[3]),
        });
        return ParseStatus::ItemComplete;
    }
}

impl CleanupPair {
    fn fully_contains(&self) -> bool {
        // check if elf 1's assigned range is fully contained in elf 2's
        if self.range_elf2.0 <= self.range_elf1.0 && self.range_elf2.1 >= self.range_elf1.1 {
            return true;
        }

        // check if elf 2's assigned range is fully contained in elf 1's
        if self.range_elf1.0 <= self.range_elf2.0 && self.range_elf1.1 >= self.range_elf2.1 {
            return true;
        }

        return false;
    }

    fn overlap(&self) -> bool {
        // check if one elf's range is copmletely after the other's
        if self.range_elf1.0 > self.range_elf2.1 || self.range_elf2.0 > self.range_elf1.1 {
            return false;
        }

        return true;
    }
}

fn main() {
    let pairs = CleanupPair::parse_file("input/day4.txt");
    println!("len cleanup pairs {:}", pairs.len());
    println!(
        "groups fully contain each other {:?}",
        pairs.iter().filter(|&pair| pair.fully_contains()).count()
    );
    println!(
        "groups overlap {:?}",
        pairs.iter().filter(|&pair| pair.overlap()).count()
    );
}
