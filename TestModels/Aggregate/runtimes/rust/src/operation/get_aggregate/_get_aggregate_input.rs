// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetAggregateInput {
    #[allow(missing_docs)] // documentation missing in model
    pub simple_string_list: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[allow(missing_docs)] // documentation missing in model
    pub structure_list: ::std::option::Option<::std::vec::Vec<crate::types::StructureListElement>>,
    #[allow(missing_docs)] // documentation missing in model
    pub simple_string_map: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
    #[allow(missing_docs)] // documentation missing in model
    pub simple_integer_map:
        ::std::option::Option<::std::collections::HashMap<::std::string::String, i32>>,
    #[allow(missing_docs)] // documentation missing in model
    pub nested_structure: ::std::option::Option<crate::types::NestedStructure>,
}
impl GetAggregateInput {
    #[allow(missing_docs)] // documentation missing in model
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.simple_string_list.is_none()`.
    pub fn simple_string_list(&self) -> &[::std::string::String] {
        self.simple_string_list.as_deref().unwrap_or_default()
    }
    #[allow(missing_docs)] // documentation missing in model
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.structure_list.is_none()`.
    pub fn structure_list(&self) -> &[crate::types::StructureListElement] {
        self.structure_list.as_deref().unwrap_or_default()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn simple_string_map(
        &self,
    ) -> ::std::option::Option<
        &::std::collections::HashMap<::std::string::String, ::std::string::String>,
    > {
        self.simple_string_map.as_ref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn simple_integer_map(
        &self,
    ) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, i32>> {
        self.simple_integer_map.as_ref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn nested_structure(&self) -> ::std::option::Option<&crate::types::NestedStructure> {
        self.nested_structure.as_ref()
    }
}
impl GetAggregateInput {
    /// Creates a new builder-style object to manufacture [`GetAggregateInput`](crate::operation::get_aggregate::GetAggregateInput).
    pub fn builder() -> crate::operation::get_aggregate::builders::GetAggregateInputBuilder {
        crate::operation::get_aggregate::builders::GetAggregateInputBuilder::default()
    }
}

/// A builder for [`GetAggregateInput`](crate::operation::get_aggregate::GetAggregateInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetAggregateInputBuilder {
    pub(crate) simple_string_list: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) structure_list:
        ::std::option::Option<::std::vec::Vec<crate::types::StructureListElement>>,
    pub(crate) simple_string_map: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
    pub(crate) simple_integer_map:
        ::std::option::Option<::std::collections::HashMap<::std::string::String, i32>>,
    pub(crate) nested_structure: ::std::option::Option<crate::types::NestedStructure>,
}
impl GetAggregateInputBuilder {
    /// Appends an item to `simple_string_list`.
    ///
    /// To override the contents of this collection use [`set_simple_string_list`](Self::set_simple_string_list).
    ///
    pub fn simple_string_list(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.simple_string_list.unwrap_or_default();
        v.push(input.into());
        self.simple_string_list = ::std::option::Option::Some(v);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_simple_string_list(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.simple_string_list = input;
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_simple_string_list(
        &self,
    ) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.simple_string_list
    }
    /// Appends an item to `structure_list`.
    ///
    /// To override the contents of this collection use [`set_structure_list`](Self::set_structure_list).
    ///
    pub fn structure_list(mut self, input: crate::types::StructureListElement) -> Self {
        let mut v = self.structure_list.unwrap_or_default();
        v.push(input);
        self.structure_list = ::std::option::Option::Some(v);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_structure_list(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::StructureListElement>>,
    ) -> Self {
        self.structure_list = input;
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_structure_list(
        &self,
    ) -> &::std::option::Option<::std::vec::Vec<crate::types::StructureListElement>> {
        &self.structure_list
    }
    /// Adds a key-value pair to `simple_string_map`.
    ///
    /// To override the contents of this collection use [`set_simple_string_map`](Self::set_simple_string_map).
    ///
    pub fn simple_string_map(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut hash_map = self.simple_string_map.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.simple_string_map = ::std::option::Option::Some(hash_map);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_simple_string_map(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.simple_string_map = input;
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_simple_string_map(
        &self,
    ) -> &::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    > {
        &self.simple_string_map
    }
    /// Adds a key-value pair to `simple_integer_map`.
    ///
    /// To override the contents of this collection use [`set_simple_integer_map`](Self::set_simple_integer_map).
    ///
    pub fn simple_integer_map(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: i32,
    ) -> Self {
        let mut hash_map = self.simple_integer_map.unwrap_or_default();
        hash_map.insert(k.into(), v);
        self.simple_integer_map = ::std::option::Option::Some(hash_map);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_simple_integer_map(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, i32>>,
    ) -> Self {
        self.simple_integer_map = input;
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_simple_integer_map(
        &self,
    ) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, i32>> {
        &self.simple_integer_map
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn nested_structure(mut self, input: crate::types::NestedStructure) -> Self {
        self.nested_structure = ::std::option::Option::Some(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_nested_structure(
        mut self,
        input: ::std::option::Option<crate::types::NestedStructure>,
    ) -> Self {
        self.nested_structure = input;
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_nested_structure(&self) -> &::std::option::Option<crate::types::NestedStructure> {
        &self.nested_structure
    }
    /// Consumes the builder and constructs a [`GetAggregateInput`](crate::operation::get_aggregate::GetAggregateInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_aggregate::GetAggregateInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::get_aggregate::GetAggregateInput {
            simple_string_list: self.simple_string_list,
            structure_list: self.structure_list,
            simple_string_map: self.simple_string_map,
            simple_integer_map: self.simple_integer_map,
            nested_structure: self.nested_structure,
        })
    }
}
