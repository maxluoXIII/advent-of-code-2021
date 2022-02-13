use std::fs::File;
use std::io::{self, BufRead};

fn get_smaller(idx: usize) -> usize {
    if 0 == idx {
        idx
    } else {
        idx - 1
    }
}

fn get_bigger(max_idx: usize, idx: usize) -> usize {
    if max_idx - 1 == idx {
        idx
    } else {
        idx + 1
    }
}

struct Map {
    elevations: Vec<Vec<u32>>,
    num_cols: usize,
    num_rows: usize
}

impl Map {
    fn from_file(file: File) -> Result<Map, std::io::Error> {
        let mut elevations = Vec::new();

        for line in io::BufReader::new(file).lines() {
            let line = line?;

            elevations.push(Vec::from_iter(line.chars().map(|c| c.to_digit(10).unwrap())));
        }

        let num_cols = elevations[0].len();
        let num_rows = elevations.len();
        Ok(Map {elevations, num_cols, num_rows})
    }

    fn get_risk_level(&self) -> u32 {
        let mut risk_sum = 0;
        for col in 0..self.num_cols {
            for row in 0..self.num_rows {
                if self.is_low_point(col, row) {
                    risk_sum += self.elevations[row][col] + 1;
                }
            }
        }

        risk_sum
    }

    fn is_low_point(&self, x: usize, y: usize) -> bool {
        let comp = self.elevations[y][x];
        let mut ret = true;
        if get_smaller(x) != x {
            ret &= self.elevations[y][get_smaller(x)] > comp;
        }

        if get_smaller(y) != y {
            ret &= self.elevations[get_smaller(y)][x] > comp;
        }

        if get_bigger(self.num_cols, x) != x {
            ret &= self.elevations[y][get_bigger(self.num_cols, x)] > comp;
        }

        if get_bigger(self.num_rows, y) != y {
            ret &= self.elevations[get_bigger(self.num_rows, y)][x] > comp;
        }

        ret
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for row in &self.elevations {
            write!(f, "[")?;
            for elevation in row {
                write!(f, "{}, ", elevation)?;
            }
            write!(f, "]\n")?;
        }

        write!(f, "]")
    }
}

fn main() {
    let file = File::open("inputs/day9.txt").expect("Failed to open file");
    
    let map = Map::from_file(file).unwrap();

    println!("{}", map.get_risk_level())
}
