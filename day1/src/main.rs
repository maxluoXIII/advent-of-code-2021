use std::fs::File;
use std::io::{self, BufRead};
use std::collections::LinkedList;

fn main() {
    let file = File::open("inputs/day1.txt").expect("Failed to open input file");
    let mut window = LinkedList::new();
    let mut counter = 0;
    for line in io::BufReader::new(file).lines() {
        if let Ok(depth) = line {
            let prev = if window.len() == 3 {
                let prev = Some(win_sum(&window));
                window.pop_front();
                prev
            } else{
                None
            };

            window.push_back(depth.parse().expect("Failed to parse line"));
            let curr = if window.len() == 3 {
                Some(win_sum(&window))
            } else {
                None
            };

            if curr.unwrap_or(i32::MIN) > prev.unwrap_or(i32::MAX) {
                counter += 1;
            }
        }
    }
    println!("{}", counter);
}

fn win_sum(window: &LinkedList<i32>) -> i32 {
    let mut sum = 0;
    for num in window {
        sum += num;
    }

    sum
}
