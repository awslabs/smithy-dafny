// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::get_resource_positional::GetResourcePositionalInput,
) -> ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16> {
    let dafny_value = match value.value {
        Some(b) => crate::implementation_from_dafny::_Wrappers_Compile::Option::Some { value: b },
        None => crate::implementation_from_dafny::_Wrappers_Compile::Option::None {},
    };

    dafny_value
}

#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
) -> crate::operation::get_resource_positional::GetResourcePositionalInput {
    let name = if matches!(
        dafny_value.value().as_ref(),
        crate::implementation_from_dafny::_Wrappers_Compile::Option::Some { .. }
    ) {
        Some(dafny_value.value().Extract())
    } else if matches!(
        dafny_value.value().as_ref(),
        crate::implementation_from_dafny::_Wrappers_Compile::Option::None { .. }
    ) {
        None
    } else {
        panic!("Unreachable")
    };

    crate::operation::get_resource_positional::GetResourcePositionalInput { name }
}
