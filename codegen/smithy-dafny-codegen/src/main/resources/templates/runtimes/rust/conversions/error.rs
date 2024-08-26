/// Wraps up an arbitrary Rust Error value as a Dafny Error
pub fn to_opaque_error<E: 'static>(value: E) ->
  ::std::rc::Rc<crate::r#$dafnyTypesModuleName:L::Error>
{
    let error_obj: ::dafny_runtime::Object<dyn::std::any::Any> = ::dafny_runtime::Object(Some(
        ::std::rc::Rc::new(::std::cell::UnsafeCell::new(value)),
    ));
    ::std::rc::Rc::new(
    crate::r#$dafnyTypesModuleName:L::Error::Opaque {
        obj: error_obj,
    },
  )
}

/// Wraps up an arbitrary Rust Error value as a Dafny Result<T, Error>.Failure
pub fn to_opaque_error_result<T: dafny_runtime::DafnyType, E: 'static>(value: E) ->
  ::std::rc::Rc<
    crate::_Wrappers_Compile::Result<
      T,
      ::std::rc::Rc<crate::r#$dafnyTypesModuleName:L::Error>
    >
  >
{
    ::std::rc::Rc::new(
        crate::_Wrappers_Compile::Result::Failure {
            error: to_opaque_error(value),
        },
    )
}
