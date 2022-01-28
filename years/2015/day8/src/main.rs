#[inline(always)]
fn is_ascii_numeric(c: &u8) -> bool {
    *c >= b'0' && *c <= b'9'
}

/* ---------- */

fn escape(idx: &mut usize, bytes: &[u8]) {
    *idx += 1;

    if bytes[*idx] == b'x' {
        *idx += 1;
    }

    while *idx < bytes.len() && is_ascii_numeric(&bytes[*idx]) {
        *idx += 1;
    }
}

/* ---------- */

fn count_mem_chars(line: &str, len: usize) -> u8 {
    let bytes = line.as_bytes();
    let mut idx = 0usize;
    let mut count = 0u8;

    while idx < len {
        match bytes[idx] {
            b'\\' => {
                escape(&mut idx, bytes);
                count += 1
            },
            b'"' => (),
            _ => count += 1
        }

        idx += 1;
    }

    count
}

/* ---------- */

fn main() {
    let count = utils::input_str!("part1.txt").lines()
        .map(|line| {
            let code_len = line.len();
            let count = count_mem_chars(line, code_len);

            code_len - count as usize
        })
        .sum::<usize>();

    println!("result = {}", count);
}
