// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct CreateOrphanedErrorInput {

}
impl CreateOrphanedErrorInput {

}
impl CreateOrphanedErrorInput {
    /// Creates a new builder-style object to manufacture [`CreateOrphanedErrorInput`](crate::operation::create_orphaned_error::builders::CreateOrphanedErrorInput).
    pub fn builder() -> crate::operation::create_orphaned_error::builders::CreateOrphanedErrorInputBuilder {
        crate::operation::create_orphaned_error::builders::CreateOrphanedErrorInputBuilder::default()
    }
}

/// A builder for [`CreateOrphanedErrorInput`](crate::operation::operation::CreateOrphanedErrorInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateOrphanedErrorInputBuilder {

}
impl CreateOrphanedErrorInputBuilder {

    /// Consumes the builder and constructs a [`CreateOrphanedErrorInput`](crate::operation::operation::CreateOrphanedErrorInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_orphaned_error::CreateOrphanedErrorInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::create_orphaned_error::CreateOrphanedErrorInput {

        })
    }
}
