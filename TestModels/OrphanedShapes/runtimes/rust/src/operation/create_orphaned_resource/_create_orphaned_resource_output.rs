// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct CreateOrphanedResourceOutput {

}
impl CreateOrphanedResourceOutput {

}
impl CreateOrphanedResourceOutput {
    /// Creates a new builder-style object to manufacture [`CreateOrphanedResourceOutput`](crate::operation::create_orphaned_resource::builders::CreateOrphanedResourceOutput).
    pub fn builder() -> crate::operation::create_orphaned_resource::builders::CreateOrphanedResourceOutputBuilder {
        crate::operation::create_orphaned_resource::builders::CreateOrphanedResourceOutputBuilder::default()
    }
}

/// A builder for [`CreateOrphanedResourceOutput`](crate::operation::operation::CreateOrphanedResourceOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateOrphanedResourceOutputBuilder {

}
impl CreateOrphanedResourceOutputBuilder {

    /// Consumes the builder and constructs a [`CreateOrphanedResourceOutput`](crate::operation::operation::CreateOrphanedResourceOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_orphaned_resource::CreateOrphanedResourceOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::create_orphaned_resource::CreateOrphanedResourceOutput {

        })
    }
}
