macro_rules! error_panic {
    ($($arg:tt)*) => {{
        log::error!($($arg)*);
        panic!($($arg)*);
    }};
}
pub(super) use error_panic;

macro_rules! log_wrapping_add {
    ($level:ident, $orig:expr, $off:expr) => {{
        let (result, overflow) = $orig.overflowing_add($off);
        if overflow {
            log::$level!(
                "Add resulted in overflow: ${:X} + ${:X} => ${:X}",
                $orig,
                $off,
                result
            );
        }
        result
    }};
}
pub(super) use log_wrapping_add;

macro_rules! warn_wrapping_add {
    ($orig:expr, $off:expr) => {
        crate::logging::log_wrapping_add!(warn, $orig, $off)
    };
}
pub(super) use warn_wrapping_add;

macro_rules! word_fmt {
    ($address:expr) => {
        format!("${:0>4X}", $address)
    };
}
pub(super) use word_fmt;

macro_rules! byte_fmt {
    ($byte:expr) => {
        format!("${:0>2X}", $byte)
    };
}
pub(super) use byte_fmt;
