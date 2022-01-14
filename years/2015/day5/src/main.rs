enum StringType {
    Nice,
    Naughty
}

impl From<&'static str> for StringType {
    fn from(line: &'static str) -> Self {
        let mut vowels = 0u8;
        let mut double = false;

        let mut prev = &b'\0';

        for c in line.as_bytes() {
            if *prev == b'\0' {
                prev = c;
                continue;
            }

            if c == prev {
                double = true
            }

            match c {
                b'a' | b'e' | b'i' | b'o' | b'u' => vowels += 1,
                b'b' if *prev == b'a' => return Self::Naughty,
                b'd' if *prev == b'c' => return Self::Naughty,
                b'q' if *prev == b'p' => return Self::Naughty,
                b'y' if *prev == b'x' => return Self::Naughty,
                _ => ()
            }

            prev = c;
        }

        if vowels < 3 || !double {
            return Self::Naughty
        }

        Self::Nice
    }
}

fn main() {
    let count = utils::input_str!("part1.txt").split_whitespace()
        .map(StringType::from)
        .filter(|str_type| matches!(str_type, StringType::Nice))
        .count();

    println!("result = {}", count);
}
