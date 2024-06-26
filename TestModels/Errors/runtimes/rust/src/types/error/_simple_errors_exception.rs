// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SimpleErrorsException  {
    #[allow(missing_docs)] // documentation missing in model
    // TODO: smithy-rs client generation makes this an Option<String> instead,
    // but that's not compatible with Dafny which makes it a plain string.
    pub message: ::std::string::String,
    pub(crate) meta: ::aws_smithy_types::error::ErrorMetadata,
}
impl SimpleErrorsException {
    /// Returns the error message.
                        pub fn message(&self) -> String { self.message }
}
impl ::std::fmt::Display for SimpleErrorsException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        ::std::write!(f, "SimpleErrorsException")?;
        ::std::write!(f, ": {}", self.message)?;
        Ok(())
    }
}
impl ::std::error::Error for SimpleErrorsException {}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for SimpleErrorsException {
    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata { &self.meta }
}
impl SimpleErrorsException {
    /// Creates a new builder-style object to manufacture [`SimpleErrorsException`](crate::types::error::SimpleErrorsException).
    pub fn builder() -> crate::types::error::builders::SimpleErrorsExceptionBuilder {
        crate::types::error::builders::SimpleErrorsExceptionBuilder::default()
    }
}

/// A builder for [`SimpleErrorsException`](crate::types::error::SimpleErrorsException).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct SimpleErrorsExceptionBuilder {
    pub(crate) message: ::std::option::Option<::std::string::String>,
    meta: std::option::Option<::aws_smithy_types::error::ErrorMetadata>,
}
impl SimpleErrorsExceptionBuilder {
    #[allow(missing_docs)] // documentation missing in model
    pub fn message(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.message = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_message(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.message = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_message(&self) -> &::std::option::Option<::std::string::String> {
        &self.message
    }
    /// Sets error metadata
                                                pub fn meta(mut self, meta: ::aws_smithy_types::error::ErrorMetadata) -> Self {
                                                    self.meta = Some(meta);
                                                    self
                                                }
    
                                                /// Sets error metadata
                                                pub fn set_meta(&mut self, meta: std::option::Option<::aws_smithy_types::error::ErrorMetadata>) -> &mut Self {
                                                    self.meta = meta;
                                                    self
                                                }
    /// Consumes the builder and constructs a [`SimpleErrorsException`](crate::types::error::SimpleErrorsException).
    pub fn build(self) -> crate::types::error::SimpleErrorsException {
        crate::types::error::SimpleErrorsException {
            // TODO: Constraints validation
            message: self.message.unwrap_or_default()
            ,
            meta: self.meta.unwrap_or_default(),
        }
    }
}

