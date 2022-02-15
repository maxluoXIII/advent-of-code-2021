use std::fs::File;
use std::io::{self, BufRead};

use std::collections::HashMap;

fn main() {
    let input_file = File::open("inputs/day10.txt").expect("Unable to open file");

    let score_map = HashMap::from([
        (')', 3),
        (']', 57),
        ('}', 1197),
        ('>', 25137)
    ]);


    let mut total_score = 0;
    
    for line in io::BufReader::new(input_file).lines() {
        let line = line.unwrap();

        let mut c_stack = Vec::new();
        for c in line.chars() {
            match c {
                '(' => c_stack.push(c),
                '[' => c_stack.push(c),
                '{' => c_stack.push(c),
                '<' => c_stack.push(c),
                ')' => {
                    if let Some(pop_c) = c_stack.pop() {
                        if '(' != pop_c {
                            total_score += score_map[&c];
                            break;
                        }
                    }
                },
                ']' => {
                    if let Some(pop_c) = c_stack.pop() {
                        if '[' != pop_c {
                            total_score += score_map[&c];
                            break;
                        }
                    }
                },
                '}' => {
                    if let Some(pop_c) = c_stack.pop() {
                        if '{' != pop_c {
                            total_score += score_map[&c];
                            break;
                        }
                    }
                },
                '>' => {
                    if let Some(pop_c) = c_stack.pop() {
                        if '<' != pop_c {
                            total_score += score_map[&c];
                            break;
                        }
                    }
                },
                _ => {}
            };
        }
    }

    println!("Total score: {total_score}");
}
