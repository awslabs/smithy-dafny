// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &crate::types::orphaned_resource::OrphanedResourceRef,
) -> ::dafny_runtime::Object<
  dyn crate::r#simple::orphaned::internaldafny::types::IOrphanedResource,
> {
  let wrap = OrphanedResourceWrapper {
      obj: value.clone(),
  };
  let inner = ::std::rc::Rc::new(::std::cell::UnsafeCell::new(wrap));
  ::dafny_runtime::Object (Some(inner) )
}

pub struct OrphanedResourceWrapper {
  obj: crate::types::orphaned_resource::OrphanedResourceRef,
}

impl ::dafny_runtime::UpcastObject<dyn ::std::any::Any> for OrphanedResourceWrapper {
  ::dafny_runtime::UpcastObjectFn!(dyn ::std::any::Any);
}

#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Object<
      dyn crate::r#simple::orphaned::internaldafny::types::IOrphanedResource,
    >,
) -> crate::types::orphaned_resource::OrphanedResourceRef {
    let wrap = IOrphanedResourceDafnyWrapper {
        obj: dafny_value.clone(),
    };
    crate::types::orphaned_resource::OrphanedResourceRef {
      inner: ::std::rc::Rc::new(::std::cell::RefCell::new(wrap))
    }
}

#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct IOrphanedResourceDafnyWrapper {
  pub(crate) obj: ::dafny_runtime::Object<
      dyn crate::r#simple::orphaned::internaldafny::types::IOrphanedResource,
  >,
}

impl crate::simple::orphaned::internaldafny::types::IOrphanedResource
  for OrphanedResourceWrapper
{
  fn r#_OrphanedResourceOperation_k(
    &self,
    input: &::std::rc::Rc<crate::simple::orphaned::internaldafny::types::OrphanedResourceOperationInput>,
) -> ::std::rc::Rc<
    crate::r#_Wrappers_Compile::Result<
        ::std::rc::Rc<crate::r#simple::orphaned::internaldafny::types::OrphanedResourceOperationOutput>,
        ::std::rc::Rc<crate::r#simple::orphaned::internaldafny::types::Error>,
    >,
>
{
    let inner_input = crate::conversions::orphaned_resource_operation::_orphaned_resource_operation_input::from_dafny(input.clone());
    let inner_result = self.obj.inner.borrow_mut().orphaned_resource_operation(inner_input);
    let result = match inner_result {
        Ok(x) => crate::r#_Wrappers_Compile::Result::Success {
            value: crate::conversions::orphaned_resource_operation::_orphaned_resource_operation_output::to_dafny(x.clone()),
        },
        Err(x) => crate::r#_Wrappers_Compile::Result::Failure {
            error: crate::conversions::error::to_dafny(x),
        },
    };
    ::std::rc::Rc::new(result)
}
}

impl crate::types::orphaned_resource::OrphanedResource for IOrphanedResourceDafnyWrapper
{
  fn orphaned_resource_operation(
  &self,
  input: crate::operation::orphaned_resource_operation::OrphanedResourceOperationInput,
) -> Result<
  crate::operation::orphaned_resource_operation::OrphanedResourceOperationOutput,
  crate::types::error::Error,
> {
  let inner_input = crate::conversions::orphaned_resource_operation::_orphaned_resource_operation_input::to_dafny(input);
  let inner_result = ::dafny_runtime::md!(self.obj.clone()).OrphanedResourceOperation(&inner_input);
  if matches!(
      inner_result.as_ref(),
      crate::r#_Wrappers_Compile::Result::Success { .. }
  ) {
      Ok(
          crate::conversions::orphaned_resource_operation::_orphaned_resource_operation_output::from_dafny(inner_result.value().clone()),
      )
  } else {
      Err(crate::conversions::error::from_dafny(
          inner_result.error().clone(),
      ))
  }
}
}
