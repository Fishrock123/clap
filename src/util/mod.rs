#![allow(clippy::single_component_path_imports)]

mod flat_map;
mod flat_set;
mod graph;
mod id;
mod os_str;
mod str;
mod str_to_bool;

pub use self::id::Id;
pub use self::os_str::OsStr;
pub use self::str::Str;

pub(crate) use self::flat_map::Entry;
pub(crate) use self::flat_map::FlatMap;
pub(crate) use self::flat_set::FlatSet;
pub(crate) use self::graph::ChildGraph;
pub(crate) use self::str_to_bool::str_to_bool;
pub(crate) use self::str_to_bool::FALSE_LITERALS;
pub(crate) use self::str_to_bool::TRUE_LITERALS;

pub(crate) mod color;

pub(crate) const SUCCESS_CODE: i32 = 0;
// While sysexists.h defines EX_USAGE as 64, this doesn't seem to be used much in practice but
// instead 2 seems to be frequently used.
// Examples
// - GNU `ls` returns 2
// - Python's `argparse` returns 2
pub(crate) const USAGE_CODE: i32 = 2;

pub(crate) fn safe_exit(code: i32) -> ! {
    use std::io::Write;

    let _ = std::io::stdout().lock().flush();
    let _ = std::io::stderr().lock().flush();

    std::process::exit(code)
}

#[cfg(not(feature = "unicode"))]
pub(crate) fn eq_ignore_case(left: &str, right: &str) -> bool {
    left.eq_ignore_ascii_case(right)
}

#[cfg(feature = "unicode")]
pub(crate) use unicase::eq as eq_ignore_case;
