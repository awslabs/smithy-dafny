// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.

pub trait SimpleResource {
    fn get_name(
        &mut self,
        input: crate::operation::get_name::GetNameInput,
      ) -> Result<
        crate::operation::get_name::GetNameOutput,
        crate::operation::get_name::GetNameError,
      >;
}

#[derive(::std::clone::Clone)]
pub struct SimpleResourceRef {
  pub inner: ::std::rc::Rc<std::cell::RefCell<dyn SimpleResource>>
}

impl ::std::cmp::PartialEq for SimpleResourceRef {
    fn eq(&self, other: &SimpleResourceRef) -> bool {
        ::std::rc::Rc::ptr_eq(&self.inner, &other.inner)
    }
}

impl ::std::fmt::Debug for SimpleResourceRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<SimpleResourceRef>")
    }
}

pub mod get_name;