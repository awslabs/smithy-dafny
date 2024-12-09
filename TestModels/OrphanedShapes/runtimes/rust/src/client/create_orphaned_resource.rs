// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`CreateOrphanedResource`](crate::operation::create_orphaned_resource::builders::CreateOrphanedResourceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:

    /// - On success, responds with [`CreateOrphanedResourceOutput`](crate::operation::create_orphaned_resource::CreateOrphanedResourceOutput) with field(s):

    /// - On failure, responds with [`SdkError<CreateOrphanedResourceError>`](crate::operation::create_orphaned_resource::CreateOrphanedResourceError)
    pub fn create_orphaned_resource(&self) -> crate::operation::create_orphaned_resource::builders::CreateOrphanedResourceFluentBuilder {
        crate::operation::create_orphaned_resource::builders::CreateOrphanedResourceFluentBuilder::new(self.clone())
    }
}
