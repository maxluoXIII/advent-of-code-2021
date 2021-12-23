use std::fs;
use std::collections::HashMap;

fn new_state() -> HashMap<u64, u64> {
    let mut state = HashMap::new();
    for i in 0..9 {
        state.insert(i, 0);
    }

    state
}

fn main() {
    let contents = fs::read_to_string("inputs/day6.txt").expect("Couldn't read file");
    let init_state: Vec<u64> = contents.split(',').map(|v| v.parse::<u64>().unwrap()).collect();
    
    let mut state = new_state();

    for fish in init_state {
        if let Some(count) = state.get_mut(&fish) {
            *count += 1;
        }
    }

    for _ in 0..256 {
        let mut next = new_state();
        for timer in (0..9).rev() {
            match timer {
                0 => if let Some(count) = state.get(&timer) {
                    next.insert(8, *count);
                    if let Some(six_count) = next.get_mut(&6) {
                        *six_count += count;
                    }
                },
                _ => if let Some(count) = state.get(&timer) {
                    next.insert(timer - 1, *count);
                }
            }
        }
        state = next;
    }

    let mut counter = 0;
    for i in 0..9 {
        if let Some(count) = state.get(&i) {
            counter += *count;
        }
    }

    println!("{}", counter);
}
