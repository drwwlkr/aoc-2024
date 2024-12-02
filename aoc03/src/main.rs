use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::BufRead;
use crate::Safety::{Decreasing, Increasing, Unsafe};

fn main() {
    let path = utils::path_from_argv();
    let mut safe_count = 0;
    if let Ok(file) = File::open(path) {
        for line in io::BufReader::new(file).lines().flatten() {
            let report: Vec<i32> = line.split_whitespace()
                .map(|l| l.parse::<i32>().unwrap())
                .collect();
            if is_report_safe(&report) {
                safe_count += 1;
            }
        }
    }
    println!("{}", safe_count);

}

#[derive(Hash,Eq, PartialEq)]
enum Safety {
    Increasing,
    Decreasing,
    Unsafe,
}

fn is_report_safe(report: &Vec<i32>) -> bool {
    let status: HashSet<Safety> = report.windows(2).map(|w| {
        let diff = w[1] - w[0];
        if diff.abs() > 3 || diff == 0{
            Unsafe
        } else if diff > 0 {
            Increasing
        } else {
            Decreasing
        }
    }).collect();
    return status.len() == 1 && !status.contains(&Unsafe);
}
