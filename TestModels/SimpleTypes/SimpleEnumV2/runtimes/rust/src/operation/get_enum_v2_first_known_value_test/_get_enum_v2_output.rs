// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetEnumV2Output {
    #[allow(missing_docs)] // documentation missing in model
pub value: ::std::option::Option<crate::types::SimpleEnumV2Shape>,
}
impl GetEnumV2Output {
    #[allow(missing_docs)] // documentation missing in model
pub fn value(&self) -> ::std::option::Option<crate::types::SimpleEnumV2Shape> {
    self.value.clone()
}
}
impl GetEnumV2Output {
    /// Creates a new builder-style object to manufacture [`GetEnumV2Output`](crate::operation::operation::GetEnumV2Output).
    pub fn builder() -> crate::operation::get_enum_v2_first_known_value_test::builders::GetEnumV2OutputBuilder {
        crate::operation::get_enum_v2_first_known_value_test::builders::GetEnumV2OutputBuilder::default()
    }
}

/// A builder for [`GetEnumV2Output`](crate::operation::operation::GetEnumV2Output).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetEnumV2OutputBuilder {
    pub(crate) value: ::std::option::Option<crate::types::SimpleEnumV2Shape>,
}
impl GetEnumV2OutputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn value(mut self, input: impl ::std::convert::Into<crate::types::SimpleEnumV2Shape>) -> Self {
    self.value = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_value(mut self, input: ::std::option::Option<crate::types::SimpleEnumV2Shape>) -> Self {
    self.value = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_value(&self) -> &::std::option::Option<crate::types::SimpleEnumV2Shape> {
    &self.value
}
    /// Consumes the builder and constructs a [`GetEnumV2Output`](crate::operation::operation::GetEnumV2Output).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_enum_v2_first_known_value_test::GetEnumV2Output,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::get_enum_v2_first_known_value_test::GetEnumV2Output {
            value: self.value,
        })
    }
}
