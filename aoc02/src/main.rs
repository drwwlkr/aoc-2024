fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input_path = args.get(1).expect( "must provide file path!");

    let (mut left, mut right) = utils::left_and_right_lists(input_path);

    let mut similarity = 0;

    left.sort();
    right.sort();
    let mut left_idx = 0;
    let mut right_idx = 0;
    while left_idx < left.len() {
        let left_val = left[left_idx];
        let mut times_matched = 0;
        while right_idx < right.len() && right[right_idx] <= left_val  {
            if right[right_idx] == left_val {
                times_matched += 1;
            }
            right_idx += 1;
        }
        similarity = similarity + (left_val * times_matched);
        left_idx += 1;
    }
    print!("{}", similarity);
}
