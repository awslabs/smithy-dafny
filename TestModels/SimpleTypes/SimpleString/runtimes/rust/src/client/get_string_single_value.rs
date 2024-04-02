// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
  /// Constructs a fluent builder for the [`GetStringSingleValue`](crate::operation::get_string_single_value::builders::GetStringSingleValueFluentBuilder) operation.
  ///
  /// - The fluent builder is configurable:
  ///   - [`value(impl Into<Option<String>>)`](crate::operation::get_string_single_value::builders::GetStringSingleValueFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::get_string_single_value::builders::GetStringSingleValueFluentBuilder::set_name):(undocumented)<br>
  /// - On success, responds with [`GetStringSingleValueOutput`](crate::operation::get_string_single_value::GetStringSingleValueOutput) with field(s):
  ///   - [`value(Option<String>)`](crate::operation::get_string_single_value::GetStringSingleValueOutput::value): (undocumented)
  /// - On failure, responds with [`SdkError<GetStringSingleValueError>`](crate::operation::get_string_single_value::GetStringSingleValueError)
  pub fn get_string_single_value(&self) -> crate::operation::get_string_single_value::builders::GetStringSingleValueFluentBuilder {
      crate::operation::get_string_single_value::builders::GetStringSingleValueFluentBuilder::new(self.handle.clone())
  }
}
