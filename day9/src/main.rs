use std::fs::File;
use std::io::{self, BufRead};
use std::collections::VecDeque;

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

    fn get_basin_sizes(&self) -> Vec<u32> {
        let mut basin_sizes = Vec::new();
        
        for row in 0..self.num_rows {
            for col in 0..self.num_cols {
                if self.is_low_point(col, row) {
                    basin_sizes.push(self.get_basin_size(row, col));
                }
            }
        }

        basin_sizes.sort_unstable();
        basin_sizes.reverse();
        basin_sizes
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

    fn get_basin_size(&self, lp_row: usize, lp_col: usize) -> u32 {
        let mut visit_map = Vec::new();

        for row in 0..self.num_rows {
            let mut row_vec = Vec::new();
            for col in 0..self.num_cols {
                row_vec.push(9 == self.elevations[row][col]);
            }
            visit_map.push(row_vec);
        }

        let mut visit_queue = VecDeque::new();
        visit_queue.push_back((lp_row, lp_col));
        visit_map[lp_row][lp_col] = true;

        let mut basin_size = 0;

        while !visit_queue.is_empty() {
            let (row, col) = visit_queue.pop_front().unwrap();
            basin_size += 1;

            if !visit_map[get_smaller(row)][col] {
                visit_queue.push_back((get_smaller(row), col));
                visit_map[get_smaller(row)][col] = true;
            }

            if !visit_map[row][get_smaller(col)] {
                visit_queue.push_back((row, get_smaller(col)));
                visit_map[row][get_smaller(col)] = true;
            }

            if !visit_map[get_bigger(self.num_rows, row)][col] {
                visit_queue.push_back((get_bigger(self.num_rows, row), col));
                visit_map[get_bigger(self.num_rows, row)][col] = true;
            }

            if !visit_map[row][get_bigger(self.num_cols, col)] {
                visit_queue.push_back((row, get_bigger(self.num_cols, col)));
                visit_map[row][get_bigger(self.num_cols, col)] = true;
            }
        }

        basin_size
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

    let basin_sizes = map.get_basin_sizes();

    println!("{}", basin_sizes[0] * basin_sizes[1] * basin_sizes[2]);
}
