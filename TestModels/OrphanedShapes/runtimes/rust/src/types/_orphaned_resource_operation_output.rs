// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct OrphanedResourceOperationOutput {
    #[allow(missing_docs)]
pub some_string: ::std::option::Option<::std::string::String>,
}
impl OrphanedResourceOperationOutput {
    #[allow(missing_docs)]
pub fn some_string(&self) -> &::std::option::Option<::std::string::String> {
    &self.some_string
}
}
impl OrphanedResourceOperationOutput {
    /// Creates a new builder-style object to manufacture [`OrphanedResourceOperationOutput`](crate::types::OrphanedResourceOperationOutput).
    pub fn builder() -> crate::types::builders::OrphanedResourceOperationOutputBuilder {
        crate::types::builders::OrphanedResourceOperationOutputBuilder::default()
    }
}

/// A builder for [`OrphanedResourceOperationOutput`](crate::types::OrphanedResourceOperationOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct OrphanedResourceOperationOutputBuilder {
    pub(crate) some_string: ::std::option::Option<::std::string::String>,
}
impl OrphanedResourceOperationOutputBuilder {
    #[allow(missing_docs)]
pub fn some_string(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.some_string = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)]
pub fn set_some_string(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.some_string = input;
    self
}
#[allow(missing_docs)]
pub fn get_some_string(&self) -> &::std::option::Option<::std::string::String> {
    &self.some_string
}
    /// Consumes the builder and constructs a [`OrphanedResourceOperationOutput`](crate::types::OrphanedResourceOperationOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::types::OrphanedResourceOperationOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::types::OrphanedResourceOperationOutput {
            some_string: self.some_string,
        })
    }
}
