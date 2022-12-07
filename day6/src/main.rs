use std::{fs, process::exit};

fn main() {
    let contents = fs::read_to_string("input/day6.txt").unwrap();
    let mut found_start = String::new();
    for (start_idx, c) in contents.chars().enumerate() {
        let found = found_start.find(c);
        let cur_len = found_start.len();
        match (found, cur_len) {
            (None, 3) => {
                found_start = found_start + &c.to_string();
                println!("found: {} {}", found_start, start_idx + 1);
                exit(0);
            }
            (Some(idx), _) => {
                found_start = found_start[idx + 1..].to_owned() + &c.to_string();
            }
            (None, _) => {
                found_start = found_start + &c.to_string();
            }
        }
        println!("current start {}", found_start);
    }
}
