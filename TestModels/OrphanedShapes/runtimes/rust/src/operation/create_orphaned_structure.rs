// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `CreateOrphanedStructure`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct CreateOrphanedStructure;
impl CreateOrphanedStructure {
    /// Creates a new `CreateOrphanedStructure`
    pub fn new() -> Self {
        Self
    }

    pub(crate) async fn send(
        client: &crate::client::Client,
        input: crate::operation::create_orphaned_structure::CreateOrphanedStructureInput,
    ) -> ::std::result::Result<
        crate::operation::create_orphaned_structure::CreateOrphanedStructureOutput,
        crate::types::error::Error,
    > {

                let inner_input = crate::conversions::create_orphaned_structure::_create_orphaned_structure_input::to_dafny(input);
        let inner_result =
            ::dafny_runtime::md!(client.dafny_client.clone()).CreateOrphanedStructure(&inner_input);
        if matches!(
            inner_result.as_ref(),
            crate::r#_Wrappers_Compile::Result::Success { .. }
        ) {
            Ok(
                crate::conversions::create_orphaned_structure::_create_orphaned_structure_output::from_dafny(inner_result.value().clone()),
            )
        } else {
            Err(crate::conversions::error::from_dafny(
                inner_result.error().clone(),
            ))
        }
    }
}

pub use crate::operation::create_orphaned_structure::_create_orphaned_structure_output::CreateOrphanedStructureOutput;

pub use crate::operation::create_orphaned_structure::_create_orphaned_structure_input::CreateOrphanedStructureInput;

pub(crate) mod _create_orphaned_structure_output;

pub(crate) mod _create_orphaned_structure_input;

/// Builders
pub mod builders;
