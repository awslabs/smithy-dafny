#![allow(warnings, unconditional_panic)]
#![allow(nonstandard_style)]

pub mod r#_SimplePositionalImplTest_Compile {
    pub struct _default {}

    impl _default {
        pub fn _allocate_object() -> ::dafny_runtime::Object<Self> {
            ::dafny_runtime::allocate_object::<Self>()
        }
        pub fn TestClient(
            client: &::dafny_runtime::Object<
                dyn super::r#simple::positional::internaldafny::types::ISimplePositionalClient,
            >,
        ) -> () {
            super::r#_SimplePositionalImplTest_Compile::_default::TestGetResource(client);
            super::r#_SimplePositionalImplTest_Compile::_default::TestGetResourcePositional(client);
            return ();
        }
        pub fn TestGetResource(
            client: &::dafny_runtime::Object<
                dyn super::r#simple::positional::internaldafny::types::ISimplePositionalClient,
            >,
        ) -> () {
            let mut input: ::std::rc::Rc<super::r#simple::positional::internaldafny::types::GetResourceInput> = ::std::rc::Rc::new(super::r#simple::positional::internaldafny::types::GetResourceInput::GetResourceInput {
            name: ::dafny_runtime::string_utf16_of("Test")
          });
            let mut valueOrError0 = ::dafny_runtime::MaybePlacebo::<
                ::std::rc::Rc<
                    super::r#_Wrappers_Compile::Result<
                        ::std::rc::Rc<
                            super::r#simple::positional::internaldafny::types::GetResourceOutput,
                        >,
                        ::std::rc::Rc<super::r#simple::positional::internaldafny::types::Error>,
                    >,
                >,
            >::new();
            let mut _out0 = ::dafny_runtime::MaybePlacebo::<
                ::std::rc::Rc<
                    super::r#_Wrappers_Compile::Result<
                        ::std::rc::Rc<
                            super::r#simple::positional::internaldafny::types::GetResourceOutput,
                        >,
                        ::std::rc::Rc<super::r#simple::positional::internaldafny::types::Error>,
                    >,
                >,
            >::new();
            _out0 = ::dafny_runtime::MaybePlacebo::from(super::r#simple::positional::internaldafny::types::ISimplePositionalClient::GetResource(::dafny_runtime::md!(client.clone()), &input));
            valueOrError0 = ::dafny_runtime::MaybePlacebo::from(_out0.read());
            if !(!valueOrError0.read().IsFailure()) {
                panic!("Halt")
            };
            let mut output: ::std::rc::Rc<
                super::r#simple::positional::internaldafny::types::GetResourceOutput,
            > = valueOrError0.read().Extract();
            let mut resource: ::dafny_runtime::Object<
                dyn super::r#simple::positional::internaldafny::types::ISimpleResource,
            > = output.output().clone();
            let mut valueOrError1 = ::dafny_runtime::MaybePlacebo::<
                ::std::rc::Rc<
                    super::r#_Wrappers_Compile::Result<
                        ::std::rc::Rc<
                            super::r#simple::positional::internaldafny::types::GetNameOutput,
                        >,
                        ::std::rc::Rc<super::r#simple::positional::internaldafny::types::Error>,
                    >,
                >,
            >::new();
            let mut _out1 = ::dafny_runtime::MaybePlacebo::<
                ::std::rc::Rc<
                    super::r#_Wrappers_Compile::Result<
                        ::std::rc::Rc<
                            super::r#simple::positional::internaldafny::types::GetNameOutput,
                        >,
                        ::std::rc::Rc<super::r#simple::positional::internaldafny::types::Error>,
                    >,
                >,
            >::new();
            _out1 = ::dafny_runtime::MaybePlacebo::from(super::r#simple::positional::internaldafny::types::ISimpleResource::GetName(::dafny_runtime::md!(resource.clone()), &::std::rc::Rc::new(super::r#simple::positional::internaldafny::types::GetNameInput::GetNameInput {})));
            valueOrError1 = ::dafny_runtime::MaybePlacebo::from(_out1.read());
            if !(!valueOrError1.read().IsFailure()) {
                panic!("Halt")
            };
            let mut getNameOutput: ::std::rc::Rc<
                super::r#simple::positional::internaldafny::types::GetNameOutput,
            > = valueOrError1.read().Extract();
            if !(getNameOutput.name().clone() == ::dafny_runtime::string_utf16_of("Test")) {
                panic!("Halt")
            };
            return ();
        }
        pub fn TestGetResourcePositional(
            client: &::dafny_runtime::Object<
                dyn super::r#simple::positional::internaldafny::types::ISimplePositionalClient,
            >,
        ) -> () {
            let mut input: ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16> =
                ::dafny_runtime::string_utf16_of("TestPositional");
            let mut valueOrError0 = ::dafny_runtime::MaybePlacebo::<
                ::std::rc::Rc<
                    super::r#_Wrappers_Compile::Result<
                        ::dafny_runtime::Object<
                            dyn super::r#simple::positional::internaldafny::types::ISimpleResource,
                        >,
                        ::std::rc::Rc<super::r#simple::positional::internaldafny::types::Error>,
                    >,
                >,
            >::new();
            let mut _out2 = ::dafny_runtime::MaybePlacebo::<
                ::std::rc::Rc<
                    super::r#_Wrappers_Compile::Result<
                        ::dafny_runtime::Object<
                            dyn super::r#simple::positional::internaldafny::types::ISimpleResource,
                        >,
                        ::std::rc::Rc<super::r#simple::positional::internaldafny::types::Error>,
                    >,
                >,
            >::new();
            _out2 = ::dafny_runtime::MaybePlacebo::from(super::r#simple::positional::internaldafny::types::ISimplePositionalClient::GetResourcePositional(::dafny_runtime::md!(client.clone()), &input));
            valueOrError0 = ::dafny_runtime::MaybePlacebo::from(_out2.read());
            if !(!valueOrError0.read().IsFailure()) {
                panic!("Halt")
            };
            let mut resource: ::dafny_runtime::Object<
                dyn super::r#simple::positional::internaldafny::types::ISimpleResource,
            > = valueOrError0.read().Extract();
            let mut valueOrError1 = ::dafny_runtime::MaybePlacebo::<
                ::std::rc::Rc<
                    super::r#_Wrappers_Compile::Result<
                        ::std::rc::Rc<
                            super::r#simple::positional::internaldafny::types::GetNameOutput,
                        >,
                        ::std::rc::Rc<super::r#simple::positional::internaldafny::types::Error>,
                    >,
                >,
            >::new();
            let mut _out3 = ::dafny_runtime::MaybePlacebo::<
                ::std::rc::Rc<
                    super::r#_Wrappers_Compile::Result<
                        ::std::rc::Rc<
                            super::r#simple::positional::internaldafny::types::GetNameOutput,
                        >,
                        ::std::rc::Rc<super::r#simple::positional::internaldafny::types::Error>,
                    >,
                >,
            >::new();
            _out3 = ::dafny_runtime::MaybePlacebo::from(super::r#simple::positional::internaldafny::types::ISimpleResource::GetName(::dafny_runtime::md!(resource.clone()), &::std::rc::Rc::new(super::r#simple::positional::internaldafny::types::GetNameInput::GetNameInput {})));
            valueOrError1 = ::dafny_runtime::MaybePlacebo::from(_out3.read());
            if !(!valueOrError1.read().IsFailure()) {
                panic!("Halt")
            };
            let mut getNameOutput: ::std::rc::Rc<
                super::r#simple::positional::internaldafny::types::GetNameOutput,
            > = valueOrError1.read().Extract();
            if !(getNameOutput.name().clone() == ::dafny_runtime::string_utf16_of("TestPositional"))
            {
                panic!("Halt")
            };
            return ();
        }
        pub fn TestDefaultConfig() -> () {
            let mut valueOrError0 = ::dafny_runtime::MaybePlacebo::<
                ::std::rc::Rc<
                    super::r#_Wrappers_Compile::Result<
                        ::dafny_runtime::Object<
                            super::r#simple::positional::internaldafny::SimplePositionalClient,
                        >,
                        ::std::rc::Rc<super::r#simple::positional::internaldafny::types::Error>,
                    >,
                >,
            >::new();
            let mut _out4 = ::dafny_runtime::MaybePlacebo::<
                ::std::rc::Rc<
                    super::r#_Wrappers_Compile::Result<
                        ::dafny_runtime::Object<
                            super::r#simple::positional::internaldafny::SimplePositionalClient,
                        >,
                        ::std::rc::Rc<super::r#simple::positional::internaldafny::types::Error>,
                    >,
                >,
            >::new();
            _out4 = ::dafny_runtime::MaybePlacebo::from(super::r#simple::positional::internaldafny::_default::SimplePositional(&super::r#simple::positional::internaldafny::_default::DefaultSimplePositionalConfig()));
            valueOrError0 = ::dafny_runtime::MaybePlacebo::from(_out4.read());
            if !(!valueOrError0.read().IsFailure()) {
                panic!("Halt")
            };
            let mut client: ::dafny_runtime::Object<
                super::r#simple::positional::internaldafny::SimplePositionalClient,
            > = valueOrError0.read().Extract();
            super::r#_SimplePositionalImplTest_Compile::_default::TestClient(
                &::dafny_runtime::upcast_object::<
                    super::r#simple::positional::internaldafny::SimplePositionalClient,
                    dyn super::r#simple::positional::internaldafny::types::ISimplePositionalClient,
                >()(client.clone()),
            );
            return ();
        }
    }

    impl ::dafny_runtime::UpcastObject<dyn::std::any::Any>
        for super::r#_SimplePositionalImplTest_Compile::_default
    {
        ::dafny_runtime::UpcastObjectFn!(dyn::std::any::Any);
    }
}
pub mod r#simple::positional::internaldafny_dwrapped {
    pub struct _default {}

    impl _default {
        pub fn _allocate_object() -> ::dafny_runtime::Object<Self> {
            ::dafny_runtime::allocate_object::<Self>()
        }
        pub fn WrappedDefaultSimplePositionalConfig(
        ) -> ::std::rc::Rc<super::r#simple::positional::internaldafny::types::SimplePositionalConfig>
        {
            ::std::rc::Rc::new(super::r#simple::positional::internaldafny::types::SimplePositionalConfig::SimplePositionalConfig {})
        }
    }

    impl ::dafny_runtime::UpcastObject<dyn::std::any::Any>
        for super::r#simple::positional::internaldafny_dwrapped::_default
    {
        ::dafny_runtime::UpcastObjectFn!(dyn::std::any::Any);
    }
}
pub mod r#_WrappedSimplePositionalTest_Compile {
    pub struct _default {}

    impl _default {
        pub fn _allocate_object() -> ::dafny_runtime::Object<Self> {
            ::dafny_runtime::allocate_object::<Self>()
        }
        pub fn TestWrappedClient() -> () {
            let mut valueOrError0 = ::dafny_runtime::MaybePlacebo::<::std::rc::Rc<super::r#_Wrappers_Compile::Result<::dafny_runtime::Object<dyn super::r#simple::positional::internaldafny::types::ISimplePositionalClient>, ::std::rc::Rc<super::r#simple::positional::internaldafny::types::Error>>>>::new();
            let mut _out5 = ::dafny_runtime::MaybePlacebo::<::std::rc::Rc<super::r#_Wrappers_Compile::Result<::dafny_runtime::Object<dyn super::r#simple::positional::internaldafny::types::ISimplePositionalClient>, ::std::rc::Rc<super::r#simple::positional::internaldafny::types::Error>>>>::new();
            _out5 = ::dafny_runtime::MaybePlacebo::from(super::r#simple::positional::internaldafny_dwrapped::_default::WrappedSimplePositional(&super::r#simple::positional::internaldafny_dwrapped::_default::WrappedDefaultSimplePositionalConfig()));
            valueOrError0 = ::dafny_runtime::MaybePlacebo::from(_out5.read());
            if !(!valueOrError0.read().IsFailure()) {
                panic!("Halt")
            };
            let mut client: ::dafny_runtime::Object<
                dyn super::r#simple::positional::internaldafny::types::ISimplePositionalClient,
            > = valueOrError0.read().Extract();
            super::r#_SimplePositionalImplTest_Compile::_default::TestClient(&client);
            return ();
        }
    }

    impl ::dafny_runtime::UpcastObject<dyn::std::any::Any>
        for super::r#_WrappedSimplePositionalTest_Compile::_default
    {
        ::dafny_runtime::UpcastObjectFn!(dyn::std::any::Any);
    }
}
pub mod _module {
    pub struct _default {}

    impl _default {
        pub fn _allocate_object() -> ::dafny_runtime::Object<Self> {
            ::dafny_runtime::allocate_object::<Self>()
        }
        pub fn _Test__Main_() -> () {
            let mut success: bool = true;
            print!(
                "{}",
                ::dafny_runtime::DafnyPrintWrapper(&::dafny_runtime::string_utf16_of(
                    r#"SimplePositionalImplTest.TestDefaultConfig: "#
                ))
            );
            super::r#_SimplePositionalImplTest_Compile::_default::TestDefaultConfig();
            print!(
                "{}",
                ::dafny_runtime::DafnyPrintWrapper(&::dafny_runtime::string_utf16_of(
                    r#"PASSED
"#
                ))
            );
            print!(
                "{}",
                ::dafny_runtime::DafnyPrintWrapper(&::dafny_runtime::string_utf16_of(
                    r#"WrappedSimplePositionalTest.TestWrappedClient: "#
                ))
            );
            super::r#_WrappedSimplePositionalTest_Compile::_default::TestWrappedClient();
            print!(
                "{}",
                ::dafny_runtime::DafnyPrintWrapper(&::dafny_runtime::string_utf16_of(
                    r#"PASSED
"#
                ))
            );
            if !success {
                panic!("Halt")
            };
            return ();
        }
    }

    impl ::dafny_runtime::UpcastObject<dyn::std::any::Any> for super::_module::_default {
        ::dafny_runtime::UpcastObjectFn!(dyn::std::any::Any);
    }
}
fn main() {
    _module::_default::_Test__Main_();
}
