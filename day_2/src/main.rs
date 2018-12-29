use std::collections::HashMap;

fn words_differ_by_one(w1: &str, w2: &str) -> bool {
    let num_different = w1.chars()
        .zip(w2.chars())
        .filter(|(a, b)| a != b)
        .count();

    num_different == 1
}

fn matching_chars(w1: &str, w2: &str) -> String {
    w1.chars()
        .zip(w2.chars())
        .filter(|(a, b)| a == b)
        .map(|(a, _)| a)
        .collect()
}

fn main() {
    // Part 1
    let puzzle = include_str!("input.txt");
    let words: Vec<&str> = puzzle.lines().collect();
    let mut twos: u64 = 0;
    let mut threes: u64 = 0;
    for word in &words {
        let mut char_count = HashMap::with_capacity(26);
        for c in word.chars() {
            *char_count.entry(c).or_insert(0) += 1;
        }
        if char_count.values().any(|&count| count == 2) {
            twos += 1;
        }
        if char_count.values().any(|&count| count == 3) {
            threes += 1;
        }
    }
    println!("checksum = {:?}", twos * threes);
    // Part 2
    for i in 0..words.len() {
        for j in i+1..words.len() {
            if words_differ_by_one(words[i], words[j]) {
                println!("{:?}", matching_chars(words[i], words[j]));
            }
        }
    }
}
