// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &crate::types::OrphanedUnion,
) -> ::std::rc::Rc<
    crate::r#simple::orphaned::internaldafny::types::OrphanedUnion,
> {
    ::std::rc::Rc::new(match value {
        crate::types::OrphanedUnion::IntegerValue(x) =>
    crate::r#simple::orphaned::internaldafny::types::OrphanedUnion::integerValue {
        integerValue: x.clone(),
    },
crate::types::OrphanedUnion::StringValue(x) =>
    crate::r#simple::orphaned::internaldafny::types::OrphanedUnion::stringValue {
        stringValue: dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&x),
    },
        _ => panic!("Unknown union variant: {:?}", value),
    })
}

#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#simple::orphaned::internaldafny::types::OrphanedUnion,
    >,
) -> crate::types::OrphanedUnion {
    match &::std::rc::Rc::unwrap_or_clone(dafny_value) {
        crate::r#simple::orphaned::internaldafny::types::OrphanedUnion::integerValue {
    integerValue: x @ _,
} => crate::types::OrphanedUnion::IntegerValue(x .clone()),
crate::r#simple::orphaned::internaldafny::types::OrphanedUnion::stringValue {
    stringValue: x @ _,
} => crate::types::OrphanedUnion::StringValue(dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(x)),
    }
}
