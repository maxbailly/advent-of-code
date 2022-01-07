const INPUT: &str = "1321131112";
const STEPS: usize = 40;

/* ---------- */

fn next_sequence(seq: &[u8]) -> Vec<u8> {
    let mut next_seq = Vec::new();

    let mut cursor = 0usize;
    let len = seq.len();

    while cursor < len {
        let count = read(seq, len, seq[cursor], &mut cursor).to_string();

        next_seq.extend_from_slice(count.as_bytes());
        next_seq.push(seq[cursor]);
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

fn main() {
    let mut seq = String::from(INPUT);

    for _ in 0..STEPS {
        let new_seq = next_sequence(seq.as_bytes());
        seq = String::from_utf8(new_seq).expect("failed to convert new seq into string");
    }

    println!("result = {}", seq.len());
}
