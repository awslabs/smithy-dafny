// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct CreateOrphanedResourceInput {

}
impl CreateOrphanedResourceInput {

}
impl CreateOrphanedResourceInput {
    /// Creates a new builder-style object to manufacture [`CreateOrphanedResourceInput`](crate::operation::create_orphaned_resource::builders::CreateOrphanedResourceInput).
    pub fn builder() -> crate::operation::create_orphaned_resource::builders::CreateOrphanedResourceInputBuilder {
        crate::operation::create_orphaned_resource::builders::CreateOrphanedResourceInputBuilder::default()
    }
}

/// A builder for [`CreateOrphanedResourceInput`](crate::operation::operation::CreateOrphanedResourceInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateOrphanedResourceInputBuilder {

}
impl CreateOrphanedResourceInputBuilder {

    /// Consumes the builder and constructs a [`CreateOrphanedResourceInput`](crate::operation::operation::CreateOrphanedResourceInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_orphaned_resource::CreateOrphanedResourceInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::create_orphaned_resource::CreateOrphanedResourceInput {

        })
    }
}
