// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::operation::create_orphaned_error::_create_orphaned_error_output::CreateOrphanedErrorOutputBuilder;

pub use crate::operation::create_orphaned_error::_create_orphaned_error_input::CreateOrphanedErrorInputBuilder;

impl CreateOrphanedErrorInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::client::Client,
    ) -> ::std::result::Result<
        crate::operation::create_orphaned_error::CreateOrphanedErrorOutput,
        crate::types::error::Error,
    > {
        let mut fluent_builder = client.create_orphaned_error();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateOrphanedError`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateOrphanedErrorFluentBuilder {
    client: crate::client::Client,
    pub(crate) inner: crate::operation::create_orphaned_error::builders::CreateOrphanedErrorInputBuilder,
}
impl CreateOrphanedErrorFluentBuilder {
    /// Creates a new `CreateOrphanedError`.
    pub(crate) fn new(client: crate::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the CreateOrphanedError as a reference.
    pub fn as_input(&self) -> &crate::operation::create_orphaned_error::builders::CreateOrphanedErrorInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_orphaned_error::CreateOrphanedErrorOutput,
        crate::types::error::Error,
    > {
        let input = self
            .inner
            .build()
            // Using Opaque since we don't have a validation-specific error yet.
            // Operations' models don't declare their own validation error,
            // and smithy-rs seems to not generate a ValidationError case unless there is.
            // Vanilla smithy-rs uses SdkError::construction_failure, but we aren't using SdkError.
            .map_err(|mut e| {
	     let msg = format!("{:?}", e);
             crate::types::error::Error::OpaqueWithText {
                obj: ::dafny_runtime::Object::from_ref(&mut e as &mut dyn ::std::any::Any),
		objMessage: msg
             }})?;
        crate::operation::create_orphaned_error::CreateOrphanedError::send(&self.client, input).await
    }


}