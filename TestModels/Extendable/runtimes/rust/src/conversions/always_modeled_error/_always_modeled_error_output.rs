// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::always_modeled_error::AlwaysModeledErrorOutput,
) -> ::std::rc::Rc<
    crate::r#simple::extendable::resources::internaldafny::types::GetExtendableResourceErrorsOutput,
>{
    ::std::rc::Rc::new(crate::r#simple::extendable::resources::internaldafny::types::GetExtendableResourceErrorsOutput::GetExtendableResourceErrorsOutput {
        value: crate::standard_library_conversions::ostring_to_dafny(value.value()),
  })
}
// _always_modeled_error_output
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#simple::extendable::resources::internaldafny::types::GetExtendableResourceErrorsOutput,
    >,
) -> crate::operation::always_modeled_error::AlwaysModeledErrorOutput {
    match &*dafny_value {
        crate::r#simple::extendable::resources::internaldafny::types::GetExtendableResourceErrorsOutput::GetExtendableResourceErrorsOutput {
            value,
        } =>
        crate::operation::always_modeled_error::AlwaysModeledErrorOutput {
            value: crate::standard_library_conversions::ostring_from_dafny(value.clone()),
        }
    }
}
