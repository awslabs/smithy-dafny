// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `OrphanedResourceOperation`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct OrphanedResourceOperation;
impl OrphanedResourceOperation {
    /// Creates a new `OrphanedResourceOperation`
    pub fn new() -> Self {
        Self
    }

    pub(crate) async fn send(
        orphaned_resource: &crate::types::orphaned_resource::OrphanedResourceRef,
        input: crate::operation::orphaned_resource_operation::OrphanedResourceOperationInput,
    ) -> ::std::result::Result<
        crate::operation::orphaned_resource_operation::OrphanedResourceOperationOutput,
        crate::types::error::Error,
    > {

        orphaned_resource.inner.borrow_mut().orphaned_resource_operation(input)
    }
}

pub use crate::operation::orphaned_resource_operation::_orphaned_resource_operation_output::OrphanedResourceOperationOutput;

pub use crate::operation::orphaned_resource_operation::_orphaned_resource_operation_input::OrphanedResourceOperationInput;

pub(crate) mod _orphaned_resource_operation_output;

pub(crate) mod _orphaned_resource_operation_input;

/// Builders
pub mod builders;
