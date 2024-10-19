pub mod language;

#[macro_export]
macro_rules! TEST_BIT {
    ($flags:expr, $bit:expr) => {
        ((($flags) & (1 << ($bit))) != 0)
    };
}