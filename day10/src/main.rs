use std::fs::File;
use std::io::{self, BufRead};

use std::collections::HashMap;

fn main() {
    let input_file = File::open("inputs/day10.txt").expect("Unable to open file");

    let syntax_score_map = HashMap::from([
        (')', 3),
        (']', 57),
        ('}', 1197),
        ('>', 25137)
    ]);
    let mut syntax_score = 0;

    let autocomp_score_map = HashMap::from([
        ('(', 1),
        ('[', 2),
        ('{', 3),
        ('<', 4)
    ]);
    let mut autocomp_scores = Vec::new();
    
    for line in io::BufReader::new(input_file).lines() {
        let line = line.unwrap();

        let mut c_stack = Vec::new();
        let mut corrupt = false;
        for c in line.chars() {
            match c {
                '(' => c_stack.push(c),
                '[' => c_stack.push(c),
                '{' => c_stack.push(c),
                '<' => c_stack.push(c),
                ')' => {
                    if let Some(pop_c) = c_stack.pop() {
                        if '(' != pop_c {
                            syntax_score += syntax_score_map[&c];
                            corrupt = true;
                            break;
                        }
                    }
                },
                ']' => {
                    if let Some(pop_c) = c_stack.pop() {
                        if '[' != pop_c {
                            syntax_score += syntax_score_map[&c];
                            corrupt = true;
                            break;
                        }
                    }
                },
                '}' => {
                    if let Some(pop_c) = c_stack.pop() {
                        if '{' != pop_c {
                            syntax_score += syntax_score_map[&c];
                            corrupt = true;
                            break;
                        }
                    }
                },
                '>' => {
                    if let Some(pop_c) = c_stack.pop() {
                        if '<' != pop_c {
                            syntax_score += syntax_score_map[&c];
                            corrupt = true;
                            break;
                        }
                    }
                },
                _ => {}
            };
        }

        if !corrupt && !c_stack.is_empty() {
            let mut autocomp_score: u64 = 0;
            while !c_stack.is_empty() {
                if let Some(pop_c) = c_stack.pop() {
                    match pop_c {
                        '(' => autocomp_score = autocomp_score * 5 + autocomp_score_map[&pop_c],
                        '[' => autocomp_score = autocomp_score * 5 + autocomp_score_map[&pop_c],
                        '{' => autocomp_score = autocomp_score * 5 + autocomp_score_map[&pop_c],
                        '<' => autocomp_score = autocomp_score * 5 + autocomp_score_map[&pop_c],
                        _ => {}
                    };
                }
            }
            autocomp_scores.push(autocomp_score);
        }
    }

    autocomp_scores.sort_unstable();
    let middle_score = autocomp_scores[autocomp_scores.len() / 2];

    println!("Syntax score: {syntax_score}");
    println!("Autocomplete score: {middle_score}");
}
