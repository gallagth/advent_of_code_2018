use std::collections::HashMap;

fn main() {
    // Part 1
    let puzzle = include_str!("input.txt");
    let words: Vec<&str> = puzzle.lines().collect();
    let mut twos: u64 = 0;
    let mut threes: u64 = 0;
    for word in words {
        let mut char_count = HashMap::new();
        for c in word.chars() {
            let count = char_count.entry(c).or_insert(0);
            *count += 1;
        }
        if char_count.values().any(|&count| count == 2) {
            twos += 1;
        }
        if char_count.values().any(|&count| count == 3) {
            threes += 1;
        }
    }
    println!("checksum = {:?}", twos * threes);
}
