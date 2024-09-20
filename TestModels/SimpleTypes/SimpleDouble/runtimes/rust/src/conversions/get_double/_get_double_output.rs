// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::get_double::GetDoubleOutput,
) -> ::std::rc::Rc<
    crate::r#simple::types::smithydouble::internaldafny::types::GetDoubleOutput,
>{
    ::std::rc::Rc::new(crate::r#simple::types::smithydouble::internaldafny::types::GetDoubleOutput::GetDoubleOutput {
        value: crate::standard_library_conversions::odouble_to_dafny(&value.value),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#simple::types::smithydouble::internaldafny::types::GetDoubleOutput,
    >,
) -> crate::operation::get_double::GetDoubleOutput {
    crate::operation::get_double::GetDoubleOutput::builder()
        .set_value(crate::standard_library_conversions::odouble_from_dafny(dafny_value.value().clone()))
        .build()
        .unwrap()
}
