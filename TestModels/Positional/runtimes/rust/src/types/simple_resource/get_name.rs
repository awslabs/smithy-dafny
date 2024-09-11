// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::types::simple_resource::SimpleResourceRef {
    /// Constructs a fluent builder for the [`GetName`](crate::operation::get_name::builders::GetNameFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    /// - On success, responds with [`GetNameOutput`](crate::operation::get_name::GetNameOutput) with field(s):
    ///   - [`blob_value(Option<::aws_smithy_types::Blob>)`](crate::operation::get_name::GetNameOutput::blob_value): (undocumented)
    /// - On failure, responds with [`SdkError<GetNameError>`](crate::operation::get_name::GetNameError)
    pub fn get_name(&self) -> crate::operation::get_name::builders::GetNameFluentBuilder {
        crate::operation::get_name::builders::GetNameFluentBuilder::new(self.clone())
    }
}