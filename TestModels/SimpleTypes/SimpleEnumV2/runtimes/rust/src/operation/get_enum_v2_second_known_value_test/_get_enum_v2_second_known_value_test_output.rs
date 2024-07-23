// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetEnumV2SecondKnownValueTestOutput {
    #[allow(missing_docs)] // documentation missing in model
    pub value: ::std::option::Option<crate::types::simple_enum_v2_shape::SimpleEnumV2Shape>,
}
impl GetEnumV2SecondKnownValueTestOutput {
    #[allow(missing_docs)] // documentation missing in model
    pub fn value(
        &self,
    ) -> ::std::option::Option<crate::types::simple_enum_v2_shape::SimpleEnumV2Shape> {
        self.value
    }
}
impl GetEnumV2SecondKnownValueTestOutput {
    /// Creates a new builder-style object to manufacture [`GetEnumV2SecondKnownValueTestOutput`](crate::operation::operation::GetEnumV2SecondKnownValueTestOutput).
    pub fn builder(
    ) -> crate::operation::get_enum_v2_second_known_value_test::builders::GetEnumV2SecondKnownValueTestOutputBuilder
    {
        crate::operation::get_enum_v2_second_known_value_test::builders::GetEnumV2SecondKnownValueTestOutputBuilder::default()
    }
}

/// A builder for [`GetEnumV2SecondKnownValueTestOutput`](crate::operation::operation::GetEnumV2SecondKnownValueTestOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetEnumV2SecondKnownValueTestOutputBuilder {
    pub(crate) value: ::std::option::Option<crate::types::simple_enum_v2_shape::SimpleEnumV2Shape>,
}
impl GetEnumV2SecondKnownValueTestOutputBuilder {
    #[allow(missing_docs)] // documentation missing in model
    pub fn value(
        mut self,
        input: impl ::std::convert::Into<crate::types::simple_enum_v2_shape::SimpleEnumV2Shape>,
    ) -> Self {
        self.value = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_value(
        mut self,
        input: ::std::option::Option<crate::types::simple_enum_v2_shape::SimpleEnumV2Shape>,
    ) -> Self {
        self.value = input;
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_value(
        &self,
    ) -> &::std::option::Option<crate::types::simple_enum_v2_shape::SimpleEnumV2Shape> {
        &self.value
    }
    /// Consumes the builder and constructs a [`GetEnumV2SecondKnownValueTestOutput`](crate::operation::operation::GetEnumV2SecondKnownValueTestOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_enum_v2_second_known_value_test::GetEnumV2SecondKnownValueTestOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::get_enum_v2_second_known_value_test::GetEnumV2SecondKnownValueTestOutput {
                value: self.value,
            },
        )
    }
}
