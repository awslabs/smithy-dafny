// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetEnum`](crate::operation::get_enum_third_known_value::builders::GetEnumFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`value(SimpleEnumShape)`](crate::operation::get_enum_third_known_value::builders::GetEnumFluentBuilder::value) / [`set_value(Option<SimpleEnumShape>)`](crate::operation::get_enum_third_known_value::builders::GetEnumFluentBuilder::set_value):(undocumented)<br>
    /// - On success, responds with [`GetEnumOutput`](crate::operation::get_enum_third_known_value::GetEnumOutput) with field(s):
    ///   - [`value(Option<Enum>)`](crate::operation::get_enum_third_known_value::GetEnumOutput::value): (undocumented)
    /// - On failure, responds with [`SdkError<GetEnumError>`](crate::operation::get_enum_third_known_value::GetEnumError)
    pub fn get_enum_third_known_value_test(
        &self,
    ) -> crate::operation::get_enum_third_known_value_test::builders::GetEnumThirdKnownValueTestFluentBuilder
    {
        crate::operation::get_enum_third_known_value_test::builders::GetEnumThirdKnownValueTestFluentBuilder::new(
            self.clone(),
        )
    }
}
