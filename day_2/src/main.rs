use std::collections::HashMap;

fn check_two_threes(word: &str) -> Vec<bool> {
    let mut char_count = HashMap::new();
    let mut vec = vec![false, false];

    for c in word.chars() {
        let count = char_count.entry(c).or_insert(0);
        *count += 1;
    }
    for (_c, count) in char_count {
        if count == 2 {
            vec[0] = true;
        } else if count == 3 {
            vec[1] = true;
        }
    }
    return vec;
}

fn main() {
    // Part 1
    let puzzle = include_str!("input.txt");
    let words: Vec<&str> = puzzle.lines().collect();
    let mut twos: u64 = 0;
    let mut threes: u64 = 0;
    for word in words {
        let two_threes = check_two_threes(word);
        if two_threes[0] {
            twos += 1;
        }
        if two_threes[1] {
            threes += 1;
        }
    }
    println!("checksum = {:?}", twos * threes);
}
