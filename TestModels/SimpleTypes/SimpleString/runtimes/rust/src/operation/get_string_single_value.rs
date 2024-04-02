// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Orchestration and serialization glue logic for `GetStringSingleValue`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct GetStringSingleValue;
impl GetStringSingleValue {
    /// Creates a new `GetStringSingleValue`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn send(
        handle: &crate::client::Handle,
        input: crate::operation::get_string_single_value::GetStringSingleValueInput,
    ) -> ::std::result::Result<
        crate::operation::get_string_single_value::GetStringSingleValueOutput,
        crate::operation::get_string_single_value::GetStringSingleValueError
    > {
        let inner_input = crate::conversions::get_string_single_value::_get_string_single_value_input::to_dafny(input);
        let inner_result = unsafe { (*handle.inner).GetStringSingleValue(&inner_input) };
        match &*inner_result {
            crate::implementation_from_dafny::r#_Wrappers_Compile::Result::Success { value } => 
                Ok(crate::conversions::get_string_single_value::_get_string_single_value_output::from_dafny(value)),
            crate::implementation_from_dafny::r#_Wrappers_Compile::Result::Failure { error } =>
                Err(crate::conversions::get_string_single_value::from_dafny_error(error.clone())), // TODO: Why is clone necessary here but not for from_dafny
            crate::implementation_from_dafny::r#_Wrappers_Compile::Result::_PhantomVariant(_, _) => panic!("Unreachable")
        }
    }
}

/// Error type for the `GetStringSingleValue` operation.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum GetStringSingleValueError {
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
    variable wildcard pattern and check `.code()`:
     \
    &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
     \
    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-GetStringSingleValueError) for what information is available for the error.")]
    Unhandled(crate::error::sealed_unhandled::Unhandled),
}
impl GetStringSingleValueError {
    /// Creates the `GetStringSingleValueError::Unhandled` variant from any error type.
    pub fn unhandled(
        err: impl ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>>,
    ) -> Self {
        Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
            source: err.into(),
            meta: ::std::default::Default::default(),
        })
    }

    /// Creates the `GetStringSingleValueError::Unhandled` variant from an [`ErrorMetadata`](::aws_smithy_types::error::ErrorMetadata).
    pub fn generic(err: ::aws_smithy_types::error::ErrorMetadata) -> Self {
        Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
            source: err.clone().into(),
            meta: err,
        })
    }
    ///
    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    ///
    pub fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::Unhandled(e) => &e.meta,
        }
    }
}
impl ::std::error::Error for GetStringSingleValueError {
    fn source(&self) -> ::std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Self::Unhandled(_inner) => ::std::option::Option::Some(&*_inner.source),
        }
    }
}
impl ::std::fmt::Display for GetStringSingleValueError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Unhandled(_inner) => {
                if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self) {
                    write!(f, "unhandled error ({code})")
                } else {
                    f.write_str("unhandled error")
                }
            }
        }
    }
}
impl ::aws_smithy_types::retry::ProvideErrorKind for GetStringSingleValueError {
    fn code(&self) -> ::std::option::Option<&str> {
        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> ::std::option::Option<::aws_smithy_types::retry::ErrorKind> {
        ::std::option::Option::None
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for GetStringSingleValueError {
    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::Unhandled(_inner) => &_inner.meta,
        }
    }
}
impl ::aws_smithy_runtime_api::client::result::CreateUnhandledError for GetStringSingleValueError {
    fn create_unhandled_error(
        source: ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>,
        meta: ::std::option::Option<::aws_smithy_types::error::ErrorMetadata>,
    ) -> Self {
        Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
            source,
            meta: meta.unwrap_or_default(),
        })
    }
}

use core::panic;

pub use crate::operation::get_string_single_value::_get_string_single_value_output::GetStringSingleValueOutput;

pub use crate::operation::get_string_single_value::_get_string_single_value_input::GetStringSingleValueInput;

mod _get_string_single_value_input;

mod _get_string_single_value_output;

/// Builders
pub mod builders;
