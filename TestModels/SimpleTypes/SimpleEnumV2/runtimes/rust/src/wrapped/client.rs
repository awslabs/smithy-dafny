// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
use tokio::runtime::Runtime;

pub struct Client {
    wrapped: crate::client::Client,

    /// A `current_thread` runtime for executing operations on the
    /// asynchronous client in a blocking manner.
    rt: Runtime
}

impl dafny_runtime::UpcastObject<dyn crate::r#simple::types::enumv2::internaldafny::types::ISimpleTypesEnumV2Client> for Client {
  ::dafny_runtime::UpcastObjectFn!(dyn crate::r#simple::types::enumv2::internaldafny::types::ISimpleTypesEnumV2Client);
}

impl dafny_runtime::UpcastObject<dyn std::any::Any> for Client {
    ::dafny_runtime::UpcastObjectFn!(dyn ::std::any::Any);
}

impl Client {
  pub fn from_conf(config: &::std::rc::Rc<
    crate::r#simple::types::enumv2::internaldafny::types::SimpleEnumV2Config,
  >) ->
::std::rc::Rc<crate::r#_Wrappers_Compile::Result<
  ::dafny_runtime::Object<dyn crate::r#simple::types::enumv2::internaldafny::types::ISimpleTypesEnumV2Client>,
  ::std::rc::Rc<crate::r#simple::types::enumv2::internaldafny::types::Error>
>> {
    let rt_result = tokio::runtime::Builder::new_current_thread()
          .enable_all()
          .build();
    let rt = match rt_result {
        Ok(x) => x,
        Err(error) => return crate::conversions::error::to_opaque_error_result(error),
    };
    let result = crate::client::Client::from_conf(
      crate::conversions::simple_enum_v2_config::_simple_enum_v2_config::from_dafny(
          config.clone(),
      ),
    );
    match result {
      Ok(client) =>  {
        let wrap = crate::wrapped::client::Client {
          wrapped: client,
          rt
        };
        std::rc::Rc::new(
          crate::_Wrappers_Compile::Result::Success {
            value: ::dafny_runtime::upcast_object()(::dafny_runtime::object::new(wrap))
          }
        )
      },
      Err(error) => crate::conversions::error::to_opaque_error_result(error)
    }
  }
}

impl crate::r#simple::types::enumv2::internaldafny::types::ISimpleTypesEnumV2Client for Client {
    fn GetEnumV2(
        &mut self,
        input: &std::rc::Rc<
            crate::r#simple::types::enumv2::internaldafny::types::GetEnumV2Input,
        >,
    ) -> std::rc::Rc<
        crate::r#_Wrappers_Compile::Result<
            std::rc::Rc<
                crate::r#simple::types::enumv2::internaldafny::types::GetEnumV2Output,
            >,
            std::rc::Rc<crate::r#simple::types::enumv2::internaldafny::types::Error>,
        >,
    >{
        let inner_input =
            crate::conversions::get_enum_v2::_get_enum_v2_input::from_dafny(input.clone());
        let result = self.rt.block_on(crate::operation::get_enum_v2::GetEnumV2::send(&self.wrapped, inner_input));
        match result {
            Err(error) => ::std::rc::Rc::new(
                crate::_Wrappers_Compile::Result::Failure {
                    error: crate::conversions::get_enum_v2::to_dafny_error(error),
                },
            ),
            Ok(client) => ::std::rc::Rc::new(
                crate::_Wrappers_Compile::Result::Success {
                    value: crate::conversions::get_enum_v2::_get_enum_v2_output::to_dafny(client),
                },
            ),
        }
    }

    fn GetEnumV2FirstKnownValueTest(
        &mut self,
        input: &std::rc::Rc<
            crate::r#simple::types::enumv2::internaldafny::types::GetEnumV2Input,
        >,
    ) -> std::rc::Rc<
        crate::r#_Wrappers_Compile::Result<
            std::rc::Rc<
                crate::r#simple::types::enumv2::internaldafny::types::GetEnumV2Output,
            >,
            std::rc::Rc<crate::r#simple::types::enumv2::internaldafny::types::Error>,
        >,
    >{
        let inner_input =
            crate::conversions::get_enum_v2_first_known_value_test::_get_enum_v2_first_known_value_test_input::from_dafny(input.clone());
        let result = self.rt.block_on(crate::operation::get_enum_v2_first_known_value_test::GetEnumV2FirstKnownValueTest::send(&self.wrapped, inner_input));
        match result {
            Err(error) => ::std::rc::Rc::new(
                crate::_Wrappers_Compile::Result::Failure {
                    error: crate::conversions::get_enum_v2_first_known_value_test::to_dafny_error(error),
                },
            ),
            Ok(client) => ::std::rc::Rc::new(
                crate::_Wrappers_Compile::Result::Success {
                    value: crate::conversions::get_enum_v2_first_known_value_test::_get_enum_v2_first_known_value_test_output::to_dafny(client),
                },
            ),
        }
    }

    fn GetEnumV2SecondKnownValueTest(
        &mut self,
        input: &std::rc::Rc<
            crate::r#simple::types::enumv2::internaldafny::types::GetEnumV2Input,
        >,
    ) -> std::rc::Rc<
        crate::r#_Wrappers_Compile::Result<
            std::rc::Rc<
                crate::r#simple::types::enumv2::internaldafny::types::GetEnumV2Output,
            >,
            std::rc::Rc<crate::r#simple::types::enumv2::internaldafny::types::Error>,
        >,
    >{
        let inner_input =
            crate::conversions::get_enum_v2_second_known_value_test::_get_enum_v2_second_known_value_test_input::from_dafny(input.clone());
        let result = self.rt.block_on(crate::operation::get_enum_v2_second_known_value_test::GetEnumV2SecondKnownValueTest::send(&self.wrapped, inner_input));
        match result {
            Err(error) => ::std::rc::Rc::new(
                crate::_Wrappers_Compile::Result::Failure {
                    error: crate::conversions::get_enum_v2_second_known_value_test::to_dafny_error(error),
                },
            ),
            Ok(client) => ::std::rc::Rc::new(
                crate::_Wrappers_Compile::Result::Success {
                    value: crate::conversions::get_enum_v2_second_known_value_test::_get_enum_v2_second_known_value_test_output::to_dafny(client),
                },
            ),
        }
    }

    fn GetEnumV2ThirdKnownValueTest(
        &mut self,
        input: &std::rc::Rc<
            crate::r#simple::types::enumv2::internaldafny::types::GetEnumV2Input,
        >,
    ) -> std::rc::Rc<
        crate::r#_Wrappers_Compile::Result<
            std::rc::Rc<
                crate::r#simple::types::enumv2::internaldafny::types::GetEnumV2Output,
            >,
            std::rc::Rc<crate::r#simple::types::enumv2::internaldafny::types::Error>,
        >,
    >{
        let inner_input =
            crate::conversions::get_enum_v2_third_known_value_test::_get_enum_v2_third_known_value_test_input::from_dafny(input.clone());
        let result = self.rt.block_on(crate::operation::get_enum_v2_third_known_value_test::GetEnumV2ThirdKnownValueTest::send(&self.wrapped, inner_input));
        match result {
            Err(error) => ::std::rc::Rc::new(
                crate::_Wrappers_Compile::Result::Failure {
                    error: crate::conversions::get_enum_v2_third_known_value_test::to_dafny_error(error),
                },
            ),
            Ok(client) => ::std::rc::Rc::new(
                crate::_Wrappers_Compile::Result::Success {
                    value: crate::conversions::get_enum_v2_third_known_value_test::_get_enum_v2_third_known_value_test_output::to_dafny(client),
                },
            ),
        }
    }
}
