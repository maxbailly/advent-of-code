use md5::{Md5, Digest};

const INPUT: &str = "bgvyzdsv";

/* ---------- */

fn part1() -> u32 {
    let mut count = 0;
    let mut hasher = Md5::new();

    loop {
        let test = format!("{}{}", INPUT, count);

        hasher.update(test.as_bytes());
        let result = hasher.finalize_reset();

        if result[0] == 0 && result[1] == 0 && result[2] < 16 {
            return count;
        } else {
            count += 1;
        }
    }
}

/* ---------- */

fn part2() -> u32 {
    let mut count = 0;
    let mut hasher = Md5::new();

    loop {
        let test = format!("{}{}", INPUT, count);

        hasher.update(test.as_bytes());
        let result = hasher.finalize_reset();

        if result[0] == 0 && result[1] == 0 && result[2] == 0 {
            return count;
        } else {
            count += 1;
        }
    }
}

/* ---------- */

fn main() {
    utils::answer!()
}
