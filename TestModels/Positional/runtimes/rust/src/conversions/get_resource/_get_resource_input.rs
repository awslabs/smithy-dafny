// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::get_resource::GetResourceInput,
) -> ::std::rc::Rc<
    crate::implementation_from_dafny::r#_simple_dpositional_dinternaldafny_dtypes::GetResourceInput,
> {
    let name = value.name().unwrap();
    let name =
        dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(name);

    ::std::rc::Rc::new(crate::implementation_from_dafny::r#_simple_dpositional_dinternaldafny_dtypes::GetResourceInput::GetResourceInput {
        name,
    })
}

#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::implementation_from_dafny::r#_simple_dpositional_dinternaldafny_dtypes::GetResourceInput,
    >,
) -> crate::operation::get_resource::GetResourceInput {
    let name =
        dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(
            dafny_value.name(),
        );

    crate::operation::get_resource::GetResourceInput { name: Some(name) }
}
