//! Define [`Command`] line [arguments][`Arg`]

mod action;
mod app_settings;
mod arg;
mod arg_group;
mod arg_predicate;
mod arg_settings;
mod command;
mod possible_value;
mod range;
mod value_hint;
mod value_parser;

#[cfg(debug_assertions)]
mod debug_asserts;

#[cfg(test)]
mod tests;

pub use action::ArgAction;
pub use arg::Arg;
pub use arg_group::ArgGroup;
pub use arg_predicate::ArgPredicate;
pub use command::Command;
pub use possible_value::PossibleValue;
pub use range::ValueRange;
pub use value_hint::ValueHint;
pub use value_parser::_AutoValueParser;
pub use value_parser::via_prelude;
pub use value_parser::BoolValueParser;
pub use value_parser::BoolishValueParser;
pub use value_parser::EnumValueParser;
pub use value_parser::FalseyValueParser;
pub use value_parser::NonEmptyStringValueParser;
pub use value_parser::OsStringValueParser;
pub use value_parser::PathBufValueParser;
pub use value_parser::PossibleValuesParser;
pub use value_parser::RangedI64ValueParser;
pub use value_parser::RangedU64ValueParser;
pub use value_parser::StringValueParser;
pub use value_parser::TypedValueParser;
pub use value_parser::ValueParser;
pub use value_parser::ValueParserFactory;
pub use value_parser::_AnonymousValueParser;

pub(crate) use action::CountType;
pub(crate) use arg::render_arg_val;
pub(crate) use arg_settings::{ArgFlags, ArgSettings};
