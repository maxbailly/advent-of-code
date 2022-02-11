#[macro_export]
macro_rules! input_bytes {
    ($name: literal) => {
        std::include_bytes!(std::concat!("../inputs/", $name))
    };

    () => {
        std::include_bytes!("../inputs/input.txt")
    };
}

/* ---------- */

#[macro_export]
macro_rules! input_str {
    ($name: literal) => {
        std::include_str!(std::concat!("../inputs/", $name))
    };

    () => {
        std::include_str!("../inputs/input.txt")
    };
}

/* ---------- */

#[macro_export]
macro_rules! bench {
    ($e: expr) => {{
        use std::time::Instant;

        let now = Instant::now();

        $e;

        let elapsed = now.elapsed();
        dbg!(elapsed);
    }};
}

/* ---------- */

#[macro_export]
macro_rules! answer {
    ($input:expr) => {{
        println!("[PART 1] Answer = {}", part1($input));
        println!("[PART 2] Answer = {}", part2($input));
    }};

    () => {{
        println!("[PART 1] Answer = {}", part1());
        println!("[PART 2] Answer = {}", part2());
    }};
}
