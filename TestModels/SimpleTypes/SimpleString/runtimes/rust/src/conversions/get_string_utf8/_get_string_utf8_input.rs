use core::str;

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::get_string_utf8::GetStringUTF8Input,
) -> ::std::rc::Rc<
    crate::r#simple::types::smithystring::internaldafny::types::GetStringUTF8Input,
> {
    let dafny_value = match value.value {
      Some(s) => crate::_Wrappers_Compile::Option::Some {
        value: dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&s.as_bytes().to_vec(), |b| *b)
      },
      None => crate::_Wrappers_Compile::Option::None {},
    };
    ::std::rc::Rc::new(crate::r#simple::types::smithystring::internaldafny::types::GetStringUTF8Input::GetStringUTF8Input {
    value: ::std::rc::Rc::new(dafny_value)
  })
}

#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#simple::types::smithystring::internaldafny::types::GetStringUTF8Input,
    >,
) -> crate::operation::get_string_utf8::GetStringUTF8Input {
    let value = if matches!(
        dafny_value.value().as_ref(),
        crate::_Wrappers_Compile::Option::Some { .. }
    ) {
        let bytes = dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(
            &dafny_value.value().Extract(), |b| *b);
        Some(String::from_utf8(bytes).unwrap())
    } else if matches!(
        dafny_value.value().as_ref(),
        crate::_Wrappers_Compile::Option::None { .. }
    ) {
        None
    } else {
        panic!("Unreachable")
    };
    crate::operation::get_string_utf8::GetStringUTF8Input { value }
}
