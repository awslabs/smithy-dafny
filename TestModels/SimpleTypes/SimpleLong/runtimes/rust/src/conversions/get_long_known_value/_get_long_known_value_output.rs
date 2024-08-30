// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::get_long_known_value::GetLongKnownValueOutput,
) -> ::std::rc::Rc<
    crate::r#simple::types::smithylong::internaldafny::types::GetLongOutput,
> {
    let dafny_value = match value.value {
        Some(b) => crate::_Wrappers_Compile::Option::Some { value: b },
        None => crate::_Wrappers_Compile::Option::None {},
    };
    ::std::rc::Rc::new(crate::r#simple::types::smithylong::internaldafny::types::GetLongOutput::GetLongOutput {
    value: ::std::rc::Rc::new(dafny_value)
  })
}

#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#simple::types::smithylong::internaldafny::types::GetLongOutput,
    >,
) -> crate::operation::get_long_known_value::GetLongKnownValueOutput {
    let value = if matches!(
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
    crate::operation::get_long_known_value::GetLongKnownValueOutput { value }
}