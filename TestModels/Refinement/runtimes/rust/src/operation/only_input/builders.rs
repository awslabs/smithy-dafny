// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::only_input::_only_input_output::OnlyInputOutputBuilder;

pub use crate::operation::only_input::_only_input_input::OnlyInputInputBuilder;

impl crate::operation::only_input::builders::OnlyInputInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::only_input::OnlyInputOutput,
        crate::operation::only_input::OnlyInputError,
    > {
        let mut fluent_builder = client.only_input();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `OnlyInput`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct OnlyInputFluentBuilder {
    client: crate::client::Client,
    inner: crate::operation::only_input::builders::OnlyInputInputBuilder,
}
impl OnlyInputFluentBuilder {
    /// Creates a new `OnlyInputFluentBuilder`.
    pub(crate) fn new(client: crate::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the OnlyInput as a reference.
    pub fn as_input(&self) -> &crate::operation::only_input::builders::OnlyInputInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::only_input::OnlyInputOutput,
        crate::operation::only_input::OnlyInputError,
    > {
        let input = self
            .inner
            .build()
            // Using unhandled since OnlyInput doesn't declare any validation,
            // and smithy-rs seems to not generate a ValidationError case unless there is
            // (but isn't that a backwards compatibility problem for output structures?)
            // Vanilla smithy-rs uses SdkError::construction_failure,
            // but we aren't using SdkError.
            .map_err(crate::operation::only_input::OnlyInputError::unhandled)?;
        crate::operation::only_input::OnlyInput::send(&self.client, input).await
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
