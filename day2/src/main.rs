use std::fs::File;
use std::io::{self, BufRead};

enum Command {
    Up(i32),
    Down(i32),
    Forward(i32),
}

fn main() {
    let file = File::open("inputs/day2.txt").expect("Failed to open input file");
    let mut hori_dist = 0;
    let mut vert_dist = 0;
    let mut aim = 0;
    for line_res in io::BufReader::new(file).lines() {
        if let Ok(line) = line_res {
            let command_vec: Vec<&str> = line.split(" ").collect();
            let magnitude = command_vec[1].parse().expect("Unable to parse magnitude");
            let command = match command_vec[0] {
                "up" => Command::Up(magnitude),
                "down" => Command::Down(magnitude),
                "forward" => Command::Forward(magnitude),
                _ => panic!("Unknown command word")
            };

            match command {
                Command::Up(mag) => aim -= mag,
                Command::Down(mag) => aim += mag,
                Command::Forward(mag) => {
                    hori_dist += mag;
                    vert_dist += aim * mag;
                }
            }
        };
    }

    println!("{} x {} = {}", hori_dist, vert_dist, hori_dist * vert_dist);
}
