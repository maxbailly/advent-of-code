#[macro_export]
macro_rules! input_bytes {
    ($num: literal) => {
        std::include_bytes!(std::concat!("inputs/part", $num, ".txt"))
    };
}

#[macro_export]
macro_rules! input_str {
    ($num: literal) => {
        std::include_str!(std::concat!("inputs/part", $num, ".txt")).trim()
    };
}

#[macro_export]
macro_rules! bench {
    ($e: expr) => {
        use std::time::Instant;

        let now = Instant::now();

        $e;

        let elapsed = now.elapsed();
        dbg!(elapsed);
    };
}
