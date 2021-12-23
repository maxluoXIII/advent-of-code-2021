use std::fs;

fn main() {
    let contents = fs::read_to_string("inputs/day6.txt").expect("Couldn't read file");
    let mut state: Vec<u32> = contents.split(',').map(|v| v.parse::<u32>().unwrap()).collect();

    for _ in 0..80 {
        let mut next = Vec::new();
        for fish in state {
            match fish {
                0 => {
                    next.push(6);
                    next.push(8);
                }
                any => next.push(any - 1)
            }
        }
        state = next;
    }

    println!("{}", state.len());
}
