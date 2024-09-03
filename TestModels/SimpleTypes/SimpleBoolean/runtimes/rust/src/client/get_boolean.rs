// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl super::Client {
    /// Constructs a fluent builder for the [`GetBoolean`](crate::operation::get_boolean::builders::GetBooleanFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`value(impl Into<Option<::std::primitive::bool>>)`](crate::operation::get_boolean::builders::GetBooleanFluentBuilder::value) / [`set_value(Option<::std::primitive::bool>)`](crate::operation::get_boolean::builders::GetBooleanFluentBuilder::set_value): (undocumented)<br>
    /// - On success, responds with [`GetBooleanOutput`](crate::operation::get_boolean::GetBooleanOutput) with field(s):
    ///   - [`value(Option<::std::primitive::bool>)`](crate::operation::get_boolean::GetBooleanOutput::value): (undocumented)
    /// - On failure, responds with [`SdkError<GetBooleanError>`](crate::operation::get_boolean::GetBooleanError)
    pub fn get_boolean(&self) -> crate::operation::get_boolean::builders::GetBooleanFluentBuilder {
        crate::operation::get_boolean::builders::GetBooleanFluentBuilder::new(self.clone())
    }
}
