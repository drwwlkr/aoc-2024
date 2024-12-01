use utils;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input_path = args.get(1).expect( "must provide file path!");

    let (left, right) = utils::left_and_right_lists(input_path);
    let iter = left.iter().zip(right.iter());
    let mut dif = 0;
    for (&l, &r) in iter {
        dif += (l - r).abs();
    }
    println!("{}", dif);
}
