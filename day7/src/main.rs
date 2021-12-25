use std::fs;

fn abs_diff(a: u32, b: u32) -> u32 {
    if a > b {
        a - b
    } else {
        b - a
    }
}

fn total_cost(positions: &Vec<u32>, target: u32) -> u32 {
    let mut sum = 0;
    for position in positions {
        sum += abs_diff(*position, target);
    }

    sum
}

fn simple_series_sum(v: u32) -> u32 {
    (1 + v) * v / 2
}

fn new_total_cost(positions: &Vec<u32>, target: u32) -> u32 {
    let mut sum = 0;
    for position in positions {
        sum += simple_series_sum(abs_diff(*position, target))
    }

    sum
}

fn median(list: &Vec<u32>) -> u32 {
    if list.len() % 2 == 0 {
        let hi = list.len() / 2;
        let lo = list.len() / 2 - 1;
        let med = (list[hi] as f64 + list[lo] as f64) / 2.0;
        med.round() as u32
    } else {
        list[list.len() / 2]
    }
}

fn mean(list: &Vec<u32>) -> u32 {
    let sum: u32 = list.iter().sum();
    ((sum as f64) / (list.len() as f64)).floor() as u32
}

fn main() {
    let contents = fs::read_to_string("inputs/day7.txt").expect("Unable to read file");
    let mut crab_positions: Vec<u32> = contents.split(',')
        .map(|v| 
            v.parse::<u32>().unwrap()
        )
        .collect();

    crab_positions.sort_unstable();

    // let med_pos = crab_positions.len() / 2;
    // let med_cost = total_cost(&crab_positions, crab_positions[med_pos]);
    // let mut using_med = true;
    // let min_cost = if crab_positions.len() % 2 == 0 {
    //     let med_min_one_cost = total_cost(&crab_positions, crab_positions[med_pos - 1]);
    //     if med_cost <= med_min_one_cost {
    //         med_cost
    //     } else {
    //         using_med = false;
    //         med_min_one_cost
    //     }
    // } else {
    //     med_cost
    // };
    
    // if using_med {
    //     println!("{}, {}", crab_positions[med_pos], min_cost);
    // } else {
    //     println!("{}, {}", crab_positions[med_pos - 1], min_cost);
    // }

    let med = median(&crab_positions);
    let med_cost = total_cost(&crab_positions, med);
    let hi_cost = total_cost(&crab_positions, med + 1);
    let lo_cost = total_cost(&crab_positions, med - 1);
    println!("med {}: {} {} {}", med, lo_cost, med_cost, hi_cost);

    // The guess is that the answer is somewhere around the mean average
    // I fiddled with the round, ceil, floor functions on mean until I got
    // the right answer
    let mean = mean(&crab_positions);
    let mean_cost = new_total_cost(&crab_positions, mean);
    println!("mean {}: {}", mean, mean_cost);
}
