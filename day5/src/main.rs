use regex::Regex;
use std::cmp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(PartialEq, Eq, Hash)]
struct Point(u32, u32);

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

    fn find_intersections(&self, other: &LineSegment) -> Vec<Point> {
        let mut ret = Vec::new();

        if *self == *other {
            return ret;
        }

        let self_max_x = cmp::max(self.start.0, self.end.0);
        let self_min_x = cmp::min(self.start.0, self.end.0);
        let self_max_y = cmp::max(self.start.1, self.end.1);
        let self_min_y = cmp::min(self.start.1, self.end.1);

        let oth_max_x = cmp::max(other.start.0, other.end.0);
        let oth_min_x = cmp::min(other.start.0, other.end.0);
        let oth_max_y = cmp::max(other.start.1, other.end.1);
        let oth_min_y = cmp::min(other.start.1, other.end.1);

        if self.is_horizontal() && other.is_horizontal() && self_min_y == oth_min_y {
            for x in self_min_x..self_max_x {
                if x >= oth_min_x && x <= oth_max_x {
                    ret.push(Point(x, self.start.1));
                }
            }
        } else if self.is_vertical() && other.is_vertical() && self_min_x == oth_min_x {
            for y in self_min_y..self_max_y {
                if y >= oth_min_y && y <= oth_max_y {
                    ret.push(Point(self.start.0, y));
                }
            }
        } else if self.is_horizontal() && other.is_vertical() {
            if oth_min_x >= self_min_x
                && oth_min_x <= self_max_x
                && self_min_y >= oth_min_y
                && self_min_y <= oth_max_y
            {
                ret.push(Point(other.start.0, self.start.1));
            }
        } else if self.is_vertical() && other.is_horizontal() {
            if self_min_x >= oth_min_x
                && self_min_x <= oth_max_x
                && oth_min_y >= self_min_y
                && oth_min_y <= self_max_y
            {
                ret.push(Point(self.start.0, other.start.1));
            }
        }

        ret
    }
}

fn main() {
    let file = File::open("inputs/day5.txt").expect("Failed to open file");
    let line_regex =
        Regex::new(r"^(?P<p1x>\d+),(?P<p1y>\d+) -> (?P<p2x>\d+),(?P<p2y>\d+)$").unwrap();
    let mut lines = Vec::new();
    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();
        for cap in line_regex.captures_iter(&line) {
            lines.push(LineSegment {
                start: Point(
                    cap.name("p1x").unwrap().as_str().parse::<u32>().unwrap(),
                    cap.name("p1y").unwrap().as_str().parse::<u32>().unwrap(),
                ),
                end: Point(
                    cap.name("p2x").unwrap().as_str().parse::<u32>().unwrap(),
                    cap.name("p2y").unwrap().as_str().parse::<u32>().unwrap(),
                ),
            });
        }
    }

    let mut intersection_counts = HashMap::new();
    for line1 in &lines {
        for line2 in &lines {
            let intersections = line1.find_intersections(line2);
            for intersection in intersections {
                let count = if let Some(stored_count) = intersection_counts.get(&intersection) {
                    *stored_count
                } else {
                    0
                };
                intersection_counts.insert(intersection, count + 1);
            }
        }
    }

    // let mut overlap_count = 0;
    // for (_, count) in intersection_counts {
    //     if count >= 2 {
    //         overlap_count += 1;
    //     }
    // }

    println!("{}", intersection_counts.len());
}
