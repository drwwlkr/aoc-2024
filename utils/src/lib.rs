use std::fs::File;
use std::io::{self, BufRead};

pub fn path_from_argv() -> String {
    let args: Vec<String> = std::env::args().collect();
    args.get(1).expect( "must provide file path!").to_string()
}
pub fn left_and_right_lists(input_path: &str) -> (Vec<i32>, Vec<i32>) {
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
    (left, right)
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
