// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct SimpleOrphanedConfig {
    #[allow(missing_docs)]
pub structure_member: ::std::option::Option<crate::types::OrphanedConfigShape>,
}
impl SimpleOrphanedConfig {
    #[allow(missing_docs)]
pub fn structure_member(&self) -> &::std::option::Option<crate::types::OrphanedConfigShape> {
    &self.structure_member
}
}
impl SimpleOrphanedConfig {
    /// Creates a new builder-style object to manufacture [`SimpleOrphanedConfig`](crate::types::SimpleOrphanedConfig).
    pub fn builder() -> crate::types::simple_orphaned_config::SimpleOrphanedConfigBuilder {
        crate::types::simple_orphaned_config::SimpleOrphanedConfigBuilder::default()
    }
}

/// A builder for [`SimpleOrphanedConfig`](crate::types::SimpleOrphanedConfig).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct SimpleOrphanedConfigBuilder {
    pub(crate) structure_member: ::std::option::Option<crate::types::OrphanedConfigShape>,
}
impl SimpleOrphanedConfigBuilder {
    #[allow(missing_docs)]
pub fn structure_member(mut self, input: impl ::std::convert::Into<crate::types::OrphanedConfigShape>) -> Self {
    self.structure_member = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)]
pub fn set_structure_member(mut self, input: ::std::option::Option<crate::types::OrphanedConfigShape>) -> Self {
    self.structure_member = input;
    self
}
#[allow(missing_docs)]
pub fn get_structure_member(&self) -> &::std::option::Option<crate::types::OrphanedConfigShape> {
    &self.structure_member
}
    /// Consumes the builder and constructs a [`SimpleOrphanedConfig`](crate::types::SimpleOrphanedConfig).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::types::simple_orphaned_config::SimpleOrphanedConfig,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::types::simple_orphaned_config::SimpleOrphanedConfig {
            structure_member: self.structure_member,
        })
    }
}
