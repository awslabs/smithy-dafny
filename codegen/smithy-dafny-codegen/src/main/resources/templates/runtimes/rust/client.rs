use aws_smithy_types::error::operation::BuildError;

#[derive(::std::clone::Clone, ::std::fmt::Debug, ::std::cmp::PartialEq)]
$rustStructureComment:L
pub struct Client {
    pub(crate) dafny_client: ::dafny_runtime::Object<dyn crate::r#$dafnyTypesModuleName:L::I$serviceName:LClient>
}

impl Client {
    /// Creates a new client from the service [`Config`](crate::Config).
    #[track_caller]
    pub fn from_conf(
        input: $qualifiedRustConfigName:L,
    ) -> Result<Self, $qualifiedRustServiceErrorType:L> {
        $rustRootModuleName:L::validation::$inputValidationFunctionName:L(&input)
            .map_err($qualifiedRustServiceErrorType:L::wrap_validation_err)?;
        let inner =
            crate::$dafnyInternalModuleName:L::_default::$sdkId:L(
                &$rustConversionsModuleName:L::$snakeCaseConfigName:L::_$snakeCaseConfigName:L::to_dafny(input),
            );
        if matches!(
            inner.as_ref(),
            crate::_Wrappers_Compile::Result::Failure { .. }
        ) {
            return Err($rustRootModuleName:L::conversions::error::from_dafny(inner.as_ref().error().clone()));
        }
        Ok(Self {
            dafny_client: ::dafny_runtime::upcast_object()(inner.Extract())
        })
    }
}

$operationModules:L
