// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_thing::_get_thing_output::GetThingOutputBuilder;

pub use crate::operation::get_thing::_get_thing_input::GetThingInputBuilder;

impl GetThingInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_thing::GetThingOutput,
        crate::operation::get_thing::GetThingError,
    > {
        let mut fluent_builder = client.get_thing();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetThing`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetThingFluentBuilder {
    client: crate::client::Client,
    inner: crate::operation::get_thing::builders::GetThingInputBuilder,
}
impl GetThingFluentBuilder {
    /// Creates a new `GetThing`.
    pub(crate) fn new(client: crate::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the GetThing as a reference.
    pub fn as_input(&self) -> &crate::operation::get_thing::builders::GetThingInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_thing::GetThingOutput,
        crate::operation::get_thing::GetThingError,
    > {
        let input = self
            .inner
            .build()
            // Using unhandled since GetThing doesn't declare any validation,
            // and smithy-rs seems to not generate a ValidationError case unless there is
            // (but isn't that a backwards compatibility problem for output structures?)
            // Vanilla smithy-rs uses SdkError::construction_failure,
            // but we aren't using SdkError.
            .map_err(crate::operation::get_thing::GetThingError::unhandled)?;
        crate::operation::get_thing::GetThing::send(&self.client, input).await
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn value(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_value(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_value(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
}