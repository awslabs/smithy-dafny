// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetResourceDataOutput {
    #[allow(missing_docs)] // documentation missing in model
pub blob_value: ::std::option::Option<::aws_smithy_types::Blob>,
#[allow(missing_docs)] // documentation missing in model
pub boolean_value: ::std::option::Option<::std::primitive::bool>,
#[allow(missing_docs)] // documentation missing in model
pub integer_value: ::std::option::Option<::std::primitive::i32>,
#[allow(missing_docs)] // documentation missing in model
pub long_value: ::std::option::Option<::std::primitive::i64>,
#[allow(missing_docs)] // documentation missing in model
pub string_value: ::std::option::Option<::std::string::String>,
}
impl GetResourceDataOutput {
    #[allow(missing_docs)] // documentation missing in model
pub fn blob_value(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
    &self.blob_value
}
#[allow(missing_docs)] // documentation missing in model
pub fn boolean_value(&self) -> &::std::option::Option<::std::primitive::bool> {
    &self.boolean_value
}
#[allow(missing_docs)] // documentation missing in model
pub fn integer_value(&self) -> &::std::option::Option<::std::primitive::i32> {
    &self.integer_value
}
#[allow(missing_docs)] // documentation missing in model
pub fn long_value(&self) -> &::std::option::Option<::std::primitive::i64> {
    &self.long_value
}
#[allow(missing_docs)] // documentation missing in model
pub fn string_value(&self) -> &::std::option::Option<::std::string::String> {
    &self.string_value
}
}
impl GetResourceDataOutput {
    /// Creates a new builder-style object to manufacture [`GetResourceDataOutput`](crate::operation::get_resource_data::builders::GetResourceDataOutput).
    pub fn builder() -> crate::operation::get_resource_data::builders::GetResourceDataOutputBuilder {
        crate::operation::get_resource_data::builders::GetResourceDataOutputBuilder::default()
    }
}

/// A builder for [`GetResourceDataOutput`](crate::operation::operation::GetResourceDataOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetResourceDataOutputBuilder {
    pub(crate) blob_value: ::std::option::Option<::aws_smithy_types::Blob>,
pub(crate) boolean_value: ::std::option::Option<::std::primitive::bool>,
pub(crate) integer_value: ::std::option::Option<::std::primitive::i32>,
pub(crate) long_value: ::std::option::Option<::std::primitive::i64>,
pub(crate) string_value: ::std::option::Option<::std::string::String>,
}
impl GetResourceDataOutputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn blob_value(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
    self.blob_value = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_blob_value(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
    self.blob_value = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_blob_value(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
    &self.blob_value
}
#[allow(missing_docs)] // documentation missing in model
pub fn boolean_value(mut self, input: impl ::std::convert::Into<::std::primitive::bool>) -> Self {
    self.boolean_value = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_boolean_value(mut self, input: ::std::option::Option<::std::primitive::bool>) -> Self {
    self.boolean_value = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_boolean_value(&self) -> &::std::option::Option<::std::primitive::bool> {
    &self.boolean_value
}
#[allow(missing_docs)] // documentation missing in model
pub fn integer_value(mut self, input: impl ::std::convert::Into<::std::primitive::i32>) -> Self {
    self.integer_value = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_integer_value(mut self, input: ::std::option::Option<::std::primitive::i32>) -> Self {
    self.integer_value = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_integer_value(&self) -> &::std::option::Option<::std::primitive::i32> {
    &self.integer_value
}
#[allow(missing_docs)] // documentation missing in model
pub fn long_value(mut self, input: impl ::std::convert::Into<::std::primitive::i64>) -> Self {
    self.long_value = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_long_value(mut self, input: ::std::option::Option<::std::primitive::i64>) -> Self {
    self.long_value = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_long_value(&self) -> &::std::option::Option<::std::primitive::i64> {
    &self.long_value
}
#[allow(missing_docs)] // documentation missing in model
pub fn string_value(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.string_value = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_string_value(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.string_value = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_string_value(&self) -> &::std::option::Option<::std::string::String> {
    &self.string_value
}
    /// Consumes the builder and constructs a [`GetResourceDataOutput`](crate::operation::operation::GetResourceDataOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_resource_data::GetResourceDataOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::get_resource_data::GetResourceDataOutput {
            blob_value: self.blob_value,
boolean_value: self.boolean_value,
integer_value: self.integer_value,
long_value: self.long_value,
string_value: self.string_value,
        })
    }
}
