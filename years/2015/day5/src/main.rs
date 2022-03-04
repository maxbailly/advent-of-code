const fn count_occurence(needle: &[u8], haystack: &[u8]) -> usize {
    let len = haystack.len() - 1;
    let mut idx = 0usize;
    let mut count = 0usize;

    while idx < len {
        if needle[0] == haystack[idx] && needle[1] == haystack[idx + 1] {
            count += 1
        }

        idx += 1;
    }

    count
}

/* ---------- */

#[derive(Debug, PartialEq)]
enum StringType {
    Nice,
    Naughty,
}

impl StringType {
    fn from_using_part1_rules(line: &'static str) -> Self {
        let mut vowels = 0u8;
        let mut double = false;

        let mut prev = &b'\0';

        for c in line.as_bytes() {
            if *prev == b'\0' {
                prev = c;
            } else if prev == c {
                double = true
            }

            match c {
                b'a' | b'e' | b'i' | b'o' | b'u' => vowels += 1,
                b'b' if *prev == b'a' => return Self::Naughty,
                b'd' if *prev == b'c' => return Self::Naughty,
                b'q' if *prev == b'p' => return Self::Naughty,
                b'y' if *prev == b'x' => return Self::Naughty,
                _ => (),
            }

            prev = c;
        }

        if vowels < 3 || !double {
            return Self::Naughty;
        }

        Self::Nice
    }

    fn from_using_part2_rules(line: &'static str) -> Self {
        let line = line.as_bytes();
        let line_len = line.len();

        let mut pair_found_twice = false;
        let mut in_between = false;

        for index in 0..line_len {
            // First rule
            if index < line_len - 3 && !pair_found_twice {
                let needle = &line[index..=index + 1];
                let haystack = &line[index + 2..];

                pair_found_twice = count_occurence(needle, haystack) >= 1;
            }

            // Second rule
            if index > 0 && index < line_len - 1 && line.get(index - 1) == line.get(index + 1) {
                in_between = true;
            }

            if pair_found_twice && in_between {
                return Self::Nice;
            }
        }

        Self::Naughty
    }
}

/* ---------- */

fn part1(input: &'static str) -> usize {
    input
        .lines()
        .map(StringType::from_using_part1_rules)
        .filter(|str_type| matches!(str_type, StringType::Nice))
        .count()
}

/* ---------- */

fn part2(input: &'static str) -> usize {
    input
        .lines()
        .map(StringType::from_using_part2_rules)
        .filter(|str_type| matches!(str_type, StringType::Nice))
        .count()
}

/* ---------- */

fn main() {
    let input = utils::input_str!();
    utils::answer!(input)
}

/* ---------- */

#[cfg(test)]
mod test {
    use crate::StringType;

    #[test]
    fn test_part1() {
        assert_eq!(
            StringType::from_using_part1_rules("ugknbfddgicrmopn"),
            StringType::Nice
        );
        assert_eq!(StringType::from_using_part1_rules("aaa"), StringType::Nice);
        assert_eq!(
            StringType::from_using_part1_rules("jchzalrnumimnmhp"),
            StringType::Naughty
        );
        assert_eq!(
            StringType::from_using_part1_rules("haegwjzuvuyypxyu"),
            StringType::Naughty
        );
        assert_eq!(
            StringType::from_using_part1_rules("dvszwmarrgswjxmb"),
            StringType::Naughty
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            StringType::from_using_part2_rules("qjhvhtzxzqqjkmpb"),
            StringType::Nice
        );
        assert_eq!(
            StringType::from_using_part2_rules("xxyxx"),
            StringType::Nice
        );
        assert_eq!(
            StringType::from_using_part2_rules("uurcxstgmygtbstg"),
            StringType::Naughty
        );
        assert_eq!(
            StringType::from_using_part2_rules("ieodomkazucvgmuy"),
            StringType::Naughty
        );
    }
}
