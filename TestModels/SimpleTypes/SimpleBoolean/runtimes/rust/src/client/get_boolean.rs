// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetBoolean`](crate::operation::get_boolean::builders::GetBooleanFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`value(impl Into<Option<Boolean>>)`](crate::operation::get_boolean::builders::GetBooleanFluentBuilder::name) / [`set_name(Option<Boolean>)`](crate::operation::get_boolean::builders::GetBooleanFluentBuilder::set_name):(undocumented)<br>
    /// - On success, responds with [`GetBooleanOutput`](crate::operation::get_boolean::GetBooleanOutput) with field(s):
    ///   - [`value(Option<Boolean>)`](crate::operation::get_boolean::GetBooleanOutput::value): (undocumented)
    /// - On failure, responds with [`SdkError<GetBooleanError>`](crate::operation::get_boolean::GetBooleanError)
    pub fn get_boolean(&self) -> crate::operation::get_boolean::builders::GetBooleanFluentBuilder {
        crate::operation::get_boolean::builders::GetBooleanFluentBuilder::new(self.handle.clone())
    }
}
