use std::thread;

/* ---------- */

const INPUT: &str = "1321131112";
const STEPS_PART1: usize = 40;
const STEPS_PART2: usize = 50;

/* ---------- */

fn next_sequence(seq: &[u8]) -> Vec<u8> {
    let mut next_seq = Vec::new();

    let mut cursor = 0;
    let len = seq.len();

    while cursor < len {
        let current = seq[cursor];
        let count = read(seq, len, seq[cursor], &mut cursor).to_string();

        next_seq.extend_from_slice(count.as_bytes());
        next_seq.push(current);
    }

    next_seq
}

/* ---------- */

fn read(seq: &[u8], len: usize, curr_char: u8, cursor: &mut usize) -> usize {
    let mut count = 0usize;

    while *cursor < len && seq[*cursor] == curr_char {
        count += 1;
        *cursor += 1;
    }

    count
}

/* ---------- */

fn compute(steps: usize) -> usize {
    let mut seq = String::from(INPUT);

    for _ in 0..steps {
        let new_seq = next_sequence(seq.as_bytes());
        seq = String::from_utf8(new_seq).expect("failed to convert new seq into string");
    }

    seq.len()
}

/* ---------- */

fn main() {
    let part1_thread = thread::spawn(|| compute(STEPS_PART1));
    let part2_thread = thread::spawn(|| compute(STEPS_PART2));

    let answer_p1 = part1_thread
        .join()
        .expect("part 1 thread failed at some point");
    let answer_p2 = part2_thread
        .join()
        .expect("part 2 thread failed at some point");

    println!("[PART 1] Answer = {answer_p1}");
    println!("[PART 2] Answer = {answer_p2}");
}
