// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &crate::types::OrphanedStructure,
) -> ::std::rc::Rc<
    crate::r#simple::orphaned::internaldafny::types::OrphanedStructure,
> {
    ::std::rc::Rc::new(to_dafny_plain(value.clone()))
}

#[allow(dead_code)]
pub fn to_dafny_plain(
    value: crate::types::OrphanedStructure,
) -> crate::r#simple::orphaned::internaldafny::types::OrphanedStructure {
    crate::r#simple::orphaned::internaldafny::types::OrphanedStructure::OrphanedStructure {
        blobValue: crate::standard_library_conversions::oblob_to_dafny(&value.blob_value),
 booleanValue: crate::standard_library_conversions::obool_to_dafny(&value.boolean_value),
 stringValue: crate::standard_library_conversions::ostring_to_dafny(&value.string_value),
 integerValue: crate::standard_library_conversions::oint_to_dafny(value.integer_value),
 longValue: crate::standard_library_conversions::olong_to_dafny(&value.long_value),
 unionValue: ::std::rc::Rc::new(match &value.union_value {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::conversions::orphaned_union::to_dafny(&x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 enumValue: ::std::rc::Rc::new(match &value.enum_value {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::conversions::orphaned_v1_enum::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 mapValue:
::std::rc::Rc::new(match &value.map_value {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::hashmap_to_dafny_map(x,
            |k| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&k),
            |v| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&v),
        )
    },
    None => crate::r#_Wrappers_Compile::Option::None {}
})
,
 listValue: ::std::rc::Rc::new(match &value.list_value {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&e),
        )
    },
    None => crate::r#_Wrappers_Compile::Option::None {}
})
,
    }
}

#[allow(dead_code)]
pub fn option_to_dafny(
  value: ::std::option::Option<crate::types::OrphanedStructure>,
) -> ::std::rc::Rc<crate::_Wrappers_Compile::Option<::std::rc::Rc<
  crate::r#simple::orphaned::internaldafny::types::OrphanedStructure,
>>>{
    ::std::rc::Rc::new(match value {
        ::std::option::Option::None => crate::_Wrappers_Compile::Option::None {},
        ::std::option::Option::Some(x) => crate::_Wrappers_Compile::Option::Some {
            value: ::std::rc::Rc::new(to_dafny_plain(x)),
        },
    })
}

#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#simple::orphaned::internaldafny::types::OrphanedStructure,
    >,
) -> crate::types::OrphanedStructure {
    plain_from_dafny(&*dafny_value)
}

#[allow(dead_code)]
pub fn plain_from_dafny(
    dafny_value: &crate::r#simple::orphaned::internaldafny::types::OrphanedStructure,
) -> crate::types::OrphanedStructure {
    match dafny_value {
        crate::r#simple::orphaned::internaldafny::types::OrphanedStructure::OrphanedStructure {..} =>
            crate::types::OrphanedStructure::builder()
                .set_blob_value(crate::standard_library_conversions::oblob_from_dafny(dafny_value.blobValue().clone()))
 .set_boolean_value(crate::standard_library_conversions::obool_from_dafny(dafny_value.booleanValue().clone()))
 .set_string_value(crate::standard_library_conversions::ostring_from_dafny(dafny_value.stringValue().clone()))
 .set_integer_value(crate::standard_library_conversions::oint_from_dafny(dafny_value.integerValue().clone()))
 .set_long_value(crate::standard_library_conversions::olong_from_dafny(dafny_value.longValue().clone()))
 .set_union_value(match (*dafny_value.unionValue()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::conversions::orphaned_union::from_dafny(value.clone())),
    _ => None,
}
)
 .set_enum_value(match &**dafny_value.enumValue() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::conversions::orphaned_v1_enum::from_dafny(value)
    ),
    _ => None,
}
)
 .set_map_value(match (*dafny_value.mapValue()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_map_to_hashmap(value,
                |k: &::dafny_runtime::dafny_runtime_conversions::DafnySequence<::dafny_runtime::dafny_runtime_conversions::DafnyCharUTF16>| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(k),
                |v: &::dafny_runtime::dafny_runtime_conversions::DafnySequence<::dafny_runtime::dafny_runtime_conversions::DafnyCharUTF16>| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(v),
            )
        ),
    _ => None
}
)
 .set_list_value(match (*dafny_value.listValue()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(value,
                |e: &::dafny_runtime::dafny_runtime_conversions::DafnySequence<::dafny_runtime::dafny_runtime_conversions::DafnyCharUTF16>| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(e),
            )
        ),
    _ => None
}
)
                .build()
                .unwrap()
    }
}

#[allow(dead_code)]
pub fn option_from_dafny(
    dafny_value: ::std::rc::Rc<crate::_Wrappers_Compile::Option<::std::rc::Rc<
        crate::r#simple::orphaned::internaldafny::types::OrphanedStructure,
    >>>,
) -> ::std::option::Option<crate::types::OrphanedStructure> {
    match &*dafny_value {
        crate::_Wrappers_Compile::Option::Some { value } => {
            ::std::option::Option::Some(plain_from_dafny(value))
        }
        _ => ::std::option::Option::None,
    }
}
