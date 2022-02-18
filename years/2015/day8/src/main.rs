const INPUT: &[u8] = utils::input_bytes!();

/* ---------- */

const fn part1(input: &[u8]) -> usize {
    let len = input.len();
    let mut cursor = 0;
    let mut diff = 0;

    while cursor < len {
        match input[cursor] {
            b'"' => diff += 1,
            b'\\' if matches!(input[cursor + 1], b'"' | b'\\') =>  {
                diff += 1;
                cursor += 1;
            },
            b'\\' if input[cursor + 1] == b'x' =>  {
                diff += 3;
                cursor += 3;
            },
            _ => ()
        }

        cursor += 1;
    }

    diff
}

/* ---------- */

const fn part2(input: &[u8]) -> usize {
    let len = input.len();
    let mut cursor = 0;
    let mut diff = 0;

    while cursor < len {
        match input[cursor] {
            b'"' => diff += 1,
            b'\\' => diff += 1,
            b'\r' if input[cursor + 1] == b'\n' => {
                diff += 2;
                cursor += 1;
            }
            b'\n' | b'\r' => diff += 2,
            _ => ()
        }

        cursor += 1;
    }

    if !matches!(input[len - 1], b'\n' | b'\r') {
        diff += 2
    }

    diff
}

/* ---------- */

fn main() {
    utils::answer!(INPUT)
}

/* ---------- */

#[cfg(test)]
mod tests {
    #[test]
    fn test_part2() {
        use crate::part2;

        assert_eq!(part2(r#""""#.as_bytes()), 4);
        assert_eq!(part2(r#""abc""#.as_bytes()), 4);
        assert_eq!(part2(r#""aaa\"aaa""#.as_bytes()), 6);
        assert_eq!(part2(r#""\x27""#.as_bytes()), 5);
    }
}
