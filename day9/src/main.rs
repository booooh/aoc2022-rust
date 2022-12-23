use std::{collections::HashSet, fs, str::FromStr, string::ParseError};

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
    curr_knot_locations: Vec<Index>,
    visited_tail_locaions: HashSet<Index>,
}

impl Grid {
    fn new() -> Self {
        let mut visited_tail_locaions = HashSet::new();
        visited_tail_locaions.insert(Index(0, 0));
        let curr_knot_locations = vec![Index(0, 0); 10];

        return Self {
            curr_knot_locations,
            visited_tail_locaions,
        };
    }

    fn motion(&mut self, motion: &Motion) {
        for _ in 0..motion.count {
            self.move_head(&motion.direction)
        }
    }

    fn move_head(&mut self, direction: &Direction) {
        let mut curr_knot_index = 0;
        {
            let first_knot_location = self.curr_knot_locations.get_mut(curr_knot_index).unwrap();
            let new_location = match direction {
                Direction::UP => Index(first_knot_location.0, first_knot_location.1 - 1),
                Direction::DOWN => Index(first_knot_location.0, first_knot_location.1 + 1),
                Direction::LEFT => Index(first_knot_location.0 - 1, first_knot_location.1),
                Direction::RIGHT => Index(first_knot_location.0 + 1, first_knot_location.1),
            };
            first_knot_location.0 = new_location.0;
            first_knot_location.1 = new_location.1;
        }

        // check if we now need to move the next node
        while curr_knot_index != self.curr_knot_locations.len() - 1 {
            let curr_knot_location = self.curr_knot_locations.get(curr_knot_index).unwrap();
            let next_knot_location = self.curr_knot_locations.get(curr_knot_index + 1).unwrap();
            let new_distance = curr_knot_location.distance(&next_knot_location);
            if new_distance < 2 {
                break;
            }

            // if required, find out where the next knot moves to
            let diff_x = isize::signum(curr_knot_location.0 - next_knot_location.0);
            let diff_y = isize::signum(curr_knot_location.1 - next_knot_location.1);

            // fix the next location
            {
                let new_location_x = next_knot_location.0 + diff_x;
                let new_location_y = next_knot_location.1 + diff_y;
                // drop(curr_knot_location);
                // drop(next_knot_location);
                let knot_to_update = self
                    .curr_knot_locations
                    .get_mut(curr_knot_index + 1)
                    .unwrap();
                knot_to_update.0 = new_location_x;
                knot_to_update.1 = new_location_y;
            }

            curr_knot_index += 1;
        }
        let tail_location = self.curr_knot_locations[..].last().unwrap();
        self.visited_tail_locaions.insert(tail_location.clone());
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
