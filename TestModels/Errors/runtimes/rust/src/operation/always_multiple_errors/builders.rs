// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::operation::always_multiple_errors::_get_errors_output::GetErrorsOutputBuilder;

pub use crate::operation::always_multiple_errors::_get_errors_input::GetErrorsInputBuilder;

impl GetErrorsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::client::Client,
    ) -> ::std::result::Result<
        crate::operation::always_multiple_errors::GetErrorsOutput,
        crate::types::error::Error,
    > {
        let mut fluent_builder = client.always_multiple_errors();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `AlwaysMultipleErrors`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct AlwaysMultipleErrorsFluentBuilder {
    client: crate::client::Client,
    pub(crate) inner: crate::operation::always_multiple_errors::builders::GetErrorsInputBuilder,
}
impl AlwaysMultipleErrorsFluentBuilder {
    /// Creates a new `AlwaysMultipleErrors`.
    pub(crate) fn new(client: crate::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the AlwaysMultipleErrors as a reference.
    pub fn as_input(&self) -> &crate::operation::always_multiple_errors::builders::GetErrorsInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::always_multiple_errors::GetErrorsOutput,
        crate::types::error::Error,
    > {
        let input = self
            .inner
            .build()
            // Using Opaque since we don't have a validation-specific error yet.
            // Operations' models don't declare their own validation error,
            // and smithy-rs seems to not generate a ValidationError case unless there is.
            // Vanilla smithy-rs uses SdkError::construction_failure, but we aren't using SdkError.
            .map_err(|mut e| crate::types::error::Error::Opaque {
                obj: ::dafny_runtime::Object::from_ref(&mut e as &mut dyn ::std::any::Any)
            })?;
        crate::operation::always_multiple_errors::AlwaysMultipleErrors::send(&self.client, input).await
    }

    #[allow(missing_docs)] // documentation missing in model
pub fn value(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.inner = self.inner.value(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_value(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.inner = self.inner.set_value(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_value(&self) -> &::std::option::Option<::std::string::String> {
    self.inner.get_value()
}
}
