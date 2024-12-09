// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.

#[allow(missing_docs)]
pub trait OrphanedResource {
    fn orphaned_resource_operation(
    &self,
    input: crate::operation::orphaned_resource_operation::OrphanedResourceOperationInput,
  ) -> Result<
    crate::operation::orphaned_resource_operation::OrphanedResourceOperationOutput,
    crate::types::error::Error,
  >;
}

#[derive(::std::clone::Clone)]
/// A reference to a OrphanedResource
pub struct OrphanedResourceRef {
  pub inner: ::std::rc::Rc<std::cell::RefCell<dyn OrphanedResource>>
}

impl<T : OrphanedResource + 'static> From<T> for OrphanedResourceRef {
    fn from(value: T) -> Self {
        Self { inner: std::rc::Rc::new(std::cell::RefCell::new(value)) }
    }
}

impl ::std::cmp::PartialEq for OrphanedResourceRef {
    fn eq(&self, other: &OrphanedResourceRef) -> bool {
        ::std::rc::Rc::ptr_eq(&self.inner, &other.inner)
    }
}

impl ::std::fmt::Debug for OrphanedResourceRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<OrphanedResourceRef>")
    }
}

mod orphaned_resource_operation;
