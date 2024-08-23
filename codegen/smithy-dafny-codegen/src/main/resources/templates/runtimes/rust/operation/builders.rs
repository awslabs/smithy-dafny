pub use crate::operation::$snakeCaseOperationName:L::_$snakeCaseOperationOutputName:L::$operationOutputName:LBuilder;

pub use crate::operation::$snakeCaseOperationName:L::_$snakeCaseOperationInputName:L::$operationInputName:LBuilder;

impl $operationInputName:LBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::$snakeCaseOperationName:L::$operationOutputName:L,
        crate::operation::$snakeCaseOperationName:L::$operationErrorName:L,
    > {
        let mut fluent_builder = client.$snakeCaseOperationName:L();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `$operationName:L`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct $operationName:LFluentBuilder {
    client: crate::client::Client,
    pub(crate) inner: crate::operation::$snakeCaseOperationName:L::builders::$operationInputName:LBuilder,
}
impl $operationName:LFluentBuilder {
    /// Creates a new `$operationName:L`.
    pub(crate) fn new(client: crate::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the $operationName:L as a reference.
    pub fn as_input(&self) -> &crate::operation::$snakeCaseOperationName:L::builders::$operationInputName:LBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::$snakeCaseOperationName:L::$operationOutputName:L,
        crate::operation::$snakeCaseOperationName:L::$operationErrorName:L,
    > {
        let input = self
            .inner
            .build()
            // Using unhandled since $operationName:L doesn't declare any validation,
            // and smithy-rs seems to not generate a ValidationError case unless there is
            // (but isn't that a backwards compatibility problem for output structures?)
            // Vanilla smithy-rs uses SdkError::construction_failure,
            // but we aren't using SdkError.
            .map_err(crate::operation::$snakeCaseOperationName:L::$operationErrorName:L::unhandled)?;
        crate::operation::$snakeCaseOperationName:L::$operationName:L::send(&self.client, input).await
    }

    $accessors:L
}