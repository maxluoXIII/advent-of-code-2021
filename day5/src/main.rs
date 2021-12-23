use regex::Regex;
use std::cmp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(PartialEq, Eq, Hash)]
struct Point(usize, usize);

#[derive(PartialEq, Eq)]
struct LineSegment {
    start: Point,
    end: Point,
}

impl LineSegment {
    fn is_horizontal(&self) -> bool {
        self.start.1 == self.end.1
    }

    fn is_vertical(&self) -> bool {
        self.start.0 == self.end.0
    }
}

struct Board {
    tiles: Vec<Vec<usize>>
}

impl Board {
    fn new(x: usize, y: usize) -> Board {
        let mut tiles = Vec::new();
        tiles.resize(x, Vec::new());
        for row in &mut tiles {
            row.resize(y, 0);
        }

        Board {tiles}
    }

    fn mark(&mut self, line: &LineSegment) {
        if line.is_horizontal() {
            let y = line.start.1;
            let start = cmp::min(line.start.0, line.end.0);
            let end = cmp::max(line.start.0, line.end.0);
            for x in start ..= end {
                self.tiles[x][y] += 1;
            }
        } else if line.is_vertical() {
            let x = line.start.0;
            let start = cmp::min(line.start.1, line.end.1);
            let end = cmp::max(line.start.1, line.end.1);
            for y in start ..= end {
                self.tiles[x][y] += 1;
            }
        } else {
            let x_iter: Box<dyn Iterator<Item = usize>> = if line.start.0 < line.end.0 {
                Box::new(line.start.0 ..= line.end.0)
            } else {
                Box::new((line.end.0 ..= line.start.0).rev())
            };
            let y_iter: Box<dyn Iterator<Item = usize>> = if line.start.1 < line.end.1 {
                Box::new(line.start.1 ..= line.end.1)
            } else {
                Box::new((line.end.1 ..= line.start.1).rev())
            };

            for (x, y) in x_iter.zip(y_iter) {
                self.tiles[x][y] += 1;
            }
        }
    }

    fn count_overlaps(&self) -> u32 {
        let mut sum = 0;
        for col in &self.tiles {
            for count in col {
                if *count >= 2 {
                    sum += 1;
                }
            }
        }

        sum
    }
}

fn main() {
    let file = File::open("inputs/day5.txt").expect("Failed to open file");
    let line_regex =
        Regex::new(r"^(?P<p1x>\d+),(?P<p1y>\d+) -> (?P<p2x>\d+),(?P<p2y>\d+)$").unwrap();
    let mut lines = Vec::new();
    let mut max_x = 0;
    let mut max_y = 0;
    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();
        for cap in line_regex.captures_iter(&line) {
            let p1x = cap.name("p1x").unwrap().as_str().parse::<usize>().unwrap();
            let p1y = cap.name("p1y").unwrap().as_str().parse::<usize>().unwrap();
            let p2x = cap.name("p2x").unwrap().as_str().parse::<usize>().unwrap();
            let p2y = cap.name("p2y").unwrap().as_str().parse::<usize>().unwrap();
            lines.push(LineSegment {start: Point(p1x, p1y), end: Point(p2x, p2y),});
            max_x = cmp::max(cmp::max(max_x, p1x), p2x);
            max_y = cmp::max(cmp::max(max_y, p1y), p2y);
        }
    }

    let mut board = Board::new(max_x+1, max_y+1);
    for line in lines {
        board.mark(&line);
    }

    println!("{}", board.count_overlaps());
}
