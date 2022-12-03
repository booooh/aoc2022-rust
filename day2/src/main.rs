use common::{Parsable, ParseStatus};

#[derive(Debug)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

enum RoundResult {
    Win,
    Lose,
    Draw,
}

#[derive(Debug)]
struct Round {
    choice: Choice,
    opponent: Choice,
}

impl Choice {
    fn from_string(s: &str) -> Self {
        if s == "A" || s == "X" {
            return Choice::Rock;
        }

        if s == "B" || s == "Y" {
            return Choice::Paper;
        }

        if s == "C" || s == "Z" {
            return Choice::Scissors;
        }

        panic!("This is not a valid value");
    }
}

impl Round {
    fn score(&self) -> i64 {
        return match self.choice {
            Choice::Rock => 1i64,
            Choice::Paper => 2i64,
            Choice::Scissors => 3i64,
        } + match self.result() {
            RoundResult::Win => 6i64,
            RoundResult::Lose => 0i64,
            RoundResult::Draw => 3i64,
        };
    }

    fn result(&self) -> RoundResult {
        match self.choice {
            Choice::Rock => match self.opponent {
                Choice::Rock => RoundResult::Draw,
                Choice::Paper => RoundResult::Lose,
                Choice::Scissors => RoundResult::Win,
            },
            Choice::Paper => match self.opponent {
                Choice::Rock => RoundResult::Win,
                Choice::Paper => RoundResult::Draw,
                Choice::Scissors => RoundResult::Lose,
            },
            Choice::Scissors => match self.opponent {
                Choice::Rock => RoundResult::Lose,
                Choice::Paper => RoundResult::Win,
                Choice::Scissors => RoundResult::Draw,
            },
        }
    }
}

impl Parsable for Round {
    fn parse_line(line: &str, curr_item: &mut Option<Round>) -> ParseStatus {
        let parts: Vec<_> = line.split(" ").collect();
        curr_item.replace(Round {
            choice: Choice::from_string(parts[1]),
            opponent: Choice::from_string(parts[0]),
        });
        return ParseStatus::ItemComplete;
    }
}

fn main() {
    let mut rounds = Round::parse_file("input/day2.txt");
    println!("len rounds {:}", rounds.len());
    println!(
        "{:?}",
        rounds.iter().map(|round| round.score()).sum::<i64>()
    );
}
