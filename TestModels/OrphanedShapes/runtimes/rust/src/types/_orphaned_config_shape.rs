// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct OrphanedConfigShape {
    #[allow(missing_docs)]
pub string_member: ::std::option::Option<::std::string::String>,
}
impl OrphanedConfigShape {
    #[allow(missing_docs)]
pub fn string_member(&self) -> &::std::option::Option<::std::string::String> {
    &self.string_member
}
}
impl OrphanedConfigShape {
    /// Creates a new builder-style object to manufacture [`OrphanedConfigShape`](crate::types::OrphanedConfigShape).
    pub fn builder() -> crate::types::builders::OrphanedConfigShapeBuilder {
        crate::types::builders::OrphanedConfigShapeBuilder::default()
    }
}

/// A builder for [`OrphanedConfigShape`](crate::types::OrphanedConfigShape).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct OrphanedConfigShapeBuilder {
    pub(crate) string_member: ::std::option::Option<::std::string::String>,
}
impl OrphanedConfigShapeBuilder {
    #[allow(missing_docs)]
pub fn string_member(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.string_member = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)]
pub fn set_string_member(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.string_member = input;
    self
}
#[allow(missing_docs)]
pub fn get_string_member(&self) -> &::std::option::Option<::std::string::String> {
    &self.string_member
}
    /// Consumes the builder and constructs a [`OrphanedConfigShape`](crate::types::OrphanedConfigShape).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::types::OrphanedConfigShape,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::types::OrphanedConfigShape {
            string_member: self.string_member,
        })
    }
}
