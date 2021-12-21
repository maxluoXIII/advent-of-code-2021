use std::fs::File;
use std::io::{self, BufRead, Lines, BufReader};

#[derive(Debug)]
struct Tile {
    value: u32,
    called: bool
}

impl Tile {
    fn new(value: u32, called: bool) -> Tile {
        Tile {
            value,
            called
        }
    }
}

struct Board {
    tiles: Vec<Vec<Tile>>,
    won: bool
}

impl Board {
    fn new(line_iter: &mut Lines<BufReader<File>>) -> Board {
        let mut tiles = Vec::new();
        for i in 0..5 {
            let values = line_iter.next().unwrap().unwrap();
            let row = values.trim().split_ascii_whitespace()
                .map(|v|
                    Tile {
                        value: v.parse::<u32>().unwrap(), 
                        called: false})
                .collect();
            tiles.push(row);
        }

        Board {
            tiles, 
            won: false
        }
    }

    fn call_num(&mut self, num: u32) {
        for row in &mut self.tiles {
            for tile in row {
                if tile.value == num {
                    tile.called = true;
                }
            }
        }
    }

    fn call_tile(&mut self, x: usize, y: usize) {
        self.tiles[x][y].called = true;
    }

    fn get_tile(&mut self, x: usize, y: usize) -> &mut Tile {
        &mut self.tiles[x][y]
    }

    fn set_value(&mut self, x: usize, y:usize, val: u32) {
        self.tiles[x][y].value = val;
    }

    fn get_score(&self, called_num: u32) -> u32 {
        let mut sum = 0;
        for row in &self.tiles {
            for tile in row {
                if !tile.called {
                    sum += tile.value;
                }
            }
        }

        sum * called_num
    }

    fn has_win(&self) -> bool {
        self._check_rows() || self._check_cols() || self._check_diags()
    }

    fn _check_rows(&self) -> bool {
        for row in &self.tiles {
            let mut won = true;
            for tile in row {
                if !tile.called {
                    won = false;
                }
            }
            if won {
                return true;
            }
        }
        return false;
    }

    fn _check_cols(&self) -> bool {
        for col_idx in 0..5 {
            let mut won = true;
            for row_idx in 0..5 {
                if !self.tiles[row_idx][col_idx].called {
                    won = false;
                }
            }
            if won {
                return true;
            }
        }
        return false;
    }

    fn _check_diags(&self) -> bool {
        let mut won = true;
        for rc_idx in 0..5 {
            if !self.tiles[rc_idx][rc_idx].called {
                won = false;
            }
        }
        if won {
            return true;
        }

        for rc_idx in (0..5).rev() {
            if !self.tiles[rc_idx][rc_idx].called {
                won = false;
            }
        }
        if won {
            return true;
        } else {
            return false;
        }
    }
}

fn main() {
    let file = File::open("inputs/day4.txt").expect("Failed to open file");
    let mut line_iter = BufReader::new(file).lines();
    let called_nums = line_iter.next().unwrap().unwrap();
    let called_nums: Vec<u32> = called_nums.split(',')
        .map(|v| 
            v.parse::<u32>().unwrap())
        .collect();

    let mut boards = Vec::new();
    while let Some(_) = line_iter.next() {
        boards.push(Board::new(&mut line_iter));
    }

    let mut win_score = 0;
    let mut win_counter = 0;
    let mut found_win = false;
    let num_boards = boards.len();
    for called_num in called_nums {
        for board in &mut boards {
            board.call_num(called_num);
            if !board.won && board.has_win() {
                win_score = board.get_score(called_num);
                board.won = true;
                win_counter += 1;
                if win_counter == num_boards {
                    found_win = true;
                    break;
                }
            }
        }
        if found_win {
            break;
        }
    }

    println!("win: {}", win_score);
}
