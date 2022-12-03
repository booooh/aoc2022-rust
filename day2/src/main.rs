use common::{Parsable, ParseStatus};

#[derive(Debug, PartialEq, Clone, Copy)]
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

    fn loses_against(&self) -> Self {
        match self {
            Choice::Rock => Choice::Paper,
            Choice::Paper => Choice::Scissors,
            Choice::Scissors => Choice::Rock,
        }
    }

    fn wins_against(&self) -> Self {
        match self {
            Choice::Rock => Choice::Scissors,
            Choice::Paper => Choice::Rock,
            Choice::Scissors => Choice::Paper,
        }
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
        if self.opponent == self.choice {
            return RoundResult::Draw;
        }

        if self.choice.wins_against() == self.opponent {
            return RoundResult::Win;
        }

        return RoundResult::Lose;
    }

    fn desired_choice(opponent: &Choice, desired_result: &str) -> Choice {
        let winninng_choice = opponent.loses_against();
        let losing_choice = opponent.wins_against();
        if desired_result == "X" {
            return losing_choice;
        }

        if desired_result == "Z" {
            return winninng_choice;
        }
        return opponent.to_owned();
    }
}

impl Parsable for Round {
    fn parse_line(line: &str, curr_item: &mut Option<Round>) -> ParseStatus {
        let parts: Vec<_> = line.split(" ").collect();
        let opponent = Choice::from_string(parts[0]);
        if (false) {
            curr_item.replace(Round {
                choice: Choice::from_string(parts[1]),
                opponent,
            });
            return ParseStatus::ItemComplete;
        } else {
            curr_item.replace(Round {
                choice: Self::desired_choice(&opponent, parts[1]),
                opponent,
            });
            return ParseStatus::ItemComplete;
        }
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
