use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    // get segment sets for 1, 4, 7, and 8

    // 0 encapsulates 1 and 4 but not 7
    // 9 encapsulates 1, 4, and 7
    // 6 does not encapsulate 1, 4, and 7

    // 3 encapsulates 1 and 7
    // 5 is missing the same two segments that 6 and 9 are
    // 2 is the last possibility
    let file = File::open("inputs/day8.txt").expect("Failed to open file");
    let mut unique_counter = 0;
    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();

        let mut patterns = Vec::new();
        let mut outputs = Vec::new();
        for (i, token) in line.split_whitespace().enumerate() {
            if i < 10 {
                patterns.push(token);
            } else if i == 10 {
                // do nothing
            } else {
                outputs.push(token);
            }
        }

        for output in outputs {
            match output.len() {
                2 => unique_counter += 1,
                3 => unique_counter += 1,
                4 => unique_counter += 1,
                7 => unique_counter += 1,
                _ => {}
            }
        }
    }

    println!("{unique_counter}");
}
