// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::orphaned_resource_operation::OrphanedResourceOperationOutput,
) -> ::std::rc::Rc<
    crate::r#simple::orphaned::internaldafny::types::OrphanedResourceOperationOutput,
>{
    ::std::rc::Rc::new(crate::r#simple::orphaned::internaldafny::types::OrphanedResourceOperationOutput::OrphanedResourceOperationOutput {
        someString: crate::standard_library_conversions::ostring_to_dafny(&value.some_string),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#simple::orphaned::internaldafny::types::OrphanedResourceOperationOutput,
    >,
) -> crate::operation::orphaned_resource_operation::OrphanedResourceOperationOutput {
    crate::operation::orphaned_resource_operation::OrphanedResourceOperationOutput::builder()
        .set_some_string(crate::standard_library_conversions::ostring_from_dafny(dafny_value.someString().clone()))
        .build()
        .unwrap()
}
