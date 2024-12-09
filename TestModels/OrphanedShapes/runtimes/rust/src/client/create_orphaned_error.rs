// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`CreateOrphanedError`](crate::operation::create_orphaned_error::builders::CreateOrphanedErrorFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:

    /// - On success, responds with [`CreateOrphanedErrorOutput`](crate::operation::create_orphaned_error::CreateOrphanedErrorOutput) with field(s):

    /// - On failure, responds with [`SdkError<CreateOrphanedErrorError>`](crate::operation::create_orphaned_error::CreateOrphanedErrorError)
    pub fn create_orphaned_error(&self) -> crate::operation::create_orphaned_error::builders::CreateOrphanedErrorFluentBuilder {
        crate::operation::create_orphaned_error::builders::CreateOrphanedErrorFluentBuilder::new(self.clone())
    }
}
