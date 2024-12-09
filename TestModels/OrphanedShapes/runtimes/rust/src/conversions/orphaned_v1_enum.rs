// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: crate::types::OrphanedV1Enum,
) -> ::std::rc::Rc<crate::r#simple::orphaned::internaldafny::types::OrphanedV1Enum>{
    ::std::rc::Rc::new(match value {
        crate::types::OrphanedV1Enum::First => crate::r#simple::orphaned::internaldafny::types::OrphanedV1Enum::FIRST {},
crate::types::OrphanedV1Enum::Second => crate::r#simple::orphaned::internaldafny::types::OrphanedV1Enum::SECOND {},
crate::types::OrphanedV1Enum::Third => crate::r#simple::orphaned::internaldafny::types::OrphanedV1Enum::THIRD {},
        _ => panic!("Unknown enum variant: {}", value),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: &crate::r#simple::orphaned::internaldafny::types::OrphanedV1Enum,
) -> crate::types::OrphanedV1Enum {
    match dafny_value {
        crate::r#simple::orphaned::internaldafny::types::OrphanedV1Enum::FIRST {} => crate::types::OrphanedV1Enum::First,
crate::r#simple::orphaned::internaldafny::types::OrphanedV1Enum::SECOND {} => crate::types::OrphanedV1Enum::Second,
crate::r#simple::orphaned::internaldafny::types::OrphanedV1Enum::THIRD {} => crate::types::OrphanedV1Enum::Third,
    }
}
