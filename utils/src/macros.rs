#[macro_export]
macro_rules! input_bytes {
    ($name: literal) => {
        std::include_bytes!(std::concat!("inputs/", $name))
    };
}

#[macro_export]
macro_rules! input_str {
    ($name: literal) => {
        std::include_str!(std::concat!("inputs/", $name)).trim()
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
