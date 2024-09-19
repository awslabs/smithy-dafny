// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::get_union::GetUnionInput,
) -> ::std::rc::Rc<crate::implementation_from_dafny::r#_simple_dunion_dinternaldafny_dtypes::GetUnionInput> {
    let dafny_value = match value.union {
        Some(v) => crate::implementation_from_dafny::_Wrappers_Compile::Option::Some {
            value: {
                let v = match v {
                crate::types::_my_union::MyUnion::IntegerValue(n) => {
                    crate::implementation_from_dafny::_simple_dunion_dinternaldafny_dtypes::MyUnion::IntegerValue { IntegerValue: n }
                }
                crate::types::_my_union::MyUnion::StringValue(s) => crate::implementation_from_dafny::_simple_dunion_dinternaldafny_dtypes::MyUnion::StringValue { StringValue: dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&s) },
                crate::types::_my_union::MyUnion::Unknown => unreachable!(),
                };

                ::std::rc::Rc::new(v)
            },
        },
        None => crate::implementation_from_dafny::_Wrappers_Compile::Option::None {},
    };
    ::std::rc::Rc::new(crate::implementation_from_dafny::r#_simple_dunion_dinternaldafny_dtypes::GetUnionInput::GetUnionInput {
    r#union: ::std::rc::Rc::new(dafny_value)
  })
}

#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::implementation_from_dafny::r#_simple_dunion_dinternaldafny_dtypes::GetUnionInput,
    >,
) -> crate::operation::get_union::GetUnionInput {
    let value = if matches!(
        dafny_value.union().as_ref(),
        crate::implementation_from_dafny::_Wrappers_Compile::Option::Some { .. }
    ) {
        let v = dafny_value.union().Extract();
        let v = ::std::rc::Rc::try_unwrap(v).unwrap_or_else(|rc| (*rc).clone());
        let v = match v {
            crate::implementation_from_dafny::_simple_dunion_dinternaldafny_dtypes::MyUnion::IntegerValue {
                IntegerValue,
            } => crate::types::_my_union::MyUnion::IntegerValue(IntegerValue),
            crate::implementation_from_dafny::_simple_dunion_dinternaldafny_dtypes::MyUnion::StringValue {
                StringValue,
            } => crate::types::_my_union::MyUnion::StringValue(dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(
                &StringValue,
            )),
        };
        Some(v)
    } else if matches!(
        dafny_value.union().as_ref(),
        crate::implementation_from_dafny::_Wrappers_Compile::Option::None { .. }
    ) {
        None
    } else {
        panic!("Unreachable")
    };

    crate::operation::get_union::GetUnionInput { union: value }
}