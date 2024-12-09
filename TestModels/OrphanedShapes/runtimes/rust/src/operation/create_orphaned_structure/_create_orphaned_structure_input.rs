// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct CreateOrphanedStructureInput {

}
impl CreateOrphanedStructureInput {

}
impl CreateOrphanedStructureInput {
    /// Creates a new builder-style object to manufacture [`CreateOrphanedStructureInput`](crate::operation::create_orphaned_structure::builders::CreateOrphanedStructureInput).
    pub fn builder() -> crate::operation::create_orphaned_structure::builders::CreateOrphanedStructureInputBuilder {
        crate::operation::create_orphaned_structure::builders::CreateOrphanedStructureInputBuilder::default()
    }
}

/// A builder for [`CreateOrphanedStructureInput`](crate::operation::operation::CreateOrphanedStructureInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateOrphanedStructureInputBuilder {

}
impl CreateOrphanedStructureInputBuilder {

    /// Consumes the builder and constructs a [`CreateOrphanedStructureInput`](crate::operation::operation::CreateOrphanedStructureInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_orphaned_structure::CreateOrphanedStructureInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::create_orphaned_structure::CreateOrphanedStructureInput {

        })
    }
}
