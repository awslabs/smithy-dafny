// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::operation::orphaned_resource_operation::_orphaned_resource_operation_output::OrphanedResourceOperationOutputBuilder;

pub use crate::operation::orphaned_resource_operation::_orphaned_resource_operation_input::OrphanedResourceOperationInputBuilder;

impl OrphanedResourceOperationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        orphaned_resource: &crate::types::orphaned_resource::OrphanedResourceRef,
    ) -> ::std::result::Result<
        crate::operation::orphaned_resource_operation::OrphanedResourceOperationOutput,
        crate::types::error::Error,
    > {
        let mut fluent_builder = orphaned_resource.orphaned_resource_operation();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `OrphanedResourceOperation`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct OrphanedResourceOperationFluentBuilder {
    orphaned_resource: crate::types::orphaned_resource::OrphanedResourceRef,
    pub(crate) inner: crate::operation::orphaned_resource_operation::builders::OrphanedResourceOperationInputBuilder,
}
impl OrphanedResourceOperationFluentBuilder {
    /// Creates a new `OrphanedResourceOperation`.
    pub(crate) fn new(orphaned_resource: crate::types::orphaned_resource::OrphanedResourceRef) -> Self {
        Self {
            orphaned_resource,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the OrphanedResourceOperation as a reference.
    pub fn as_input(&self) -> &crate::operation::orphaned_resource_operation::builders::OrphanedResourceOperationInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::orphaned_resource_operation::OrphanedResourceOperationOutput,
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
        crate::operation::orphaned_resource_operation::OrphanedResourceOperation::send(&self.orphaned_resource, input).await
    }

    #[allow(missing_docs)]
pub fn some_string(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.inner = self.inner.some_string(input.into());
    self
}
#[allow(missing_docs)]
pub fn set_some_string(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.inner = self.inner.set_some_string(input);
    self
}
#[allow(missing_docs)]
pub fn get_some_string(&self) -> &::std::option::Option<::std::string::String> {
    self.inner.get_some_string()
}
}
