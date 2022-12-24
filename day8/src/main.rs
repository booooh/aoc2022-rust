use std::{cmp::max, fs, str::FromStr, string::ParseError};
#[derive(Debug)]
struct Index(usize, usize);

#[derive(Debug)]
struct TreeHeightMap(Vec<Vec<usize>>);

#[derive(Debug, Clone, Copy)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

struct TreeHeightMapIter<'a> {
    direction: Direction,
    curr_index: Index,
    height_map: &'a TreeHeightMap,
    done: bool,
}

impl<'a> Iterator for TreeHeightMapIter<'a> {
    type Item = &'a usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        // get the value for the current iteration
        let prev_index = Index(self.curr_index.0, self.curr_index.1);

        // update for the next iteration
        self.curr_index = match (&self.direction, self.curr_index.0, self.curr_index.1) {
            (Direction::UP, _, 0) | (Direction::LEFT, 0, _) => {
                self.done = true;
                Index(self.curr_index.0, self.curr_index.1)
            }
            (Direction::UP, _, _) => Index(self.curr_index.0, self.curr_index.1 - 1),
            (Direction::DOWN, _, _) => Index(self.curr_index.0, self.curr_index.1 + 1),
            (Direction::LEFT, _, _) => Index(self.curr_index.0 - 1, self.curr_index.1),
            (Direction::RIGHT, _, _) => Index(self.curr_index.0 + 1, self.curr_index.1),
        };

        // return the value
        return self.height_map.get(&prev_index);
    }
}

impl TreeHeightMap {
    fn walk(&self, index: &Index, direction: &Direction) -> TreeHeightMapIter {
        return TreeHeightMapIter {
            curr_index: Index(index.0, index.1),
            direction: *direction,
            height_map: self,
            done: false,
        };
    }

    fn get(&self, index: &Index) -> Option<&usize> {
        let row = self.0.get(index.1)?;
        row.get(index.0)
    }

    fn is_visible_in_direction(&self, index: &Index, direction: &Direction) -> bool {
        // check that there is nothing equal or higher to the left (ignore the first element, since that's the item itself)
        let value_at_index = self.get(&index).unwrap();
        let mut larger_than_current = self.walk(index, direction).filter(|x| x >= &value_at_index);
        larger_than_current.next();
        let val = larger_than_current.next();
        return val.is_none();
    }

    fn viewing_distance(&self, index: &Index, direction: &Direction) -> usize {
        // check that there is nothing equal or higher to the left (ignore the first element, since that's the item itself)
        let value_at_index = self.get(&index).unwrap();
        let mut iter = self.walk(index, direction);
        iter.next(); // skip current element
        let mut distance = 0usize;

        while let Some(height) = iter.next() {
            if height > value_at_index {
                return distance;
            } else if height == value_at_index {
                return distance + 1;
            } else {
                distance = distance + 1
            }
        }
        return distance;
    }

    fn is_visible(&self, index: &Index) -> bool {
        let all_directions = [
            Direction::LEFT,
            Direction::RIGHT,
            Direction::UP,
            Direction::DOWN,
        ];
        all_directions
            .iter()
            .any(|direction| self.is_visible_in_direction(index, direction))
    }

    fn scenic_score(&self, index: &Index) -> usize {
        let all_directions = [
            Direction::LEFT,
            Direction::RIGHT,
            Direction::UP,
            Direction::DOWN,
        ];
        return all_directions
            .iter()
            .map(|direction| self.viewing_distance(index, direction))
            .fold(1usize, |score, distance| score * distance);
    }
}

impl FromStr for TreeHeightMap {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut height_map = TreeHeightMap(Vec::new());
        for line in s.lines() {
            height_map.0.push(
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect::<Vec<_>>(),
            );
        }

        return Ok(height_map);
    }
}

fn main() {
    let contents = fs::read_to_string("input/day8.txt").unwrap();
    let height_map: TreeHeightMap = contents.parse().unwrap();
    let mut num_visible = 0usize;
    for row in 0..height_map.0.len() {
        for col in 0..height_map.0.get(0).unwrap().len() {
            if height_map.is_visible(&Index(col, row)) {
                num_visible += 1;
            }
        }
    }

    let mut max_score = 0usize;
    for row in 0..height_map.0.len() {
        for col in 0..height_map.0.get(0).unwrap().len() {
            max_score = max(max_score, height_map.scenic_score(&Index(col, row)))
        }
    }

    dbg!(num_visible);
    dbg!(max_score);
}
