// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::types::orphaned_resource::OrphanedResourceRef {
    /// Constructs a fluent builder for the [`OrphanedResourceOperation`](crate::operation::orphaned_resource_operation::builders::OrphanedResourceOperationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`some_string(impl Into<Option<::std::string::String>>)`](crate::operation::orphaned_resource_operation::builders::OrphanedResourceOperationFluentBuilder::some_string) / [`set_some_string(Option<::std::string::String>)`](crate::operation::orphaned_resource_operation::builders::OrphanedResourceOperationFluentBuilder::set_some_string): (undocumented)<br>
    /// - On success, responds with [`OrphanedResourceOperationOutput`](crate::operation::orphaned_resource_operation::OrphanedResourceOperationOutput) with field(s):
    ///   - [`some_string(Option<::std::string::String>)`](crate::operation::orphaned_resource_operation::OrphanedResourceOperationOutput::some_string): (undocumented)
    /// - On failure, responds with [`SdkError<OrphanedResourceOperationError>`](crate::operation::orphaned_resource_operation::OrphanedResourceOperationError)
    pub fn orphaned_resource_operation(&self) -> crate::operation::orphaned_resource_operation::builders::OrphanedResourceOperationFluentBuilder {
        crate::operation::orphaned_resource_operation::builders::OrphanedResourceOperationFluentBuilder::new(self.clone())
    }
}
