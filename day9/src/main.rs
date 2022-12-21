use std::{collections::HashSet, convert::Infallible, fs, str::FromStr, string::ParseError};

#[derive(Debug, Clone, Copy)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}
#[derive(Debug, Clone)]
struct Motion {
    count: usize,
    direction: Direction,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Index(isize, isize);

struct Grid {
    curr_head_location: Index,
    curr_tail_location: Index,
    visited_tail_locaions: HashSet<Index>,
}

impl Grid {
    fn new() -> Self {
        let mut set = HashSet::new();
        set.insert(Index(0, 0));
        return Self {
            curr_head_location: Index(0, 0),
            curr_tail_location: Index(0, 0),
            visited_tail_locaions: set,
        };
    }

    fn motion(&mut self, motion: &Motion) {
        for _ in 0..motion.count {
            self.move_head(&motion.direction)
        }
    }

    fn move_head(&mut self, direction: &Direction) {
        self.curr_head_location = match direction {
            Direction::UP => Index(self.curr_head_location.0, self.curr_head_location.1 - 1),
            Direction::DOWN => Index(self.curr_head_location.0, self.curr_head_location.1 + 1),
            Direction::LEFT => Index(self.curr_head_location.0 - 1, self.curr_head_location.1),
            Direction::RIGHT => Index(self.curr_head_location.0 + 1, self.curr_head_location.1),
        };

        let new_distance = self.curr_head_location.distance(&self.curr_tail_location);
        if new_distance < 2 {
            return;
        }

        // otherwise, need to move the tail
        let diff_x = isize::signum(self.curr_head_location.0 - self.curr_tail_location.0);
        let diff_y = isize::signum(self.curr_head_location.1 - self.curr_tail_location.1);

        self.curr_tail_location = Index(
            self.curr_tail_location.0 + diff_x,
            self.curr_tail_location.1 + diff_y,
        );

        self.visited_tail_locaions
            .insert(self.curr_tail_location.clone());
    }
}

impl Index {
    fn distance(&self, other: &Self) -> usize {
        let dist_x = isize::abs(self.0 - other.0);
        let dist_y = isize::abs(self.1 - other.1);

        return isize::max(dist_x, dist_y) as usize;
    }
}

impl FromStr for Direction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let direction_char = s.chars().next().unwrap();
        let dir = match direction_char {
            'L' => Direction::LEFT,
            'R' => Direction::RIGHT,
            'U' => Direction::UP,
            'D' => Direction::DOWN,
            _ => todo!("error parsing direction"),
        };
        Ok(dir)
    }
}

impl FromStr for Motion {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" ");
        let direction = parts.next().unwrap().parse().unwrap();
        let count = parts.next().unwrap().parse().unwrap();
        return Ok(Self { count, direction });
    }
}

fn main() {
    let contents = fs::read_to_string("input/day9.txt").unwrap();
    let mut grid = Grid::new();
    for line in contents.lines() {
        let motion = line.parse::<Motion>().unwrap();
        grid.motion(&motion);
    }
    dbg!(grid.visited_tail_locaions.len());
}
