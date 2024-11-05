/// Wraps up an arbitrary Rust Error value as a Dafny Error
pub fn to_opaque_error(value: String) ->
    ::std::rc::Rc<crate::r#$dafnyTypesModuleName:L::Error>
{
    let error_msg = value.clone();
    let error_msg = ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&error_msg);
    let error_obj: ::dafny_runtime::Object<dyn::std::any::Any> = ::dafny_runtime::Object(Some(
        ::std::rc::Rc::new(::std::cell::UnsafeCell::new(value)),
    ));
    ::std::rc::Rc::new(
        crate::r#$dafnyTypesModuleName:L::Error::OpaqueWithText {
            obj: error_obj,
	    objMessage: error_msg
        },
    )
}
