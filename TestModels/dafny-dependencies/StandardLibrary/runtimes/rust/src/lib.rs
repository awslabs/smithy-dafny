#![allow(
    deprecated,
    non_upper_case_globals,
    unused,
    non_snake_case,
    non_camel_case_types
)]

pub(crate) mod implementation_from_dafny;
/// All operations that this crate can perform.
pub mod UTF8_externs;
pub mod conversion;
pub(crate) use crate::implementation_from_dafny::_Wrappers_Compile;
pub(crate) use crate::implementation_from_dafny::UTF8;