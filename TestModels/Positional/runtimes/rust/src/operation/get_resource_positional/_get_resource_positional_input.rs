// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetResourcePositionalInput {
    #[allow(missing_docs)] // documentation missing in model
pub name: ::std::option::Option<::std::string::String>,
}
impl GetResourcePositionalInput {
    #[allow(missing_docs)] // documentation missing in model
pub fn name(&self) -> &::std::option::Option<::std::string::String> {
    &self.name
}
}
impl GetResourcePositionalInput {
    /// Creates a new builder-style object to manufacture [`GetResourcePositionalInput`](crate::operation::get_resource_positional::builders::GetResourcePositionalInput).
    pub fn builder() -> crate::operation::get_resource_positional::builders::GetResourcePositionalInputBuilder {
        crate::operation::get_resource_positional::builders::GetResourcePositionalInputBuilder::default()
    }
}

/// A builder for [`GetResourcePositionalInput`](crate::operation::operation::GetResourcePositionalInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetResourcePositionalInputBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
}
impl GetResourcePositionalInputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.name = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.name = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.name
}
    /// Consumes the builder and constructs a [`GetResourcePositionalInput`](crate::operation::operation::GetResourcePositionalInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_resource_positional::GetResourcePositionalInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::get_resource_positional::GetResourcePositionalInput {
            name: self.name,
        })
    }
}
