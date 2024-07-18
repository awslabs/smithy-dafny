// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetThingOutput {
    /// The thing that you just got.
    pub thing: crate::types::Thing,
}
impl GetThingOutput {
    /// The thing that you just got.
    pub fn thing(&self) -> &crate::types::Thing {
        &self.thing
    }
}
impl GetThingOutput {
    /// Creates a new builder-style object to manufacture [`GetThingOutput`](crate::operation::get_thing::GetThingOutput).
    pub fn builder() -> crate::operation::get_thing::builders::GetThingOutputBuilder {
        crate::operation::get_thing::builders::GetThingOutputBuilder::default()
    }
}

/// A builder for [`GetThingOutput`](crate::operation::get_thing::GetThingOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetThingOutputBuilder {
    pub(crate) thing: ::std::option::Option<crate::types::Thing>,
}
impl GetThingOutputBuilder {
    /// The thing that you just got.
    /// This field is required.
    pub fn thing(mut self, input: crate::types::Thing) -> Self {
        self.thing = ::std::option::Option::Some(input);
        self
    }
    /// The thing that you just got.
    pub fn set_thing(mut self, input: ::std::option::Option<crate::types::Thing>) -> Self {
        self.thing = input;
        self
    }
    /// The thing that you just got.
    pub fn get_thing(&self) -> &::std::option::Option<crate::types::Thing> {
        &self.thing
    }
    /// Consumes the builder and constructs a [`GetThingOutput`](crate::operation::get_thing::GetThingOutput).
    /// This method will fail if any of the following fields are not set:
    /// - [`thing`](crate::operation::get_thing::builders::GetThingOutputBuilder::thing)
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_thing::GetThingOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::get_thing::GetThingOutput {
            thing: self.thing.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "thing",
                    "thing was not specified but it is required when building GetThingOutput",
                )
            })?,
        })
    }
}