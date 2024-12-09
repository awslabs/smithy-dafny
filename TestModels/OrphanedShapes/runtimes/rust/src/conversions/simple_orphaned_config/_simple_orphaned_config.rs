// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: crate::types::simple_orphaned_config::SimpleOrphanedConfig,
) -> ::std::rc::Rc<
    crate::r#simple::orphaned::internaldafny::types::SimpleOrphanedConfig,
> {
    ::std::rc::Rc::new(to_dafny_plain(value))
}

#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#simple::orphaned::internaldafny::types::SimpleOrphanedConfig,
    >,
) -> crate::types::simple_orphaned_config::SimpleOrphanedConfig {
    plain_from_dafny(&*dafny_value)
}


#[allow(dead_code)]
pub fn to_dafny_plain(
    value: crate::types::simple_orphaned_config::SimpleOrphanedConfig,
) -> crate::r#simple::orphaned::internaldafny::types::SimpleOrphanedConfig {
    crate::r#simple::orphaned::internaldafny::types::SimpleOrphanedConfig::SimpleOrphanedConfig {
        structureMember: ::std::rc::Rc::new(match &value.structure_member {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::conversions::orphaned_config_shape::to_dafny(&x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    }
}

#[allow(dead_code)]
pub fn plain_from_dafny(
    dafny_value: &crate::r#simple::orphaned::internaldafny::types::SimpleOrphanedConfig,
) -> crate::types::simple_orphaned_config::SimpleOrphanedConfig {
    match dafny_value {
        crate::r#simple::orphaned::internaldafny::types::SimpleOrphanedConfig::SimpleOrphanedConfig {..} =>
            crate::types::simple_orphaned_config::SimpleOrphanedConfig::builder()
                .set_structure_member(match (*dafny_value.structureMember()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::conversions::orphaned_config_shape::from_dafny(value.clone())),
    _ => None,
}
)
                .build()
                .unwrap()
    }
}
