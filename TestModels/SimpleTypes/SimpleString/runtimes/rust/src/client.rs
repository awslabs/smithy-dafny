// Code intended to be generated by software.amazon.smithy.rust.codegen.smithy-rs.

use std::sync::Arc;

#[derive(Debug)]
pub(crate) struct Handle {
    pub(crate) conf: crate::Config,
    pub(crate) inner: *mut dyn crate::implementation_from_dafny::r#_simple_dtypes_dsmithystring_dinternaldafny_dtypes::ISimpleTypesStringClient
}

#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct Client {
    handle: ::std::sync::Arc<Handle>,
}

impl Client {
    /// Creates a new client from the service [`Config`](crate::Config).
    #[track_caller]
    pub fn from_conf(conf: crate::Config) -> Self {
        // TODO: Support configuration
        let inner_config = ::std::rc::Rc::new(
            crate::implementation_from_dafny::_simple_dtypes_dsmithystring_dinternaldafny::_default::DefaultSimpleStringConfig());
        let inner =
            crate::implementation_from_dafny::_simple_dtypes_dsmithystring_dinternaldafny::_default::SimpleString(&inner_config);
        if matches!((&inner).as_ref(), crate::implementation_from_dafny::_Wrappers_Compile::Result::Failure { .. }) {
            // TODO: display error
            panic!("Invalid client configuration");
        }
        let handle = Handle {
            conf: conf.clone(),
            inner: inner.Extract()
        };
        Self {
            handle: ::std::sync::Arc::new(handle),
        }
    }

    /// Returns the client's configuration.
    pub fn config(&self) -> &crate::Config {
        &self.handle.conf
    }
}

mod get_string;

mod get_string_single_value;

mod get_string_utf8;

