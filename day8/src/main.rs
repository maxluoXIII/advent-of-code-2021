use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

fn get_decoder_map(patterns: &Vec<String>) -> HashMap<String, i32> {
    // get segment sets for 1, 4, 7, and 8

    // 0 encapsulates 1 and 7 but not 4
    // 9 encapsulates 1, 4, and 7
    // 6 does not encapsulate 1, 4, and 7

    // 3 encapsulates 1 and 7
    // 5 is subset of 6
    // 2 is the last possibility

    let mut decoder_map = HashMap::new();
    let mut encoder_map = HashMap::new();
    for pattern in patterns {
        let pattern_set: HashSet<char> = HashSet::from_iter(pattern.chars());
        match pattern.len() {
            2 => {
                encoder_map.insert(1, pattern_set);
                decoder_map.insert(pattern.clone(), 1);
            },
            4 => {
                encoder_map.insert(4, pattern_set);
                decoder_map.insert(pattern.clone(), 4);
            },
            3 => {
                encoder_map.insert(7, pattern_set);
                decoder_map.insert(pattern.clone(), 7);
            },
            7 => {
                encoder_map.insert(8, pattern_set);
                decoder_map.insert(pattern.clone(), 8);
            },
            _ => {}
        }
    }

    for pattern in patterns { 
        let pattern_set: HashSet<char> = HashSet::from_iter(pattern.chars());
        match pattern.len() {
            6 => {
                if pattern_set.is_superset(&encoder_map[&1])
                && pattern_set.is_superset(&encoder_map[&4])
                && pattern_set.is_superset(&encoder_map[&7]) {
                    encoder_map.insert(9, pattern_set);
                    decoder_map.insert(pattern.clone(), 9);
                } else if pattern_set.is_superset(&encoder_map[&1])
                && !pattern_set.is_superset(&encoder_map[&4])
                && pattern_set.is_superset(&encoder_map[&7]) {
                    encoder_map.insert(0, pattern_set);
                    decoder_map.insert(pattern.clone(), 0);
                } else if !pattern_set.is_superset(&encoder_map[&1])
                && !pattern_set.is_superset(&encoder_map[&4])
                && !pattern_set.is_superset(&encoder_map[&7]) {
                    encoder_map.insert(6, pattern_set);
                    decoder_map.insert(pattern.clone(), 6);
                }
            },
            _ => {}
        }
    }

    for pattern in patterns { 
        let pattern_set: HashSet<char> = HashSet::from_iter(pattern.chars());
        match pattern.len() {
            5 => {
                if pattern_set.is_superset(&encoder_map[&1]) {
                    encoder_map.insert(3, pattern_set);
                    decoder_map.insert(pattern.clone(), 3);
                } else if pattern_set.is_subset(&encoder_map[&6]) {
                    encoder_map.insert(5, pattern_set);
                    decoder_map.insert(pattern.clone(), 5);
                } else {
                    encoder_map.insert(2, pattern_set);
                    decoder_map.insert(pattern.clone(), 2);
                }
            },
            _ => {}
        }
    }

    decoder_map
}

fn normalize_token(token: &str) -> String {
    let mut token = token.clone().chars().collect::<Vec<char>>();
    token.sort_unstable();

    token.into_iter().collect()
}

fn main() {
    let file = File::open("inputs/day8.txt").expect("Failed to open file");
    let mut output_sum = 0;
    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();

        let mut patterns = Vec::new();
        let mut outputs = Vec::new();

        for (i, token) in line.split_ascii_whitespace().enumerate() {
            let token = normalize_token(token);
            if i < 10 {
                patterns.push(token);
            } else if i == 10 {
                // do nothing
            } else {
                outputs.push(token);
            }
        }

        let decoder_map = get_decoder_map(&patterns);

        let mut power =1000;
        for output in outputs {
            output_sum += decoder_map[&output] * power;
            power /= 10;
        }
    }

    println!("{output_sum}");
}

#[cfg(test)]
mod tests {
    use crate::normalize_token;

    #[test]
    fn normalize_test() {
        assert_eq!(normalize_token("abdecf"), "abcdef");
    }
}
