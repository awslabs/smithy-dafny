use tokio::runtime::Runtime;

pub struct Client {
    wrapped: crate::client::Client,

    /// A `current_thread` runtime for executing operations on the
    /// asynchronous client in a blocking manner.
    rt: Runtime,
}

impl dafny_runtime::UpcastObject<dyn crate::implementation_from_dafny::r#_simple_ddocumentation_dinternaldafny_dtypes::ISimpleDocumentationClient> for Client {
  ::dafny_runtime::UpcastObjectFn!(dyn crate::implementation_from_dafny::r#_simple_ddocumentation_dinternaldafny_dtypes::ISimpleDocumentationClient);
}

impl dafny_runtime::UpcastObject<dyn std::any::Any> for Client {
    ::dafny_runtime::UpcastObjectFn!(dyn ::std::any::Any);
}

impl Client {
    pub fn from_conf(config: &::std::rc::Rc<
    crate::implementation_from_dafny::r#_simple_ddocumentation_dinternaldafny_dtypes::SimpleDocumentationConfig,
  >) ->
::std::rc::Rc<crate::implementation_from_dafny::r#_Wrappers_Compile::Result<
  ::dafny_runtime::Object<dyn crate::implementation_from_dafny::r#_simple_ddocumentation_dinternaldafny_dtypes::ISimpleDocumentationClient>,
  ::std::rc::Rc<crate::implementation_from_dafny::r#_simple_ddocumentation_dinternaldafny_dtypes::Error>
    >>{
        let rt_result = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build();
        let rt = match rt_result {
            Ok(x) => x,
            Err(error) => return crate::conversions::error::to_opaque_error_result(error),
        };
        let result = crate::client::Client::from_conf(
            crate::conversions::simple_documentation_config::_simple_documentation_config::from_dafny(
                config.clone(),
            ),
        );
        match result {
            Ok(client) => {
                let wrap = crate::wrapped::client::Client {
                    wrapped: client,
                    rt,
                };
                std::rc::Rc::new(
                    crate::implementation_from_dafny::_Wrappers_Compile::Result::Success {
                        value: ::dafny_runtime::upcast_object()(::dafny_runtime::object::new(wrap)),
                    },
                )
            }
            Err(error) => crate::conversions::error::to_opaque_error_result(error),
        }
    }
}

impl crate::implementation_from_dafny::r#_simple_ddocumentation_dinternaldafny_dtypes::ISimpleDocumentationClient
    for Client
{
    fn GetThing(
        &mut self,
        input: &std::rc::Rc<
            crate::implementation_from_dafny::r#_simple_ddocumentation_dinternaldafny_dtypes::GetThingInput,
        >,
    ) -> std::rc::Rc<
        crate::implementation_from_dafny::r#_Wrappers_Compile::Result<
            std::rc::Rc<
                crate::implementation_from_dafny::r#_simple_ddocumentation_dinternaldafny_dtypes::GetThingOutput,
            >,
            std::rc::Rc<crate::implementation_from_dafny::r#_simple_ddocumentation_dinternaldafny_dtypes::Error>,
        >,
    >{
        let inner_input =
            crate::conversions::get_thing::_get_thing_input::from_dafny(input.clone());
        let result = self.rt.block_on(crate::operation::get_thing::GetThing::send(&self.wrapped, inner_input));
        match result {
            Err(error) => ::std::rc::Rc::new(
                crate::implementation_from_dafny::_Wrappers_Compile::Result::Failure {
                    error: crate::conversions::get_thing::to_dafny_error(error),
                },
            ),
            Ok(client) => ::std::rc::Rc::new(
                crate::implementation_from_dafny::_Wrappers_Compile::Result::Success {
                    value: crate::conversions::get_thing::_get_thing_output::to_dafny(client),
                },
            ),
        }
    }
}