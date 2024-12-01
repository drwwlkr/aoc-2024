use utils;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input_path = args.get(1).expect( "must provide file path!");

    let (mut left, mut right) = utils::left_and_right_lists(input_path);

    left.sort();
    right.sort();
    let dif: i32 = left.iter().zip(right.iter())
        .map(|(l, r)| l - r)
        .map(i32::abs)
        .sum();

    println!("{}", dif);
}
