use std::fs::File;
use std::io::{self, BufRead};
use std::cmp::{min, max};

struct Octopus {
    energy: i32,
    flashed: bool
}

struct OctoGrid {
    grid: Vec<Vec<Octopus>>,
    num_rows: usize,
    num_cols: usize,
    flash_count: usize,
    first_sync: usize
}

impl OctoGrid {
    fn new(file: File) -> Result<OctoGrid, std::io::Error> {
        let mut grid = Vec::new();

        for line in io::BufReader::new(file).lines() {
            let line = line?;

            let mut row = Vec::new();
            for c in line.chars() {
                row.push(Octopus {
                    energy: c.to_digit(10).unwrap()
                            .try_into().expect("Could not parse energy"), 
                    flashed: false
                });
            }

            grid.push(row);
        }

        let num_rows = grid.len();
        let num_cols = grid[0].len();
        Ok(OctoGrid{grid, num_rows, num_cols, flash_count: 0, first_sync: 0})
    }

    fn step(&mut self) -> bool {
        for row in 0..self.grid.len() {
            for col in 0..self.grid[0].len() {
                self.grid[row][col].flashed = false;
            }
        }

        for row in 0..self.grid.len() {
            for col in 0..self.grid[0].len() { 
                if !self.grid[row][col].flashed {
                    self.grid[row][col].energy += 1;
                    if self.grid[row][col].energy == 10 {
                        self.flash(row, col);
                    }
                }
            }
        }

        let mut is_synced = true;
        for row in 0..self.grid.len() {
            for col in 0..self.grid[0].len() {
                is_synced = is_synced && self.grid[row][col].flashed;
            }
        }

        is_synced
    }

    fn flash(&mut self, f_row: usize, f_col: usize) {
        self.grid[f_row][f_col].energy = 0;
        self.grid[f_row][f_col].flashed = true;

        let f_row:i32 = f_row.try_into().unwrap();
        let f_col:i32 = f_col.try_into().unwrap();
        for row in max(0, f_row-1)..=min((self.num_rows as i32)-1, f_row+1) {
            for col in max(0, f_col-1)..=min((self.num_cols as i32)-1, f_col+1) {
                let row = row as usize;
                let col = col as usize;

                if (row, col) != (f_row as usize, f_col as usize) && !self.grid[row][col].flashed {
                    self.grid[row][col].energy += 1;
                    if self.grid[row][col].energy == 10 {
                        self.flash(row, col);
                    }
                }
            }
        }

        self.flash_count += 1;
    }
}

fn main() {
    let file = File::open("inputs/day11.txt").expect("Unable to open file");

    let mut octo_grid = OctoGrid::new(file).unwrap();

    let mut epoch = 0;
    while !octo_grid.step() {
        epoch += 1;
    }
    println!("First sync in epoch {}", epoch + 1);

    println!("{} flahes", octo_grid.flash_count);
}
