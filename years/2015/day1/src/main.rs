fn main() {
    let input = utils::inputs_bytes!("1");

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
