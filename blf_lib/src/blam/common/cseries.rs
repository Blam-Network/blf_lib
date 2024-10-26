pub mod language;

#[macro_export]
macro_rules! FLAG {
    ($bit:expr) => {
        (1 << ($bit))
    };
}


#[macro_export]
macro_rules! TEST_BIT {
    ($flags:expr, $bit:expr) => {
        ((($flags) & (1 << ($bit))) != 0)
    };
}

#[macro_export]
macro_rules! SET_BIT {
    ($flags:expr, $bit:expr, $value:expr) => {
        if $value { $flags |= (1 << ($bit)); } else { $flags &= !(1 << ($bit)); }
    };
}
