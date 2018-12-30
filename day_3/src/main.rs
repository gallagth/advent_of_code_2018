extern crate regex;
#[macro_use] extern crate lazy_static;

use regex::Regex;
use std::fmt;
use std::collections::HashSet;

struct Input {
    id: u32,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl Input {
    pub fn from_string(input: String) -> Input {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
        }
        let groups = RE.captures(&input).unwrap();
        Input {
            id:     groups[1].parse().unwrap(),
            x:      groups[2].parse().unwrap(),
            y:      groups[3].parse().unwrap(),
            width:  groups[4].parse().unwrap(),
            height: groups[5].parse().unwrap()
        }
    }
}

impl fmt::Debug for Input {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "@{},{} {}x{}", self.x, self.y, self.width, self.height)
    }
}

fn main() {
    let puzzle = include_str!("input.txt");
    let words = puzzle.lines();
    let mut inputs = Vec::new();
    for w in words {
        inputs.push(Input::from_string(w.to_string()));
    }
    let mut grid: Vec<Vec<_>> = vec![vec![HashSet::new(); 1000]; 1000];
    let mut all_ids = HashSet::new();
    let mut surface: u32 = 0;
    for i in inputs {
        all_ids.insert(i.id);
        for x in i.x..i.x+i.width {
            for y in i.y..i.y+i.height {
                grid[x][y].insert(i.id);
            }
        }
    }
    for x in 0..1000 {
        for y in 0..1000 {
            if grid[x][y].len() > 1 {
                surface += 1;
                for id in &grid[x][y] {
                    all_ids.remove(&id);
                }
            }
        }
    }
    println!("surface = {:?}", surface);
    println!("single = {:?}", all_ids);
}
