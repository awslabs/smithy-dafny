impl super::Client {
    /// Constructs a fluent builder for the [`$operationName:L`](crate::operation::$snakeCaseOperationName:L::builders::$operationName:LFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
$builderSettersDoc:L
    /// - On success, responds with [`$operationOutputName:L`](crate::operation::$snakeCaseOperationName:L::$operationOutputName:L) with field(s):
$outputDoc:L
    /// - On failure, responds with [`SdkError<$operationErrorName:L>`](crate::operation::$snakeCaseOperationName:L::$operationErrorName:L)
    pub fn $snakeCaseOperationName:L(&self) -> crate::operation::$snakeCaseOperationName:L::builders::$operationName:LFluentBuilder {
        crate::operation::$snakeCaseOperationName:L::builders::$operationName:LFluentBuilder::new(self.clone())
    }
}