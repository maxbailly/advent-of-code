fn main() {
    let input = utils::input_bytes!("part1.txt");

    let ret = input.iter()
        .map(|c | {
            match c {
                b'(' => 1,
                b')' => -1,
                _ => 0
            }
        })
        .sum::<i32>();

    println!("{}", ret);
}
