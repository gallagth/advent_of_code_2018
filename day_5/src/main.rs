use std::collections::VecDeque;

fn are_opposites(c1: char, c2: char) -> bool {
    (c1 as i32 - c2 as i32).abs() == 0x20
}

fn collapse(puzzle: &str) -> usize {
    let chars: Vec<char> = puzzle.chars().collect();
    let mut tail: VecDeque<char> = VecDeque::new();
    for c in chars {
        if tail.is_empty() {
            tail.push_back(c);
            continue;
        }
        if are_opposites(*tail.back().unwrap(), c) {
            tail.pop_back();
        } else {
            tail.push_back(c);
        }
    }
    tail.len()
}

fn main() {
    let puzzle = include_str!("input.txt").trim();
    println!("{:?}", puzzle.len());
    println!("{:?}", collapse(puzzle));
}
