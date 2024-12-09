// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::orphaned_resource_operation::OrphanedResourceOperationInput,
) -> ::std::rc::Rc<
    crate::r#simple::orphaned::internaldafny::types::OrphanedResourceOperationInput,
>{
    ::std::rc::Rc::new(crate::r#simple::orphaned::internaldafny::types::OrphanedResourceOperationInput::OrphanedResourceOperationInput {
        someString: crate::standard_library_conversions::ostring_to_dafny(&value.some_string),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#simple::orphaned::internaldafny::types::OrphanedResourceOperationInput,
    >,
) -> crate::operation::orphaned_resource_operation::OrphanedResourceOperationInput {
    crate::operation::orphaned_resource_operation::OrphanedResourceOperationInput::builder()
        .set_some_string(crate::standard_library_conversions::ostring_from_dafny(dafny_value.someString().clone()))
        .build()
        .unwrap()
}
