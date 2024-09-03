#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct $structureName:L {
    $fields:L
}
impl $structureName:L {
    $getters:L
}
impl $structureName:L {
    /// Creates a new builder-style object to manufacture [`$structureName:L`](crate::operation::operation::$structureName:L).
    pub fn builder() -> crate::operation::$snakeCaseOperationName:L::builders::$structureName:LBuilder {
        crate::operation::$snakeCaseOperationName:L::builders::$structureName:LBuilder::default()
    }
}

/// A builder for [`$structureName:L`](crate::operation::operation::$structureName:L).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct $structureName:LBuilder {
    $builderFields:L
}
impl $structureName:LBuilder {
    $builderAccessors:L
    /// Consumes the builder and constructs a [`$structureName:L`](crate::operation::operation::$structureName:L).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::$snakeCaseOperationName:L::$structureName:L,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::$snakeCaseOperationName:L::$structureName:L {
            value: self.value,
        })
    }
}