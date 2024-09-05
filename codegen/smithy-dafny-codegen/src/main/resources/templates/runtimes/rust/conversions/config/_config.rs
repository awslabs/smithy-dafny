#[allow(dead_code)]

pub fn to_dafny(
    value: crate::types::$snakeCaseConfigName:L::$configName:L,
) -> ::std::rc::Rc<
    crate::r#$dafnyTypesModuleName:L::$configName:L,
> {
    ::std::rc::Rc::new(to_dafny_plain(value))
}

#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#$dafnyTypesModuleName:L::$configName:L,
    >,
) -> crate::types::$snakeCaseConfigName:L::$configName:L {
    plain_from_dafny(&*dafny_value)
}


#[allow(dead_code)]
pub fn to_dafny_plain(
    value: crate::types::$snakeCaseConfigName:L::$configName:L,
) -> crate::r#$dafnyTypesModuleName:L::$configName:L {
    crate::r#$dafnyTypesModuleName:L::$configName:L::$configName:L {
        $variants:L
    }
}

#[allow(dead_code)]
pub fn plain_from_dafny(
    dafny_value: &crate::r#$dafnyTypesModuleName:L::$configName:L,
) -> crate::types::$snakeCaseConfigName:L::$configName:L {
    match dafny_value {
        crate::r#$dafnyTypesModuleName:L::$structureName:L::$configName:L {..} =>
            crate::types::$snakeCaseConfigName:L::$configName:L::builder()
                $fluentMemberSetters:L
                .build()
                .unwrap()
    }
}
