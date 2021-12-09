use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = File::open("inputs/day3.txt").expect("Failed to open input file");
    let mut readings = Vec::new();
    for line in io::BufReader::new(file).lines() {
        let reading = line.expect("Could not read line");
        readings.push((reading.clone(), true, true));
    }

    let mut low_valid_count = readings.len();
    let mut high_valid_count = readings.len();
    let mut o2_rating = String::from("");
    let mut co2_rating = String::from("");
    let mut found_o2_rating = false;
    let mut found_co2_rating = false;
    for i in 0..readings[0].0.len() {
        let mut low_counter = (0, 0);
        let mut high_counter = (0, 0);
        let mut low_indices = (Vec::new(), Vec::new());
        let mut high_indices = (Vec::new(), Vec::new());

        for (j, (reading, low_valid, high_valid)) in readings.iter().enumerate() {
            let bit = reading.as_bytes()[i] as char;
            match bit {
                '0' => {
                    if *low_valid {
                        low_counter.0 += 1;
                        low_indices.0.push(j);
                    }
                    if *high_valid {
                        high_counter.0 += 1;
                        high_indices.0.push(j);
                    }
                },
                '1' => {
                    if *low_valid {
                        low_counter.1 += 1;
                        low_indices.1.push(j);
                    }
                    if *high_valid {
                        high_counter.1 += 1;
                        high_indices.1.push(j);
                    }
                },
                default => panic!("Invalid bit: {}", default)
            }
        }

        if low_counter.0 <= low_counter.1 {
            for idx in low_indices.1 {
                readings[idx].1 = false;
                low_valid_count -= 1;
                if low_valid_count == 1 && !found_co2_rating {
                    co2_rating = readings.iter().find(|(_, low_valid, _)| *low_valid).unwrap().0.clone();
                    found_co2_rating = true;
                }
            }
        } else {
            for idx in low_indices.0 {
                readings[idx].1 = false;
                low_valid_count -= 1;
                if low_valid_count == 1 && !found_co2_rating {
                    co2_rating = readings.iter().find(|(_, low_valid, _)| *low_valid).unwrap().0.clone();
                    found_co2_rating = true;
                }
            }
        }

        if high_counter.0 > high_counter.1 {
            for idx in high_indices.1 {
                readings[idx].2 = false;
                high_valid_count -= 1;
                if high_valid_count == 1 && !found_o2_rating {
                    o2_rating = readings.iter().find(|(_, _, high_valid)| *high_valid).unwrap().0.clone();
                    found_o2_rating = true;
                }
            }
        } else {
            for idx in high_indices.0 {
                readings[idx].2 = false;
                high_valid_count -= 1;
                if high_valid_count == 1  && !found_o2_rating {
                    o2_rating = readings.iter().find(|(_, _, high_valid)| *high_valid).unwrap().0.clone();
                    found_o2_rating = true;
                }
            }
        }

        if found_co2_rating && found_o2_rating {
            break;
        }
    }

    let o2_dec = isize::from_str_radix(&o2_rating, 2).expect("Unable to parse as binary");
    let co2_dec = isize::from_str_radix(&co2_rating, 2).expect("Unable to parse as binary");

    println!("{} x {} = {}", o2_dec, co2_dec, o2_dec * co2_dec);
}
