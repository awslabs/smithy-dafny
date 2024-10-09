// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &crate::deps::simple_multiplemodels_dependencyproject::types::SomeDependencyOperationOutput,
) -> ::std::rc::Rc<
    crate::r#simple::multiplemodels::dependencyproject::internaldafny::types::SomeDependencyOperationOutput,
> {
    ::std::rc::Rc::new(to_dafny_plain(value.clone()))
}

#[allow(dead_code)]
pub fn to_dafny_plain(
    value: crate::deps::simple_multiplemodels_dependencyproject::types::SomeDependencyOperationOutput,
) -> crate::r#simple::multiplemodels::dependencyproject::internaldafny::types::SomeDependencyOperationOutput {
    crate::r#simple::multiplemodels::dependencyproject::internaldafny::types::SomeDependencyOperationOutput::SomeDependencyOperationOutput {

    }
}

#[allow(dead_code)]
pub fn option_to_dafny(
  value: ::std::option::Option<crate::deps::simple_multiplemodels_dependencyproject::types::SomeDependencyOperationOutput>,
) -> ::std::rc::Rc<crate::_Wrappers_Compile::Option<::std::rc::Rc<
  crate::r#simple::multiplemodels::dependencyproject::internaldafny::types::SomeDependencyOperationOutput,
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
        crate::r#simple::multiplemodels::dependencyproject::internaldafny::types::SomeDependencyOperationOutput,
    >,
) -> crate::deps::simple_multiplemodels_dependencyproject::types::SomeDependencyOperationOutput {
    plain_from_dafny(&*dafny_value)
}

#[allow(dead_code)]
pub fn plain_from_dafny(
    dafny_value: &crate::r#simple::multiplemodels::dependencyproject::internaldafny::types::SomeDependencyOperationOutput,
) -> crate::deps::simple_multiplemodels_dependencyproject::types::SomeDependencyOperationOutput {
    match dafny_value {
        crate::r#simple::multiplemodels::dependencyproject::internaldafny::types::SomeDependencyOperationOutput::SomeDependencyOperationOutput {..} =>
            crate::deps::simple_multiplemodels_dependencyproject::types::SomeDependencyOperationOutput::builder()

                .build()
                .unwrap()
    }
}

#[allow(dead_code)]
pub fn option_from_dafny(
    dafny_value: ::std::rc::Rc<crate::_Wrappers_Compile::Option<::std::rc::Rc<
        crate::r#simple::multiplemodels::dependencyproject::internaldafny::types::SomeDependencyOperationOutput,
    >>>,
) -> ::std::option::Option<crate::deps::simple_multiplemodels_dependencyproject::types::SomeDependencyOperationOutput> {
    match &*dafny_value {
        crate::_Wrappers_Compile::Option::Some { value } => {
            ::std::option::Option::Some(plain_from_dafny(value))
        }
        _ => ::std::option::Option::None,
    }
}
