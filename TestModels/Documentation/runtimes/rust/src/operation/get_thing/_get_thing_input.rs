// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Inputs for getting a thing.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetThingInput {
    /// The name of the thing to get, obviously.
    pub name: ::std::option::Option<::std::string::String>,
}
impl GetThingInput {
    /// The name of the thing to get, obviously.
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
}
impl GetThingInput {
    /// Creates a new builder-style object to manufacture [`GetThingInput`](crate::operation::get_thing::GetThingInput).
    pub fn builder() -> crate::operation::get_thing::builders::GetThingInputBuilder {
        crate::operation::get_thing::builders::GetThingInputBuilder::default()
    }
}

/// A builder for [`GetThingInput`](crate::operation::get_thing::GetThingInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetThingInputBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
}
impl GetThingInputBuilder {
    /// The name of the thing to get, obviously.
    /// This field is required.
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// The name of the thing to get, obviously.
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// The name of the thing to get, obviously.
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.name
    }
    /// Consumes the builder and constructs a [`GetThingInput`](crate::operation::get_thing::GetThingInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_thing::GetThingInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::get_thing::GetThingInput { name: self.name })
    }
}
