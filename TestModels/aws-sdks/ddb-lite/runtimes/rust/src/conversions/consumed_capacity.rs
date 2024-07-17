// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::ConsumedCapacity,
) -> ::std::rc::Rc<crate::implementation_from_dafny::r#_software_damazon_dcryptography_dservices_ddynamodb_dinternaldafny_dtypes::ConsumedCapacity>{
  ::std::rc::Rc::new(
    crate::implementation_from_dafny::r#_software_damazon_dcryptography_dservices_ddynamodb_dinternaldafny_dtypes::ConsumedCapacity::ConsumedCapacity {
        TableName: dafny_standard_library::conversion::ostring_to_dafny(&value.table_name),
 CapacityUnits: todo!(),
 ReadCapacityUnits: todo!(),
 WriteCapacityUnits: todo!(),
 Table: ::std::rc::Rc::new(match &value.table {
    Some(x) => crate::implementation_from_dafny::_Wrappers_Compile::Option::Some { value: crate::conversions::capacity::to_dafny(&x) },
    None => crate::implementation_from_dafny::_Wrappers_Compile::Option::None { }
})
,
 LocalSecondaryIndexes:
::std::rc::Rc::new(match &value.local_secondary_indexes {
    Some(x) => crate::implementation_from_dafny::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::hashmap_to_dafny_map(x,
            |k| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(k),
            |v| crate::conversions::capacity::to_dafny(&v)
,
        )
    },
    None => crate::implementation_from_dafny::r#_Wrappers_Compile::Option::None {}
})
,
 GlobalSecondaryIndexes:
::std::rc::Rc::new(match &value.global_secondary_indexes {
    Some(x) => crate::implementation_from_dafny::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::hashmap_to_dafny_map(x,
            |k| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(k),
            |v| crate::conversions::capacity::to_dafny(&v)
,
        )
    },
    None => crate::implementation_from_dafny::r#_Wrappers_Compile::Option::None {}
})
,
    }
  )
} #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::implementation_from_dafny::r#_software_damazon_dcryptography_dservices_ddynamodb_dinternaldafny_dtypes::ConsumedCapacity,
    >,
) -> aws_sdk_dynamodb::types::ConsumedCapacity {
    aws_sdk_dynamodb::types::ConsumedCapacity::builder()
          .set_table_name(dafny_standard_library::conversion::ostring_from_dafny(dafny_value.TableName().clone()))
 .set_capacity_units(todo!())
 .set_read_capacity_units(todo!())
 .set_write_capacity_units(todo!())
 .set_table(match (*dafny_value.Table()).as_ref() {
    crate::implementation_from_dafny::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::conversions::capacity::from_dafny(value.clone())),
    _ => None,
}
)
 .set_local_secondary_indexes(match (*dafny_value.LocalSecondaryIndexes()).as_ref() {
    crate::implementation_from_dafny::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_map_to_hashmap(value,
                |k| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(k),
                |v| crate::conversions::capacity::from_dafny(v.clone())
,
            )
        ),
    _ => None
}
)
 .set_global_secondary_indexes(match (*dafny_value.GlobalSecondaryIndexes()).as_ref() {
    crate::implementation_from_dafny::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_map_to_hashmap(value,
                |k| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(k),
                |v| crate::conversions::capacity::from_dafny(v.clone())
,
            )
        ),
    _ => None
}
)
          .build()

}
