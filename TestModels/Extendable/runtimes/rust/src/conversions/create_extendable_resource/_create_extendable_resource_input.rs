// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::create_extendable_resource::CreateExtendableResourceInput,
) -> ::std::rc::Rc<
    crate::r#simple::extendable::resources::internaldafny::types::CreateExtendableResourceInput,
>{
    ::std::rc::Rc::new(crate::r#simple::extendable::resources::internaldafny::types::CreateExtendableResourceInput::CreateExtendableResourceInput {
        name: dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(value.name())
    })
}

#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#simple::extendable::resources::internaldafny::types::CreateExtendableResourceInput,
    >,
) -> crate::operation::create_extendable_resource::CreateExtendableResourceInput {
    match &*dafny_value {
        crate::r#simple::extendable::resources::internaldafny::types::CreateExtendableResourceInput::CreateExtendableResourceInput {
            name
        } =>
        crate::operation::create_extendable_resource::CreateExtendableResourceInput {
            name: dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(
                &*name
            )
         }
    }
}
