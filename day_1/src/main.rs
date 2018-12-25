use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};


fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    // Part 1
    let file = File::open(filename)?;
    let mut freq: i64 = 0;
    for line in BufReader::new(file).lines() {
        let freq_shift = line?.parse::<i64>().unwrap();
        freq += freq_shift;
    }
    println!("{}", freq);

    // Part 2

    Ok(())
}
