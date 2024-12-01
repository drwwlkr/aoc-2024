use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input_path = args.get(1).expect( "must provide file path!");

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    if let Ok(file) = File::open(input_path) {
        let reader = io::BufReader::new(file);
        for line in reader.lines().flatten() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            left.push(parts[0].parse().unwrap());
            right.push(parts[1].parse().unwrap());
        }
    }

    left.sort();
    right.sort();

    let iter = left.iter().zip(right.iter());
    let mut dif = 0;
    for (&l, &r) in iter {
        dif += (l - r).abs();
    }
    println!("{}", dif);
}
