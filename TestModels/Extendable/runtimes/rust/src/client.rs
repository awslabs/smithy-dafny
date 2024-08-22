// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

use aws_smithy_types::error::operation::BuildError;

#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct Client {
    pub(crate) dafny_client: ::dafny_runtime::Object<dyn crate::r#simple::extendable::resources::internaldafny::types::ISimpleExtendableResourcesClient>
}

impl Client {
    /// Creates a new client from the service [`Config`](crate::Config).
    #[track_caller]
    pub fn from_conf(
        conf: crate::types::simple_extendable_resources_config::SimpleExtendableResourcesConfig,
    ) -> Result<Self, BuildError> {
        let inner =
            crate::simple::extendable::resources::internaldafny::_default::SimpleExtendableResources(
                &crate::conversions::simple_extendable_resources_config::_simple_extendable_resources_config::to_dafny(
                    conf,
                ),
            );
        if matches!(
            inner.as_ref(),
            crate::_Wrappers_Compile::Result::Failure { .. }
        ) {
            // TODO: convert error - the potential types are not modeled!
            return Err(BuildError::other(
                ::aws_smithy_types::error::metadata::ErrorMetadata::builder()
                    .message("Invalid client config")
                    .build(),
            ));
        }
        Ok(Self {
            dafny_client: ::dafny_runtime::upcast_object()(inner.Extract()),
        })
    }
}

mod create_extendable_resource;
mod use_extendable_resource;