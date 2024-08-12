use crate::*;

pub fn ostring_to_dafny(
    input: &Option<String>,
) -> ::std::rc::Rc<
    _Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>,
> {
    let dafny_value = match input {
    Some(b) => _Wrappers_Compile::Option::Some { value:
        dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&b)
        },
    None => _Wrappers_Compile::Option::None {},
};
    ::std::rc::Rc::new(dafny_value)
}

pub fn ostring_from_dafny(
    input: ::std::rc::Rc<
        _Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>,
    >,
) -> Option<String> {
    if matches!(input.as_ref(), _Wrappers_Compile::Option::Some { .. }) {
        Some(
            dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(
                &input.Extract(),
            ),
        )
    } else {
        None
    }
}

pub fn obool_to_dafny(input: Option<bool>) -> ::std::rc::Rc<_Wrappers_Compile::Option<bool>> {
    let dafny_value = match input {
        Some(b) => _Wrappers_Compile::Option::Some { value: b },
        None => _Wrappers_Compile::Option::None {},
    };
    ::std::rc::Rc::new(dafny_value)
}

pub fn obool_from_dafny(input: ::std::rc::Rc<_Wrappers_Compile::Option<bool>>) -> Option<bool> {
    if matches!(input.as_ref(), _Wrappers_Compile::Option::Some { .. }) {
        Some(input.Extract())
    } else {
        None
    }
}

pub fn oint_to_dafny(input: Option<i32>) -> ::std::rc::Rc<_Wrappers_Compile::Option<i32>> {
    let dafny_value = match input {
        Some(b) => _Wrappers_Compile::Option::Some { value: b },
        None => _Wrappers_Compile::Option::None {},
    };
    ::std::rc::Rc::new(dafny_value)
}

pub fn oint_from_dafny(input: ::std::rc::Rc<_Wrappers_Compile::Option<i32>>) -> Option<i32> {
    if matches!(input.as_ref(), _Wrappers_Compile::Option::Some { .. }) {
        Some(input.Extract())
    } else {
        None
    }
}

pub fn olong_to_dafny(input: Option<i64>) -> ::std::rc::Rc<_Wrappers_Compile::Option<i64>> {
    let dafny_value = match input {
        Some(b) => _Wrappers_Compile::Option::Some { value: b },
        None => _Wrappers_Compile::Option::None {},
    };
    ::std::rc::Rc::new(dafny_value)
}

pub fn olong_from_dafny(input: ::std::rc::Rc<_Wrappers_Compile::Option<i64>>) -> Option<i64> {
    if matches!(input.as_ref(), _Wrappers_Compile::Option::Some { .. }) {
        Some(input.Extract())
    } else {
        None
    }
}

pub fn blob_to_dafny(
    input: &::aws_smithy_types::Blob,
) -> ::dafny_runtime::Sequence<u8> {
    ::dafny_runtime::Sequence::from_array(&input.clone().into_inner())
}

pub fn oblob_to_dafny(
    input: &Option<::aws_smithy_types::Blob>,
) -> ::std::rc::Rc<_Wrappers_Compile::Option<::dafny_runtime::Sequence<u8>>> {
    let dafny_value = match input {
        Some(b) => _Wrappers_Compile::Option::Some {
            value: blob_to_dafny(&b),
        },
        None => _Wrappers_Compile::Option::None {},
    };
    ::std::rc::Rc::new(dafny_value)
}

pub fn blob_from_dafny(
    input: ::dafny_runtime::Sequence<u8>,
) -> ::aws_smithy_types::Blob {
    
    ::aws_smithy_types::Blob::new(
        ::std::rc::Rc::try_unwrap(input.to_array())
            .unwrap_or_else(|rc| (*rc).clone()),
    )
}

pub fn oblob_from_dafny(
    input: ::std::rc::Rc<_Wrappers_Compile::Option<::dafny_runtime::Sequence<u8>>>,
) -> Option<::aws_smithy_types::Blob> {
    if matches!(input.as_ref(), _Wrappers_Compile::Option::Some { .. }) {
        Some(blob_from_dafny(input.Extract()))
    } else {
        None
    }
}

pub fn option_from_dafny<T: ::dafny_runtime::DafnyType, TR>(
    input: ::std::rc::Rc<_Wrappers_Compile::Option<T>>,
    converter: fn(&T) -> TR,
) -> Option<TR> {
    match &*input {
        _Wrappers_Compile::Option::Some { value } => Some(converter(value)),
        _Wrappers_Compile::Option::None { } => None,
    }
}

pub fn option_to_dafny<T: ::dafny_runtime::DafnyType, TR>(
    input: &Option<TR>,
    converter: fn(&TR) -> T,
) -> ::std::rc::Rc<_Wrappers_Compile::Option<T>> {
    match input {
        Some(value) => ::std::rc::Rc::new(
            _Wrappers_Compile::Option::Some {
                value: converter(&value)
            }
        ),
        None => ::std::rc::Rc::new(
            _Wrappers_Compile::Option::None {}
        ),
    }
}

pub fn result_from_dafny<T: ::dafny_runtime::DafnyType, TR, E: ::dafny_runtime::DafnyType, ER>(
    input: ::std::rc::Rc<_Wrappers_Compile::Result<T, E>>,
    converter_t: fn(&T) -> TR,
    converter_e: fn(&E) -> ER,
) -> Result<TR, ER> {
    match &*input {
        _Wrappers_Compile::Result::Success { value } => Ok(converter_t(value)),
        _Wrappers_Compile::Result::Failure { error } => Err(converter_e(error)),
    }
}

pub fn result_to_dafny<T: ::dafny_runtime::DafnyType, TR, E: ::dafny_runtime::DafnyType, ER>(
    input: &Result<TR, ER>,
    converter_t: fn(&TR) -> T,
    converter_e: fn(&ER) -> E,
) -> ::std::rc::Rc<_Wrappers_Compile::Result<T, E>> {
    match input {
        Ok(value) => ::std::rc::Rc::new(
            _Wrappers_Compile::Result::Success {
                value: converter_t(&value)
            }
        ),
        Err(error) => ::std::rc::Rc::new(
            _Wrappers_Compile::Result::Failure {
                error: converter_e(&error)
            }
        ),
    }
}