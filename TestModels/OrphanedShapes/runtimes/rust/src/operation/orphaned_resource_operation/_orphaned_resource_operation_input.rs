// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct OrphanedResourceOperationInput {
    #[allow(missing_docs)]
pub some_string: ::std::option::Option<::std::string::String>,
}
impl OrphanedResourceOperationInput {
    #[allow(missing_docs)]
pub fn some_string(&self) -> &::std::option::Option<::std::string::String> {
    &self.some_string
}
}
impl OrphanedResourceOperationInput {
    /// Creates a new builder-style object to manufacture [`OrphanedResourceOperationInput`](crate::operation::orphaned_resource_operation::builders::OrphanedResourceOperationInput).
    pub fn builder() -> crate::operation::orphaned_resource_operation::builders::OrphanedResourceOperationInputBuilder {
        crate::operation::orphaned_resource_operation::builders::OrphanedResourceOperationInputBuilder::default()
    }
}

/// A builder for [`OrphanedResourceOperationInput`](crate::operation::operation::OrphanedResourceOperationInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct OrphanedResourceOperationInputBuilder {
    pub(crate) some_string: ::std::option::Option<::std::string::String>,
}
impl OrphanedResourceOperationInputBuilder {
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
    /// Consumes the builder and constructs a [`OrphanedResourceOperationInput`](crate::operation::operation::OrphanedResourceOperationInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::orphaned_resource_operation::OrphanedResourceOperationInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::orphaned_resource_operation::OrphanedResourceOperationInput {
            some_string: self.some_string,
        })
    }
}
