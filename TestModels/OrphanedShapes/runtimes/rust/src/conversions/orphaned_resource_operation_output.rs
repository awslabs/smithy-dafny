// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &crate::types::OrphanedResourceOperationOutput,
) -> ::std::rc::Rc<
    crate::r#simple::orphaned::internaldafny::types::OrphanedResourceOperationOutput,
> {
    ::std::rc::Rc::new(to_dafny_plain(value.clone()))
}

#[allow(dead_code)]
pub fn to_dafny_plain(
    value: crate::types::OrphanedResourceOperationOutput,
) -> crate::r#simple::orphaned::internaldafny::types::OrphanedResourceOperationOutput {
    crate::r#simple::orphaned::internaldafny::types::OrphanedResourceOperationOutput::OrphanedResourceOperationOutput {
        someString: crate::standard_library_conversions::ostring_to_dafny(&value.some_string),
    }
}

#[allow(dead_code)]
pub fn option_to_dafny(
  value: ::std::option::Option<crate::types::OrphanedResourceOperationOutput>,
) -> ::std::rc::Rc<crate::_Wrappers_Compile::Option<::std::rc::Rc<
  crate::r#simple::orphaned::internaldafny::types::OrphanedResourceOperationOutput,
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
        crate::r#simple::orphaned::internaldafny::types::OrphanedResourceOperationOutput,
    >,
) -> crate::types::OrphanedResourceOperationOutput {
    plain_from_dafny(&*dafny_value)
}

#[allow(dead_code)]
pub fn plain_from_dafny(
    dafny_value: &crate::r#simple::orphaned::internaldafny::types::OrphanedResourceOperationOutput,
) -> crate::types::OrphanedResourceOperationOutput {
    match dafny_value {
        crate::r#simple::orphaned::internaldafny::types::OrphanedResourceOperationOutput::OrphanedResourceOperationOutput {..} =>
            crate::types::OrphanedResourceOperationOutput::builder()
                .set_some_string(crate::standard_library_conversions::ostring_from_dafny(dafny_value.someString().clone()))
                .build()
                .unwrap()
    }
}

#[allow(dead_code)]
pub fn option_from_dafny(
    dafny_value: ::std::rc::Rc<crate::_Wrappers_Compile::Option<::std::rc::Rc<
        crate::r#simple::orphaned::internaldafny::types::OrphanedResourceOperationOutput,
    >>>,
) -> ::std::option::Option<crate::types::OrphanedResourceOperationOutput> {
    match &*dafny_value {
        crate::_Wrappers_Compile::Option::Some { value } => {
            ::std::option::Option::Some(plain_from_dafny(value))
        }
        _ => ::std::option::Option::None,
    }
}