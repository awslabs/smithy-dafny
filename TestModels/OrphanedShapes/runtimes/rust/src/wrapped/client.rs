// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
use std::sync::LazyLock;

pub struct Client {
    wrapped: crate::client::Client
}

/// A runtime for executing operations on the asynchronous client in a blocking manner.
/// Necessary because Dafny only generates synchronous code.
static dafny_tokio_runtime: LazyLock<tokio::runtime::Runtime> = LazyLock::new(|| {
    tokio::runtime::Builder::new_multi_thread()
          .enable_all()
          .build()
          .unwrap()
});

impl dafny_runtime::UpcastObject<dyn crate::r#simple::orphaned::internaldafny::types::ISimpleOrphanedClient> for Client {
  ::dafny_runtime::UpcastObjectFn!(dyn crate::r#simple::orphaned::internaldafny::types::ISimpleOrphanedClient);
}

impl dafny_runtime::UpcastObject<dyn std::any::Any> for Client {
    ::dafny_runtime::UpcastObjectFn!(dyn ::std::any::Any);
}

impl Client {
  pub fn from_conf(config: &::std::rc::Rc<
    crate::r#simple::orphaned::internaldafny::types::SimpleOrphanedConfig,
  >) ->
::std::rc::Rc<crate::r#_Wrappers_Compile::Result<
  ::dafny_runtime::Object<dyn crate::r#simple::orphaned::internaldafny::types::ISimpleOrphanedClient>,
  ::std::rc::Rc<crate::r#simple::orphaned::internaldafny::types::Error>
>> {
    let result = crate::client::Client::from_conf(
      crate::conversions::simple_orphaned_config::_simple_orphaned_config::from_dafny(
          config.clone(),
      ),
    );
    match result {
      Ok(client) =>  {
        let wrap = crate::wrapped::client::Client {
          wrapped: client
        };
        std::rc::Rc::new(
          crate::_Wrappers_Compile::Result::Success {
            value: ::dafny_runtime::upcast_object()(::dafny_runtime::object::new(wrap))
          }
        )
      },
	Err(error) => {
          let msg = format!("{:?}", error);
	  crate::conversions::error::to_opaque_error_result(msg)
	}
    }
  }
}

impl crate::r#simple::orphaned::internaldafny::types::ISimpleOrphanedClient for Client {
    fn CreateOrphanedError(
        &self,
        input: &::std::rc::Rc<crate::r#simple::orphaned::internaldafny::types::CreateOrphanedErrorInput>,
    ) -> std::rc::Rc<
        crate::r#_Wrappers_Compile::Result<
            ::std::rc::Rc<crate::r#simple::orphaned::internaldafny::types::CreateOrphanedErrorOutput>,
            std::rc::Rc<crate::r#simple::orphaned::internaldafny::types::Error>,
        >,
    >{
        let inner_input = crate::conversions::create_orphaned_error::_create_orphaned_error_input::from_dafny(input.clone());
        let result = tokio::task::block_in_place(|| {
            dafny_tokio_runtime.block_on(crate::operation::create_orphaned_error::CreateOrphanedError::send(&self.wrapped, inner_input))
        });
        match result {
            Err(error) => ::std::rc::Rc::new(
                crate::_Wrappers_Compile::Result::Failure {
                    error: crate::conversions::error::to_dafny(error),
                },
            ),
            Ok(inner_result) => ::std::rc::Rc::new(
                crate::_Wrappers_Compile::Result::Success {
                    value: crate::conversions::create_orphaned_error::_create_orphaned_error_output::to_dafny(inner_result),
                },
            ),
        }
    }

    fn CreateOrphanedStructure(
        &self,
        input: &::std::rc::Rc<crate::r#simple::orphaned::internaldafny::types::CreateOrphanedStructureInput>,
    ) -> std::rc::Rc<
        crate::r#_Wrappers_Compile::Result<
            ::std::rc::Rc<crate::r#simple::orphaned::internaldafny::types::CreateOrphanedStructureOutput>,
            std::rc::Rc<crate::r#simple::orphaned::internaldafny::types::Error>,
        >,
    >{
        let inner_input = crate::conversions::create_orphaned_structure::_create_orphaned_structure_input::from_dafny(input.clone());
        let result = tokio::task::block_in_place(|| {
            dafny_tokio_runtime.block_on(crate::operation::create_orphaned_structure::CreateOrphanedStructure::send(&self.wrapped, inner_input))
        });
        match result {
            Err(error) => ::std::rc::Rc::new(
                crate::_Wrappers_Compile::Result::Failure {
                    error: crate::conversions::error::to_dafny(error),
                },
            ),
            Ok(inner_result) => ::std::rc::Rc::new(
                crate::_Wrappers_Compile::Result::Success {
                    value: crate::conversions::create_orphaned_structure::_create_orphaned_structure_output::to_dafny(inner_result),
                },
            ),
        }
    }

    fn CreateOrphanedResource(
        &self,
        input: &::std::rc::Rc<crate::r#simple::orphaned::internaldafny::types::CreateOrphanedResourceInput>,
    ) -> std::rc::Rc<
        crate::r#_Wrappers_Compile::Result<
            ::std::rc::Rc<crate::r#simple::orphaned::internaldafny::types::CreateOrphanedResourceOutput>,
            std::rc::Rc<crate::r#simple::orphaned::internaldafny::types::Error>,
        >,
    >{
        let inner_input = crate::conversions::create_orphaned_resource::_create_orphaned_resource_input::from_dafny(input.clone());
        let result = tokio::task::block_in_place(|| {
            dafny_tokio_runtime.block_on(crate::operation::create_orphaned_resource::CreateOrphanedResource::send(&self.wrapped, inner_input))
        });
        match result {
            Err(error) => ::std::rc::Rc::new(
                crate::_Wrappers_Compile::Result::Failure {
                    error: crate::conversions::error::to_dafny(error),
                },
            ),
            Ok(inner_result) => ::std::rc::Rc::new(
                crate::_Wrappers_Compile::Result::Success {
                    value: crate::conversions::create_orphaned_resource::_create_orphaned_resource_output::to_dafny(inner_result),
                },
            ),
        }
    }
}
