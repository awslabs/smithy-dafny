// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::get_known_value_union::GetKnownValueUnionOutput,
) -> ::std::rc::Rc<::simple_union_dafny::r#_simple_dunion_dinternaldafny_dtypes::GetUnionOutput> {
    let dafny_value = match value.union {
        Some(v) => ::simple_union_dafny::_Wrappers_Compile::Option::Some {
            value: {
                let v = match v {
                crate::types::_known_value_union::KnownValueUnion::Value(n) => {
                    ::simple_union_dafny::_simple_dunion_dinternaldafny_dtypes::MyUnion::IntegerValue { IntegerValue: n }
                }
                crate::types::_known_value_union::KnownValueUnion::Unknown => unreachable!(),
                };

                ::std::rc::Rc::new(v)
            },
        },
        None => ::simple_union_dafny::_Wrappers_Compile::Option::None {},
    };
    ::std::rc::Rc::new(::simple_union_dafny::r#_simple_dunion_dinternaldafny_dtypes::GetUnionOutput::GetUnionOutput {
    r#union: ::std::rc::Rc::new(dafny_value)
  })
}

#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        ::simple_union_dafny::r#_simple_dunion_dinternaldafny_dtypes::GetKnownValueUnionOutput,
    >,
) -> crate::operation::get_known_value_union::GetKnownValueUnionOutput {
    let union = if matches!(
        dafny_value.union().as_ref(),
        ::simple_union_dafny::_Wrappers_Compile::Option::Some { .. }
    ) {
        let v = dafny_value.union().Extract();
        let v = ::std::rc::Rc::try_unwrap(v).unwrap_or_else(|rc| (*rc).clone());
        let v = match v {
            simple_union_dafny::_simple_dunion_dinternaldafny_dtypes::KnownValueUnion::Value {
                Value,
            } => crate::types::_known_value_union::KnownValueUnion::Value(Value),
        };
        Some(v)
    } else if matches!(
        dafny_value.union().as_ref(),
        ::simple_union_dafny::_Wrappers_Compile::Option::None { .. }
    ) {
        None
    } else {
        panic!("Unreachable")
    };

    crate::operation::get_known_value_union::GetKnownValueUnionOutput { union }
}
