// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::get_resource::GetResourceOutput,
) -> ::std::rc::Rc<
    crate::simple::positional::internaldafny::types::GetResourceOutput,
> {
    let dafny_value = match value.value {
        Some(b) => crate::_Wrappers_Compile::Option::Some { value: b },
        None => crate::_Wrappers_Compile::Option::None {},
    };
    ::std::rc::Rc::new(crate::simple::positional::internaldafny::types::GetResourceOutput::GetResourceOutput {
            output: ::std::rc::Rc::new(dafny_value)
        })
}

#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::simple::positional::internaldafny::types::GetResourceOutput,
        >,
) -> crate::operation::get_resource::GetResourceOutput {
    let output = if matches!(
        dafny_value.value().as_ref(),
        crate::_Wrappers_Compile::Option::Some { .. }
    ) {
        Some(dafny_value.value().Extract())
    } else if matches!(
        dafny_value.value().as_ref(),
        crate::_Wrappers_Compile::Option::None { .. }
    ) {
        None
    } else {
        panic!("Unreachable")
    };
    crate::operation::get_resource::GetResourceOutput { output }
}
