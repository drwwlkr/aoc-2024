use std::collections::HashMap;

fn main() {


    let (left, right) = utils::left_and_right_lists(&utils::path_from_argv());

    let mut right_counts = HashMap::new();

    let mut similarity = 0;

    for right_val in right {
        let count = right_counts.entry(right_val).or_insert(0);
        *count += 1;
    }
    for left_val in left {
        similarity += left_val * right_counts.get(&left_val).unwrap_or(&0);
    }

    print!("{}", similarity);
}
