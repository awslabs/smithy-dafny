// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::create_orphaned_structure::CreateOrphanedStructureOutput,
) -> ::std::rc::Rc<
    crate::r#simple::orphaned::internaldafny::types::CreateOrphanedStructureOutput,
>{
    ::std::rc::Rc::new(crate::r#simple::orphaned::internaldafny::types::CreateOrphanedStructureOutput::CreateOrphanedStructureOutput {

    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#simple::orphaned::internaldafny::types::CreateOrphanedStructureOutput,
    >,
) -> crate::operation::create_orphaned_structure::CreateOrphanedStructureOutput {
    crate::operation::create_orphaned_structure::CreateOrphanedStructureOutput::builder()

        .build()
        .unwrap()
}
