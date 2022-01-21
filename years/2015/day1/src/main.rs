const INPUT: &[u8] = utils::input_bytes!("part1.txt");

fn part1() -> i32 {
    INPUT.iter()
        .map(|c | {
            match c {
                b'(' => 1,
                b')' => -1,
                _ => 0
            }
        })
        .sum::<i32>()
}

/* ---------- */

fn part2() -> usize {
    let mut position = 0;

    for (index, character) in INPUT.iter().enumerate() {
        match character {
            b'(' => position += 1,
            b')' => position -= 1,
            _ => ()
        }

        if position == -1 {
            return index + 1
        }
    }

    panic!("Santa never went to basement -1")
}

/* ---------- */

fn main() {
    println!("[PART 1] Answer = {}", part1());
    println!("[PART 1] Answer = {}", part2())
}
