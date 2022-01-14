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
