use std::collections::HashMap;

fn main() {
    // Part 1
    let puzzle = include_str!("input.txt");
    let values: Vec<i64> = puzzle.lines().map(|x| x.parse::<i64>().unwrap()).collect();
    let freq_sum: i64 = values.iter().sum();
    println!("{}", freq_sum);

    // Part 2
    let mut frequencies = HashMap::new();
    let mut current_freq = 0;
    let mut found = false;
    while !found {
        for v in values.iter() {
            frequencies.insert(current_freq, true);
            current_freq += v;
            if frequencies.contains_key(&current_freq) {
                found = true;
                break;
            }
        }
    }
    println!("{:?}", current_freq);
}
