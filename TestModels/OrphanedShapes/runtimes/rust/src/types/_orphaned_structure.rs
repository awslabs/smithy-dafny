// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct OrphanedStructure {
    #[allow(missing_docs)]
pub blob_value: ::std::option::Option<::aws_smithy_types::Blob>,
#[allow(missing_docs)]
pub boolean_value: ::std::option::Option<::std::primitive::bool>,
#[allow(missing_docs)]
pub enum_value: ::std::option::Option<crate::types::OrphanedV1Enum>,
#[allow(missing_docs)]
pub integer_value: ::std::option::Option<::std::primitive::i32>,
#[allow(missing_docs)]
pub list_value: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
#[allow(missing_docs)]
pub long_value: ::std::option::Option<::std::primitive::i64>,
#[allow(missing_docs)]
pub map_value: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
#[allow(missing_docs)]
pub string_value: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)]
pub union_value: ::std::option::Option<crate::types::OrphanedUnion>,
}
impl OrphanedStructure {
    #[allow(missing_docs)]
pub fn blob_value(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
    &self.blob_value
}
#[allow(missing_docs)]
pub fn boolean_value(&self) -> &::std::option::Option<::std::primitive::bool> {
    &self.boolean_value
}
#[allow(missing_docs)]
pub fn enum_value(&self) -> &::std::option::Option<crate::types::OrphanedV1Enum> {
    &self.enum_value
}
#[allow(missing_docs)]
pub fn integer_value(&self) -> &::std::option::Option<::std::primitive::i32> {
    &self.integer_value
}
#[allow(missing_docs)]
pub fn list_value(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
    &self.list_value
}
#[allow(missing_docs)]
pub fn long_value(&self) -> &::std::option::Option<::std::primitive::i64> {
    &self.long_value
}
#[allow(missing_docs)]
pub fn map_value(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
    &self.map_value
}
#[allow(missing_docs)]
pub fn string_value(&self) -> &::std::option::Option<::std::string::String> {
    &self.string_value
}
#[allow(missing_docs)]
pub fn union_value(&self) -> &::std::option::Option<crate::types::OrphanedUnion> {
    &self.union_value
}
}
impl OrphanedStructure {
    /// Creates a new builder-style object to manufacture [`OrphanedStructure`](crate::types::OrphanedStructure).
    pub fn builder() -> crate::types::builders::OrphanedStructureBuilder {
        crate::types::builders::OrphanedStructureBuilder::default()
    }
}

/// A builder for [`OrphanedStructure`](crate::types::OrphanedStructure).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct OrphanedStructureBuilder {
    pub(crate) blob_value: ::std::option::Option<::aws_smithy_types::Blob>,
pub(crate) boolean_value: ::std::option::Option<::std::primitive::bool>,
pub(crate) enum_value: ::std::option::Option<crate::types::OrphanedV1Enum>,
pub(crate) integer_value: ::std::option::Option<::std::primitive::i32>,
pub(crate) list_value: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
pub(crate) long_value: ::std::option::Option<::std::primitive::i64>,
pub(crate) map_value: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
pub(crate) string_value: ::std::option::Option<::std::string::String>,
pub(crate) union_value: ::std::option::Option<crate::types::OrphanedUnion>,
}
impl OrphanedStructureBuilder {
    #[allow(missing_docs)]
pub fn blob_value(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
    self.blob_value = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)]
pub fn set_blob_value(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
    self.blob_value = input;
    self
}
#[allow(missing_docs)]
pub fn get_blob_value(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
    &self.blob_value
}
#[allow(missing_docs)]
pub fn boolean_value(mut self, input: impl ::std::convert::Into<::std::primitive::bool>) -> Self {
    self.boolean_value = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)]
pub fn set_boolean_value(mut self, input: ::std::option::Option<::std::primitive::bool>) -> Self {
    self.boolean_value = input;
    self
}
#[allow(missing_docs)]
pub fn get_boolean_value(&self) -> &::std::option::Option<::std::primitive::bool> {
    &self.boolean_value
}
#[allow(missing_docs)]
pub fn enum_value(mut self, input: impl ::std::convert::Into<crate::types::OrphanedV1Enum>) -> Self {
    self.enum_value = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)]
pub fn set_enum_value(mut self, input: ::std::option::Option<crate::types::OrphanedV1Enum>) -> Self {
    self.enum_value = input;
    self
}
#[allow(missing_docs)]
pub fn get_enum_value(&self) -> &::std::option::Option<crate::types::OrphanedV1Enum> {
    &self.enum_value
}
#[allow(missing_docs)]
pub fn integer_value(mut self, input: impl ::std::convert::Into<::std::primitive::i32>) -> Self {
    self.integer_value = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)]
pub fn set_integer_value(mut self, input: ::std::option::Option<::std::primitive::i32>) -> Self {
    self.integer_value = input;
    self
}
#[allow(missing_docs)]
pub fn get_integer_value(&self) -> &::std::option::Option<::std::primitive::i32> {
    &self.integer_value
}
#[allow(missing_docs)]
pub fn list_value(mut self, input: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>) -> Self {
    self.list_value = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)]
pub fn set_list_value(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
    self.list_value = input;
    self
}
#[allow(missing_docs)]
pub fn get_list_value(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
    &self.list_value
}
#[allow(missing_docs)]
pub fn long_value(mut self, input: impl ::std::convert::Into<::std::primitive::i64>) -> Self {
    self.long_value = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)]
pub fn set_long_value(mut self, input: ::std::option::Option<::std::primitive::i64>) -> Self {
    self.long_value = input;
    self
}
#[allow(missing_docs)]
pub fn get_long_value(&self) -> &::std::option::Option<::std::primitive::i64> {
    &self.long_value
}
#[allow(missing_docs)]
pub fn map_value(mut self, input: impl ::std::convert::Into<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
    self.map_value = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)]
pub fn set_map_value(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
    self.map_value = input;
    self
}
#[allow(missing_docs)]
pub fn get_map_value(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
    &self.map_value
}
#[allow(missing_docs)]
pub fn string_value(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.string_value = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)]
pub fn set_string_value(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.string_value = input;
    self
}
#[allow(missing_docs)]
pub fn get_string_value(&self) -> &::std::option::Option<::std::string::String> {
    &self.string_value
}
#[allow(missing_docs)]
pub fn union_value(mut self, input: impl ::std::convert::Into<crate::types::OrphanedUnion>) -> Self {
    self.union_value = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)]
pub fn set_union_value(mut self, input: ::std::option::Option<crate::types::OrphanedUnion>) -> Self {
    self.union_value = input;
    self
}
#[allow(missing_docs)]
pub fn get_union_value(&self) -> &::std::option::Option<crate::types::OrphanedUnion> {
    &self.union_value
}
    /// Consumes the builder and constructs a [`OrphanedStructure`](crate::types::OrphanedStructure).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::types::OrphanedStructure,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::types::OrphanedStructure {
            blob_value: self.blob_value,
boolean_value: self.boolean_value,
enum_value: self.enum_value,
integer_value: self.integer_value,
list_value: self.list_value,
long_value: self.long_value,
map_value: self.map_value,
string_value: self.string_value,
union_value: self.union_value,
        })
    }
}
