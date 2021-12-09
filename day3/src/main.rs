use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = File::open("inputs/day3.txt").expect("Failed to open input file");
    let mut counter = Vec::new();
    for line in io::BufReader::new(file).lines() {
        let reading = line.expect("Could not read line");
        
        for (i, bit) in reading.char_indices() {
            if counter.len() <= i {
                counter.push((0, 0));
            }

            match bit {
                '0' => counter[i].0 += 1,
                '1' => counter[i].1 += 1,
                default => panic!("Invalid bit: {}", default)
            }
        }
    }

    let mut gamma = String::from("");
    let mut epsilon = String::from("");
    for (zero_count, one_count) in counter {
        if zero_count > one_count {
            gamma += "0";
            epsilon += "1";
        } else {
            gamma += "1";
            epsilon += "0";
        }
    }

    let gamma_dec = isize::from_str_radix(&gamma, 2).expect("Unable to parse as binary");
    let epsilon_dec = isize::from_str_radix(&epsilon, 2).expect("Unable to parse as binary");

    println!("{} x {} = {}", gamma_dec, epsilon_dec, gamma_dec * epsilon_dec);
}
