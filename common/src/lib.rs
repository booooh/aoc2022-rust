use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

pub enum ParseStatus {
    ItemComplete,
    ItemIncomplete,
}

pub trait Parsable: Sized {
    fn parse(lines: io::Lines<io::BufReader<File>>) -> Vec<Self> {
        let mut items = Vec::new();
        let mut curr_item = None;
        for line in lines {
            if let Ok(line_val) = line {
                let parse_res = Self::parse_line(&line_val, &mut curr_item);
                curr_item = match parse_res {
                    ParseStatus::ItemComplete => {
                        items.push(curr_item.unwrap());
                        None
                    }
                    ParseStatus::ItemIncomplete => curr_item,
                }
            }
        }
        // add the last item
        if let Some(item) = curr_item {
            items.push(item);
        }

        return items;
    }
    fn parse_line(line: &str, curr_item: &mut Option<Self>) -> ParseStatus;
    fn parse_file<PathType>(filename: PathType) -> Vec<Self>
    where
        PathType: AsRef<Path>,
    {
        let file = File::open(filename).unwrap();
        Self::parse(io::BufReader::new(file).lines())
    }
}
