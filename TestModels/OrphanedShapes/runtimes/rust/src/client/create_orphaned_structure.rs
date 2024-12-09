// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`CreateOrphanedStructure`](crate::operation::create_orphaned_structure::builders::CreateOrphanedStructureFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:

    /// - On success, responds with [`CreateOrphanedStructureOutput`](crate::operation::create_orphaned_structure::CreateOrphanedStructureOutput) with field(s):

    /// - On failure, responds with [`SdkError<CreateOrphanedStructureError>`](crate::operation::create_orphaned_structure::CreateOrphanedStructureError)
    pub fn create_orphaned_structure(&self) -> crate::operation::create_orphaned_structure::builders::CreateOrphanedStructureFluentBuilder {
        crate::operation::create_orphaned_structure::builders::CreateOrphanedStructureFluentBuilder::new(self.clone())
    }
}
