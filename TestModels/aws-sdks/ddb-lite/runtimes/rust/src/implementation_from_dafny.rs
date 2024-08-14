#![allow(warnings, unconditional_panic)]
#![allow(nonstandard_style)]

mod client;
mod conversions;
mod standard_library_conversions;

pub mod r#_Wrappers_Compile {
    pub use dafny_runtime::DafnyPrint;
    pub use std::cmp::Eq;
    pub use std::convert::AsRef;
    pub use std::default::Default;
    pub use std::fmt::Debug;
    pub use std::hash::Hash;

    pub struct _default {}

    impl _default {
        pub fn Need<_E: ::dafny_runtime::DafnyType>(
            condition: bool,
            error: &_E,
        ) -> ::std::rc::Rc<crate::r#_Wrappers_Compile::Outcome<_E>> {
            if condition {
                ::std::rc::Rc::new(crate::r#_Wrappers_Compile::Outcome::<_E>::Pass {})
            } else {
                ::std::rc::Rc::new(crate::r#_Wrappers_Compile::Outcome::<_E>::Fail {
                    error: error.clone(),
                })
            }
        }
    }

    #[derive(PartialEq, Clone)]
    pub enum Option<T: ::dafny_runtime::DafnyType> {
        None {},
        Some { value: T },
    }

    impl<T: ::dafny_runtime::DafnyType> Option<T> {
        pub fn ToResult(
            self: &::std::rc::Rc<Self>,
        ) -> ::std::rc::Rc<
            crate::r#_Wrappers_Compile::Result<
                T,
                ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
            >,
        > {
            let mut _source0: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<T>> = self.clone();
            if matches!(
                (&_source0).as_ref(),
                crate::r#_Wrappers_Compile::Option::None { .. }
            ) {
                ::std::rc::Rc::new(crate::r#_Wrappers_Compile::Result::<
                    T,
                    ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                >::Failure {
                    error: ::dafny_runtime::string_utf16_of("Option is None"),
                })
            } else {
                let mut r#___mcc_h0: T = _source0.value().clone();
                let mut v: T = r#___mcc_h0.clone();
                ::std::rc::Rc::new(crate::r#_Wrappers_Compile::Result::<
                    T,
                    ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                >::Success {
                    value: v.clone(),
                })
            }
        }
        pub fn UnwrapOr(self: &::std::rc::Rc<Self>, default: &T) -> T {
            let mut _source1: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<T>> = self.clone();
            if matches!(
                (&_source1).as_ref(),
                crate::r#_Wrappers_Compile::Option::None { .. }
            ) {
                default.clone()
            } else {
                let mut r#___mcc_h0: T = _source1.value().clone();
                let mut v: T = r#___mcc_h0.clone();
                v.clone()
            }
        }
        pub fn IsFailure(self: &::std::rc::Rc<Self>) -> bool {
            matches!(
                self.as_ref(),
                crate::r#_Wrappers_Compile::Option::None { .. }
            )
        }
        pub fn PropagateFailure<_U: ::dafny_runtime::DafnyType>(
            self: &::std::rc::Rc<Self>,
        ) -> ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<_U>> {
            ::std::rc::Rc::new(crate::r#_Wrappers_Compile::Option::<_U>::None {})
        }
        pub fn Extract(self: &::std::rc::Rc<Self>) -> T {
            self.value().clone()
        }
        pub fn value(&self) -> &T {
            match self {
                Option::None {} => panic!("field does not exist on this variant"),
                Option::Some { value } => value,
            }
        }
    }

    impl<T: ::dafny_runtime::DafnyType> Debug for Option<T> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
        }
    }

    impl<T: ::dafny_runtime::DafnyType> DafnyPrint for Option<T> {
        fn fmt_print(
            &self,
            _formatter: &mut ::std::fmt::Formatter,
            _in_seq: bool,
        ) -> std::fmt::Result {
            match self {
                Option::None {} => {
                    write!(_formatter, "Wrappers_Compile.Option.None")?;
                    Ok(())
                }
                Option::Some { value } => {
                    write!(_formatter, "Wrappers_Compile.Option.Some(")?;
                    ::dafny_runtime::DafnyPrint::fmt_print(value, _formatter, false)?;
                    write!(_formatter, ")")?;
                    Ok(())
                }
            }
        }
    }

    impl<T: ::dafny_runtime::DafnyType> Option<T> {
        pub fn coerce<r#__T0: ::dafny_runtime::DafnyType>(
            f_0: ::std::rc::Rc<impl ::std::ops::Fn(T) -> r#__T0 + 'static>,
        ) -> ::std::rc::Rc<impl ::std::ops::Fn(Option<T>) -> Option<r#__T0>> {
            ::std::rc::Rc::new(move |this: Self| -> Option<r#__T0> {
                match this {
                    Option::None {} => Option::None {},
                    Option::Some { value } => Option::Some {
                        value: f_0.clone()(value),
                    },
                }
            })
        }
    }

    impl<T: ::dafny_runtime::DafnyType + Eq> Eq for Option<T> {}

    impl<T: ::dafny_runtime::DafnyType + Hash> Hash for Option<T> {
        fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
            match self {
                Option::None {} => {}
                Option::Some { value } => ::std::hash::Hash::hash(value, _state),
            }
        }
    }

    impl<T: ::dafny_runtime::DafnyType + Default> Default for Option<T> {
        fn default() -> Option<T> {
            Option::None {}
        }
    }

    impl<T: ::dafny_runtime::DafnyType> AsRef<Option<T>> for &Option<T> {
        fn as_ref(&self) -> Self {
            self
        }
    }

    #[derive(PartialEq, Clone)]
    pub enum Result<T: ::dafny_runtime::DafnyType, R: ::dafny_runtime::DafnyType> {
        Success { value: T },
        Failure { error: R },
    }

    impl<T: ::dafny_runtime::DafnyType, R: ::dafny_runtime::DafnyType> Result<T, R> {
        pub fn ToOption(
            self: &::std::rc::Rc<Self>,
        ) -> ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<T>> {
            let mut _source2: ::std::rc::Rc<crate::r#_Wrappers_Compile::Result<T, R>> =
                self.clone();
            if matches!(
                (&_source2).as_ref(),
                crate::r#_Wrappers_Compile::Result::Success { .. }
            ) {
                let mut r#___mcc_h0: T = _source2.value().clone();
                let mut s: T = r#___mcc_h0.clone();
                ::std::rc::Rc::new(crate::r#_Wrappers_Compile::Option::<T>::Some {
                    value: s.clone(),
                })
            } else {
                let mut r#___mcc_h1: R = _source2.error().clone();
                let mut e: R = r#___mcc_h1.clone();
                ::std::rc::Rc::new(crate::r#_Wrappers_Compile::Option::<T>::None {})
            }
        }
        pub fn UnwrapOr(self: &::std::rc::Rc<Self>, default: &T) -> T {
            let mut _source3: ::std::rc::Rc<crate::r#_Wrappers_Compile::Result<T, R>> =
                self.clone();
            if matches!(
                (&_source3).as_ref(),
                crate::r#_Wrappers_Compile::Result::Success { .. }
            ) {
                let mut r#___mcc_h0: T = _source3.value().clone();
                let mut s: T = r#___mcc_h0.clone();
                s.clone()
            } else {
                let mut r#___mcc_h1: R = _source3.error().clone();
                let mut e: R = r#___mcc_h1.clone();
                default.clone()
            }
        }
        pub fn IsFailure(self: &::std::rc::Rc<Self>) -> bool {
            matches!(
                self.as_ref(),
                crate::r#_Wrappers_Compile::Result::Failure { .. }
            )
        }
        pub fn PropagateFailure<_U: ::dafny_runtime::DafnyType>(
            self: &::std::rc::Rc<Self>,
        ) -> ::std::rc::Rc<crate::r#_Wrappers_Compile::Result<_U, R>> {
            ::std::rc::Rc::new(crate::r#_Wrappers_Compile::Result::<_U, R>::Failure {
                error: self.error().clone(),
            })
        }
        pub fn MapFailure<_NewR: ::dafny_runtime::DafnyType>(
            self: &::std::rc::Rc<Self>,
            reWrap: &::std::rc::Rc<dyn::std::ops::Fn(&R) -> _NewR>,
        ) -> ::std::rc::Rc<crate::r#_Wrappers_Compile::Result<T, _NewR>> {
            let mut _source4: ::std::rc::Rc<crate::r#_Wrappers_Compile::Result<T, R>> =
                self.clone();
            if matches!(
                (&_source4).as_ref(),
                crate::r#_Wrappers_Compile::Result::Success { .. }
            ) {
                let mut r#___mcc_h0: T = _source4.value().clone();
                let mut s: T = r#___mcc_h0.clone();
                ::std::rc::Rc::new(crate::r#_Wrappers_Compile::Result::<T, _NewR>::Success {
                    value: s.clone(),
                })
            } else {
                let mut r#___mcc_h1: R = _source4.error().clone();
                let mut e: R = r#___mcc_h1.clone();
                ::std::rc::Rc::new(crate::r#_Wrappers_Compile::Result::<T, _NewR>::Failure {
                    error: reWrap(&e),
                })
            }
        }
        pub fn Extract(self: &::std::rc::Rc<Self>) -> T {
            self.value().clone()
        }
        pub fn value(&self) -> &T {
            match self {
                Result::Success { value } => value,
                Result::Failure { error } => panic!("field does not exist on this variant"),
            }
        }
        pub fn error(&self) -> &R {
            match self {
                Result::Success { value } => panic!("field does not exist on this variant"),
                Result::Failure { error } => error,
            }
        }
    }

    impl<T: ::dafny_runtime::DafnyType, R: ::dafny_runtime::DafnyType> Debug for Result<T, R> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
        }
    }

    impl<T: ::dafny_runtime::DafnyType, R: ::dafny_runtime::DafnyType> DafnyPrint for Result<T, R> {
        fn fmt_print(
            &self,
            _formatter: &mut ::std::fmt::Formatter,
            _in_seq: bool,
        ) -> std::fmt::Result {
            match self {
                Result::Success { value } => {
                    write!(_formatter, "Wrappers_Compile.Result.Success(")?;
                    ::dafny_runtime::DafnyPrint::fmt_print(value, _formatter, false)?;
                    write!(_formatter, ")")?;
                    Ok(())
                }
                Result::Failure { error } => {
                    write!(_formatter, "Wrappers_Compile.Result.Failure(")?;
                    ::dafny_runtime::DafnyPrint::fmt_print(error, _formatter, false)?;
                    write!(_formatter, ")")?;
                    Ok(())
                }
            }
        }
    }

    impl<T: ::dafny_runtime::DafnyType, R: ::dafny_runtime::DafnyType> Result<T, R> {
        pub fn coerce<r#__T0: ::dafny_runtime::DafnyType, r#__T1: ::dafny_runtime::DafnyType>(
            f_0: ::std::rc::Rc<impl ::std::ops::Fn(T) -> r#__T0 + 'static>,
            f_1: ::std::rc::Rc<impl ::std::ops::Fn(R) -> r#__T1 + 'static>,
        ) -> ::std::rc::Rc<impl ::std::ops::Fn(Result<T, R>) -> Result<r#__T0, r#__T1>> {
            ::std::rc::Rc::new(move |this: Self| -> Result<r#__T0, r#__T1> {
                match this {
                    Result::Success { value } => Result::Success {
                        value: f_0.clone()(value),
                    },
                    Result::Failure { error } => Result::Failure {
                        error: f_1.clone()(error),
                    },
                }
            })
        }
    }

    impl<T: ::dafny_runtime::DafnyType + Eq, R: ::dafny_runtime::DafnyType + Eq> Eq for Result<T, R> {}

    impl<T: ::dafny_runtime::DafnyType + Hash, R: ::dafny_runtime::DafnyType + Hash> Hash
        for Result<T, R>
    {
        fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
            match self {
                Result::Success { value } => ::std::hash::Hash::hash(value, _state),
                Result::Failure { error } => ::std::hash::Hash::hash(error, _state),
            }
        }
    }

    impl<T: ::dafny_runtime::DafnyType + Default, R: ::dafny_runtime::DafnyType + Default> Default
        for Result<T, R>
    {
        fn default() -> Result<T, R> {
            Result::Success {
                value: ::std::default::Default::default(),
            }
        }
    }

    impl<T: ::dafny_runtime::DafnyType, R: ::dafny_runtime::DafnyType> AsRef<Result<T, R>>
        for &Result<T, R>
    {
        fn as_ref(&self) -> Self {
            self
        }
    }

    #[derive(PartialEq, Clone)]
    pub enum Outcome<E: ::dafny_runtime::DafnyType> {
        Pass {},
        Fail { error: E },
    }

    impl<E: ::dafny_runtime::DafnyType> Outcome<E> {
        pub fn IsFailure(self: &::std::rc::Rc<Self>) -> bool {
            matches!(
                self.as_ref(),
                crate::r#_Wrappers_Compile::Outcome::Fail { .. }
            )
        }
        pub fn PropagateFailure<_U: ::dafny_runtime::DafnyType>(
            self: &::std::rc::Rc<Self>,
        ) -> ::std::rc::Rc<crate::r#_Wrappers_Compile::Result<_U, E>> {
            ::std::rc::Rc::new(crate::r#_Wrappers_Compile::Result::<_U, E>::Failure {
                error: self.error().clone(),
            })
        }
        pub fn error(&self) -> &E {
            match self {
                Outcome::Pass {} => panic!("field does not exist on this variant"),
                Outcome::Fail { error } => error,
            }
        }
    }

    impl<E: ::dafny_runtime::DafnyType> Debug for Outcome<E> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
        }
    }

    impl<E: ::dafny_runtime::DafnyType> DafnyPrint for Outcome<E> {
        fn fmt_print(
            &self,
            _formatter: &mut ::std::fmt::Formatter,
            _in_seq: bool,
        ) -> std::fmt::Result {
            match self {
                Outcome::Pass {} => {
                    write!(_formatter, "Wrappers_Compile.Outcome.Pass")?;
                    Ok(())
                }
                Outcome::Fail { error } => {
                    write!(_formatter, "Wrappers_Compile.Outcome.Fail(")?;
                    ::dafny_runtime::DafnyPrint::fmt_print(error, _formatter, false)?;
                    write!(_formatter, ")")?;
                    Ok(())
                }
            }
        }
    }

    impl<E: ::dafny_runtime::DafnyType + Eq> Eq for Outcome<E> {}

    impl<E: ::dafny_runtime::DafnyType + Hash> Hash for Outcome<E> {
        fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
            match self {
                Outcome::Pass {} => {}
                Outcome::Fail { error } => ::std::hash::Hash::hash(error, _state),
            }
        }
    }

    impl<E: ::dafny_runtime::DafnyType + Default> Default for Outcome<E> {
        fn default() -> Outcome<E> {
            Outcome::Pass {}
        }
    }

    impl<E: ::dafny_runtime::DafnyType> AsRef<Outcome<E>> for &Outcome<E> {
        fn as_ref(&self) -> Self {
            self
        }
    }
}
pub mod r#_StandardLibrary_Compile {
    pub struct _default {}

    impl _default {
        pub fn Join<_T: ::dafny_runtime::DafnyType>(
            ss: &::dafny_runtime::Sequence<::dafny_runtime::Sequence<_T>>,
            joiner: &::dafny_runtime::Sequence<_T>,
        ) -> ::dafny_runtime::Sequence<_T> {
            let mut _accumulator: ::dafny_runtime::Sequence<_T> =
                ::dafny_runtime::seq![] as ::dafny_runtime::Sequence<_T>;
            let mut ss = ss.clone();
            let mut joiner = joiner.clone();
            let mut _accumulator = _accumulator.clone();
            'TAIL_CALL_START: loop {
                if ss.cardinality() == ::dafny_runtime::int!(1) {
                    return _accumulator.concat(&ss.get(&::dafny_runtime::int!(0)));
                } else {
                    _accumulator =
                        _accumulator.concat(&ss.get(&::dafny_runtime::int!(0)).concat(&joiner));
                    let mut _in0: ::dafny_runtime::Sequence<::dafny_runtime::Sequence<_T>> =
                        ss.drop(&::dafny_runtime::int!(1));
                    let mut _in1: ::dafny_runtime::Sequence<_T> = joiner.clone();
                    ss = _in0.clone();
                    joiner = _in1.clone();
                    continue 'TAIL_CALL_START;
                }
            }
        }
        pub fn Split<_T: ::dafny_runtime::DafnyTypeEq>(
            s: &::dafny_runtime::Sequence<_T>,
            delim: &_T,
        ) -> ::dafny_runtime::Sequence<::dafny_runtime::Sequence<_T>> {
            let mut _accumulator: ::dafny_runtime::Sequence<::dafny_runtime::Sequence<_T>> =
                ::dafny_runtime::seq![] as ::dafny_runtime::Sequence<::dafny_runtime::Sequence<_T>>;
            let mut s = s.clone();
            let mut delim = delim.clone();
            let mut _accumulator = _accumulator.clone();
            'TAIL_CALL_START: loop {
                let mut i: ::std::rc::Rc<
                    crate::r#_Wrappers_Compile::Option<::dafny_runtime::_System::nat>,
                > = crate::r#_StandardLibrary_Compile::_default::FindIndexMatching::<_T>(
                    &s,
                    &delim,
                    &::dafny_runtime::int!(0),
                );
                if matches!(
                    (&i).as_ref(),
                    crate::r#_Wrappers_Compile::Option::Some { .. }
                ) {
                    _accumulator = _accumulator.concat(&::dafny_runtime::seq![s.take(i.value())]);
                    let mut _in2: ::dafny_runtime::Sequence<_T> =
                        s.drop(&(i.value().clone() + ::dafny_runtime::int!(1)));
                    let mut _in3: _T = delim.clone();
                    s = _in2.clone();
                    delim = _in3.clone();
                    continue 'TAIL_CALL_START;
                } else {
                    return _accumulator.concat(&::dafny_runtime::seq![s.clone()]);
                }
            }
        }
        pub fn SplitOnce<_T: ::dafny_runtime::DafnyTypeEq>(
            s: &::dafny_runtime::Sequence<_T>,
            delim: &_T,
        ) -> (::dafny_runtime::Sequence<_T>, ::dafny_runtime::Sequence<_T>) {
            let mut i: ::std::rc::Rc<
                crate::r#_Wrappers_Compile::Option<::dafny_runtime::_System::nat>,
            > = crate::r#_StandardLibrary_Compile::_default::FindIndexMatching::<_T>(
                s,
                delim,
                &::dafny_runtime::int!(0),
            );
            (
                s.take(i.value()),
                s.drop(&(i.value().clone() + ::dafny_runtime::int!(1))),
            )
        }
        pub fn r#_SplitOnce_q<_T: ::dafny_runtime::DafnyTypeEq>(
            s: &::dafny_runtime::Sequence<_T>,
            delim: &_T,
        ) -> ::std::rc::Rc<
            crate::r#_Wrappers_Compile::Option<(
                ::dafny_runtime::Sequence<_T>,
                ::dafny_runtime::Sequence<_T>,
            )>,
        > {
            let mut valueOrError0: ::std::rc::Rc<
                crate::r#_Wrappers_Compile::Option<::dafny_runtime::_System::nat>,
            > = crate::r#_StandardLibrary_Compile::_default::FindIndexMatching::<_T>(
                s,
                delim,
                &::dafny_runtime::int!(0),
            );
            if valueOrError0.IsFailure() {
                valueOrError0.PropagateFailure::<(::dafny_runtime::Sequence<_T>, ::dafny_runtime::Sequence<_T>)>()
            } else {
                let mut i: ::dafny_runtime::_System::nat = valueOrError0.Extract();
                ::std::rc::Rc::new(crate::r#_Wrappers_Compile::Option::<(
                    ::dafny_runtime::Sequence<_T>,
                    ::dafny_runtime::Sequence<_T>,
                )>::Some {
                    value: (s.take(&i), s.drop(&(i.clone() + ::dafny_runtime::int!(1)))),
                })
            }
        }
        pub fn FindIndexMatching<_T: ::dafny_runtime::DafnyTypeEq>(
            s: &::dafny_runtime::Sequence<_T>,
            c: &_T,
            i: &::dafny_runtime::_System::nat,
        ) -> ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::_System::nat>>
        {
            crate::r#_StandardLibrary_Compile::_default::FindIndex::<_T>(
                s,
                {
                    let c: _T = c.clone();
                    &({
                        let mut c = c.clone();
                        ::std::rc::Rc::new(move |x: &_T| -> bool { x.clone() == c.clone() })
                    })
                },
                i,
            )
        }
        pub fn FindIndex<_T: ::dafny_runtime::DafnyType>(
            s: &::dafny_runtime::Sequence<_T>,
            f: &::std::rc::Rc<dyn::std::ops::Fn(&_T) -> bool>,
            i: &::dafny_runtime::_System::nat,
        ) -> ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::_System::nat>>
        {
            let mut s = s.clone();
            let mut f = f.clone();
            let mut i = i.clone();
            'TAIL_CALL_START: loop {
                if i.clone() == s.cardinality() {
                    return ::std::rc::Rc::new(crate::r#_Wrappers_Compile::Option::<
                        ::dafny_runtime::_System::nat,
                    >::None {});
                } else {
                    if (&f)(&s.get(&i)) {
                        return ::std::rc::Rc::new(crate::r#_Wrappers_Compile::Option::<
                            ::dafny_runtime::_System::nat,
                        >::Some {
                            value: i.clone(),
                        });
                    } else {
                        let mut _in4: ::dafny_runtime::Sequence<_T> = s.clone();
                        let mut _in5: ::std::rc::Rc<dyn::std::ops::Fn(&_T) -> bool> = f.clone();
                        let mut _in6: ::dafny_runtime::DafnyInt =
                            i.clone() + ::dafny_runtime::int!(1);
                        s = _in4.clone();
                        f = _in5.clone();
                        i = _in6.clone();
                        continue 'TAIL_CALL_START;
                    }
                }
            }
        }
        pub fn Filter<_T: ::dafny_runtime::DafnyType>(
            s: &::dafny_runtime::Sequence<_T>,
            f: &::std::rc::Rc<dyn::std::ops::Fn(&_T) -> bool>,
        ) -> ::dafny_runtime::Sequence<_T> {
            let mut _accumulator: ::dafny_runtime::Sequence<_T> =
                ::dafny_runtime::seq![] as ::dafny_runtime::Sequence<_T>;
            let mut s = s.clone();
            let mut f = f.clone();
            let mut _accumulator = _accumulator.clone();
            'TAIL_CALL_START: loop {
                if s.cardinality() == ::dafny_runtime::int!(0) {
                    return _accumulator
                        .concat(&(::dafny_runtime::seq![] as ::dafny_runtime::Sequence<_T>));
                } else {
                    if (&f)(&s.get(&::dafny_runtime::int!(0))) {
                        _accumulator = _accumulator
                            .concat(&::dafny_runtime::seq![s.get(&::dafny_runtime::int!(0))]);
                        let mut _in7: ::dafny_runtime::Sequence<_T> =
                            s.drop(&::dafny_runtime::int!(1));
                        let mut _in8: ::std::rc::Rc<dyn::std::ops::Fn(&_T) -> bool> = f.clone();
                        s = _in7.clone();
                        f = _in8.clone();
                        continue 'TAIL_CALL_START;
                    } else {
                        let mut _in9: ::dafny_runtime::Sequence<_T> =
                            s.drop(&::dafny_runtime::int!(1));
                        let mut _in10: ::std::rc::Rc<dyn::std::ops::Fn(&_T) -> bool> = f.clone();
                        s = _in9.clone();
                        f = _in10.clone();
                        continue 'TAIL_CALL_START;
                    }
                }
            }
        }
        pub fn Min(
            a: &::dafny_runtime::DafnyInt,
            b: &::dafny_runtime::DafnyInt,
        ) -> ::dafny_runtime::DafnyInt {
            if a.clone() < b.clone() {
                a.clone()
            } else {
                b.clone()
            }
        }
        pub fn Fill<_T: ::dafny_runtime::DafnyType>(
            value: &_T,
            n: &::dafny_runtime::_System::nat,
        ) -> ::dafny_runtime::Sequence<_T> {
            {
                let _initializer = {
                    let value: _T = value.clone();
                    {
                        let mut value = value.clone();
                        ::std::rc::Rc::new(move |_v0: &::dafny_runtime::DafnyInt| -> _T {
                            value.clone()
                        })
                    }
                };
                ::dafny_runtime::integer_range(::dafny_runtime::Zero::zero(), n.clone())
                    .map(|i| _initializer(&i))
                    .collect::<::dafny_runtime::Sequence<_>>()
            }
        }
        pub fn SeqToArray<_T: ::dafny_runtime::DafnyType>(
            s: &::dafny_runtime::Sequence<_T>,
        ) -> ::dafny_runtime::Object<[_T]> {
            let mut a = ::dafny_runtime::MaybePlacebo::<::dafny_runtime::Object<[_T]>>::new();
            let mut _init0: ::std::rc::Rc<dyn::std::ops::Fn(&::dafny_runtime::DafnyInt) -> _T> = {
                let s: ::dafny_runtime::Sequence<_T> = s.clone();
                {
                    let mut s = s.clone();
                    ::std::rc::Rc::new(move |i: &::dafny_runtime::DafnyInt| -> _T { s.get(i) })
                }
            };
            let mut _nw0: ::dafny_runtime::Object<[::std::mem::MaybeUninit<_T>]> =
                ::dafny_runtime::array::placebos_usize_object::<_T>(
                    ::dafny_runtime::DafnyUsize::into_usize(s.cardinality()),
                );
            for r#__i0_0 in
                ::dafny_runtime::integer_range(0, ::dafny_runtime::rd!(_nw0.clone()).len())
            {
                {
                    let __idx0 = ::dafny_runtime::DafnyUsize::into_usize(r#__i0_0.clone());
                    ::dafny_runtime::md!(_nw0)[__idx0] = ::std::mem::MaybeUninit::new((&_init0)(
                        &::dafny_runtime::int!(r#__i0_0.clone()),
                    ));
                }
            }
            a = ::dafny_runtime::MaybePlacebo::from(::dafny_runtime::array::construct_object(
                _nw0.clone(),
            ));
            return a.read();
        }
        pub fn LexicographicLessOrEqual<_T: ::dafny_runtime::DafnyTypeEq>(
            a: &::dafny_runtime::Sequence<_T>,
            b: &::dafny_runtime::Sequence<_T>,
            less: &::std::rc::Rc<dyn::std::ops::Fn(&_T, &_T) -> bool>,
        ) -> bool {
            ::dafny_runtime::integer_range(::dafny_runtime::int!(0), a.cardinality() + ::dafny_runtime::int!(1)).any(({
          let mut a = a.clone();
          let mut b = b.clone();
          let mut less = less.clone();
          ::std::rc::Rc::new(move |r#__exists_var_0: ::dafny_runtime::DafnyInt| -> bool{
              let mut k: ::dafny_runtime::DafnyInt = r#__exists_var_0.clone();
              ::dafny_runtime::int!(0) <= k.clone() && k.clone() <= a.cardinality() && crate::r#_StandardLibrary_Compile::_default::LexicographicLessOrEqualAux::<_T>(&a, &b, &less, &k)
            })
        }).as_ref())
        }
        pub fn LexicographicLessOrEqualAux<_T: ::dafny_runtime::DafnyTypeEq>(
            a: &::dafny_runtime::Sequence<_T>,
            b: &::dafny_runtime::Sequence<_T>,
            less: &::std::rc::Rc<dyn::std::ops::Fn(&_T, &_T) -> bool>,
            lengthOfCommonPrefix: &::dafny_runtime::_System::nat,
        ) -> bool {
            lengthOfCommonPrefix.clone() <= b.cardinality()
                && ::dafny_runtime::integer_range(
                    ::dafny_runtime::int!(0),
                    lengthOfCommonPrefix.clone(),
                )
                .all(
                    ({
                        let mut lengthOfCommonPrefix = lengthOfCommonPrefix.clone();
                        let mut a = a.clone();
                        let mut b = b.clone();
                        ::std::rc::Rc::new(
                            move |r#__forall_var_0: ::dafny_runtime::DafnyInt| -> bool {
                                let mut i: ::dafny_runtime::DafnyInt = r#__forall_var_0.clone();
                                !(::dafny_runtime::int!(0) <= i.clone()
                                    && i.clone() < lengthOfCommonPrefix.clone())
                                    || a.get(&i) == b.get(&i)
                            },
                        )
                    })
                    .as_ref(),
                )
                && (lengthOfCommonPrefix.clone() == a.cardinality()
                    || lengthOfCommonPrefix.clone() < b.cardinality()
                        && less(&a.get(lengthOfCommonPrefix), &b.get(lengthOfCommonPrefix)))
        }
        pub fn SetToOrderedSequence<_T: ::dafny_runtime::DafnyTypeEq>(
            s: &::dafny_runtime::Set<::dafny_runtime::Sequence<_T>>,
            less: &::std::rc::Rc<dyn::std::ops::Fn(&_T, &_T) -> bool>,
        ) -> ::dafny_runtime::Sequence<::dafny_runtime::Sequence<_T>> {
            let mut _accumulator: ::dafny_runtime::Sequence<::dafny_runtime::Sequence<_T>> =
                ::dafny_runtime::seq![] as ::dafny_runtime::Sequence<::dafny_runtime::Sequence<_T>>;
            let mut s = s.clone();
            let mut less = less.clone();
            let mut _accumulator = _accumulator.clone();
            'TAIL_CALL_START: loop {
                if s.clone() == ::dafny_runtime::set! {} {
                    return _accumulator.concat(
                        &(::dafny_runtime::seq![]
                            as ::dafny_runtime::Sequence<::dafny_runtime::Sequence<_T>>),
                    );
                } else {
                    return (&({
                        let mut s = s.clone();
                        let mut less = less.clone();
                        ::std::rc::Rc::new(move |r#__let_dummy_0: &::dafny_runtime::DafnyInt| -> ::dafny_runtime::Sequence<::dafny_runtime::Sequence<_T>>{
                  let mut a = ::dafny_runtime::MaybePlacebo::<::dafny_runtime::Sequence<_T>>::new();
                  'label_goto__ASSIGN_SUCH_THAT_0: loop {
                    for r#__assign_such_that_0 in (&s).iter().cloned() {
                      a = ::dafny_runtime::MaybePlacebo::from(r#__assign_such_that_0.clone());
                      if s.contains(&a.read()) && crate::r#_StandardLibrary_Compile::_default::IsMinimum::<_T>(&a.read(), &s, &less) {
                        break 'label_goto__ASSIGN_SUCH_THAT_0;
                      }
                    }
                    panic!("Halt");
                    break;
                  };
                  ::dafny_runtime::seq![a.read()].concat(&crate::r#_StandardLibrary_Compile::_default::SetToOrderedSequence::<_T>(&s.subtract(&::dafny_runtime::set!{a.read()}), &less))
                })
                    }))(&::dafny_runtime::int!(0));
                }
            }
        }
        pub fn IsMinimum<_T: ::dafny_runtime::DafnyTypeEq>(
            a: &::dafny_runtime::Sequence<_T>,
            s: &::dafny_runtime::Set<::dafny_runtime::Sequence<_T>>,
            less: &::std::rc::Rc<dyn::std::ops::Fn(&_T, &_T) -> bool>,
        ) -> bool {
            s.contains(a) && s.iter().all(({
          let mut a = a.clone();
          let mut s = s.clone();
          let mut less = less.clone();
          ::std::rc::Rc::new(move |r#__forall_var_1: &::dafny_runtime::Sequence<_T>| -> bool{
              let mut z: ::dafny_runtime::Sequence<_T> = r#__forall_var_1.clone();
              !s.contains(&z) || crate::r#_StandardLibrary_Compile::_default::LexicographicLessOrEqual::<_T>(&a, &z, &less)
            })
        }).as_ref())
        }
    }

    pub mod r#_UInt_Compile {
        pub use dafny_runtime::DafnyPrint;
        pub use std::default::Default;

        pub struct _default {}

        impl _default {
            pub fn UInt8Less(a: u8, b: u8) -> bool {
                a < b
            }
            pub fn HasUint16Len<_T: ::dafny_runtime::DafnyType>(
                s: &::dafny_runtime::Sequence<_T>,
            ) -> bool {
                s.cardinality()
                    < crate::r#_StandardLibrary_Compile::r#_UInt_Compile::_default::UINT16_LIMIT()
            }
            pub fn HasUint32Len<_T: ::dafny_runtime::DafnyType>(
                s: &::dafny_runtime::Sequence<_T>,
            ) -> bool {
                s.cardinality()
                    < crate::r#_StandardLibrary_Compile::r#_UInt_Compile::_default::UINT32_LIMIT()
            }
            pub fn HasUint64Len<_T: ::dafny_runtime::DafnyType>(
                s: &::dafny_runtime::Sequence<_T>,
            ) -> bool {
                s.cardinality()
                    < crate::r#_StandardLibrary_Compile::r#_UInt_Compile::_default::UINT64_LIMIT()
            }
            pub fn UInt16ToSeq(x: u16) -> ::dafny_runtime::Sequence<u8> {
                let mut b0: u8 = (x / 256) as u8;
                let mut b1: u8 = (x % 256) as u8;
                ::dafny_runtime::seq![b0, b1]
            }
            pub fn SeqToUInt16(s: &::dafny_runtime::Sequence<u8>) -> u16 {
                let mut x0: u16 = s.get(&::dafny_runtime::int!(0)) as u16 * 256;
                x0 + s.get(&::dafny_runtime::int!(1)) as u16
            }
            pub fn UInt32ToSeq(x: u32) -> ::dafny_runtime::Sequence<u8> {
                let mut b0: u8 = (x / 16777216) as u8;
                let mut x0: u32 = x - b0 as u32 * 16777216;
                let mut b1: u8 = (x0 / 65536) as u8;
                let mut x1: u32 = x0 - b1 as u32 * 65536;
                let mut b2: u8 = (x1 / 256) as u8;
                let mut b3: u8 = (x1 % 256) as u8;
                ::dafny_runtime::seq![b0, b1, b2, b3]
            }
            pub fn SeqToUInt32(s: &::dafny_runtime::Sequence<u8>) -> u32 {
                let mut x0: u32 = s.get(&::dafny_runtime::int!(0)) as u32 * 16777216;
                let mut x1: u32 = x0 + s.get(&::dafny_runtime::int!(1)) as u32 * 65536;
                let mut x2: u32 = x1 + s.get(&::dafny_runtime::int!(2)) as u32 * 256;
                x2 + s.get(&::dafny_runtime::int!(3)) as u32
            }
            pub fn UInt64ToSeq(x: u64) -> ::dafny_runtime::Sequence<u8> {
                let mut b0: u8 = (x / 72057594037927936) as u8;
                let mut x0: u64 = x - b0 as u64 * 72057594037927936;
                let mut b1: u8 = (x0 / 281474976710656) as u8;
                let mut x1: u64 = x0 - b1 as u64 * 281474976710656;
                let mut b2: u8 = (x1 / 1099511627776) as u8;
                let mut x2: u64 = x1 - b2 as u64 * 1099511627776;
                let mut b3: u8 = (x2 / 4294967296) as u8;
                let mut x3: u64 = x2 - b3 as u64 * 4294967296;
                let mut b4: u8 = (x3 / 16777216) as u8;
                let mut x4: u64 = x3 - b4 as u64 * 16777216;
                let mut b5: u8 = (x4 / 65536) as u8;
                let mut x5: u64 = x4 - b5 as u64 * 65536;
                let mut b6: u8 = (x5 / 256) as u8;
                let mut b7: u8 = (x5 % 256) as u8;
                ::dafny_runtime::seq![b0, b1, b2, b3, b4, b5, b6, b7]
            }
            pub fn SeqToUInt64(s: &::dafny_runtime::Sequence<u8>) -> u64 {
                let mut x0: u64 = s.get(&::dafny_runtime::int!(0)) as u64 * 72057594037927936;
                let mut x1: u64 = x0 + s.get(&::dafny_runtime::int!(1)) as u64 * 281474976710656;
                let mut x2: u64 = x1 + s.get(&::dafny_runtime::int!(2)) as u64 * 1099511627776;
                let mut x3: u64 = x2 + s.get(&::dafny_runtime::int!(3)) as u64 * 4294967296;
                let mut x4: u64 = x3 + s.get(&::dafny_runtime::int!(4)) as u64 * 16777216;
                let mut x5: u64 = x4 + s.get(&::dafny_runtime::int!(5)) as u64 * 65536;
                let mut x6: u64 = x5 + s.get(&::dafny_runtime::int!(6)) as u64 * 256;
                let mut x: u64 = x6 + s.get(&::dafny_runtime::int!(7)) as u64;
                x
            }
            pub fn UINT16_LIMIT() -> ::dafny_runtime::DafnyInt {
                ::dafny_runtime::int!(b"65536")
            }
            pub fn UINT32_LIMIT() -> ::dafny_runtime::DafnyInt {
                ::dafny_runtime::int!(b"4294967296")
            }
            pub fn UINT64_LIMIT() -> ::dafny_runtime::DafnyInt {
                ::dafny_runtime::int!(b"18446744073709551616")
            }
            pub fn INT32_MAX_LIMIT() -> ::dafny_runtime::DafnyInt {
                ::dafny_runtime::int!(b"2147483648")
            }
            pub fn INT64_MAX_LIMIT() -> ::dafny_runtime::DafnyInt {
                ::dafny_runtime::int!(b"9223372036854775808")
            }
        }

        #[derive(Clone, PartialEq)]
        #[repr(transparent)]
        pub struct uint8(pub u8);

        impl uint8 {
            pub fn is(_source: u8) -> bool {
                return true;
            }
        }

        impl Default for uint8 {
            fn default() -> Self {
                uint8(::std::default::Default::default())
            }
        }

        impl DafnyPrint for uint8 {
            fn fmt_print(
                &self,
                _formatter: &mut ::std::fmt::Formatter,
                in_seq: bool,
            ) -> ::std::fmt::Result {
                ::dafny_runtime::DafnyPrint::fmt_print(&self.0, _formatter, in_seq)
            }
        }

        impl ::std::ops::Deref for uint8 {
            type Target = u8;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        #[derive(Clone, PartialEq)]
        #[repr(transparent)]
        pub struct uint16(pub u16);

        impl uint16 {
            pub fn is(_source: u16) -> bool {
                return true;
            }
        }

        impl Default for uint16 {
            fn default() -> Self {
                uint16(::std::default::Default::default())
            }
        }

        impl DafnyPrint for uint16 {
            fn fmt_print(
                &self,
                _formatter: &mut ::std::fmt::Formatter,
                in_seq: bool,
            ) -> ::std::fmt::Result {
                ::dafny_runtime::DafnyPrint::fmt_print(&self.0, _formatter, in_seq)
            }
        }

        impl ::std::ops::Deref for uint16 {
            type Target = u16;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        #[derive(Clone, PartialEq)]
        #[repr(transparent)]
        pub struct uint32(pub u32);

        impl uint32 {
            pub fn is(_source: u32) -> bool {
                return true;
            }
        }

        impl Default for uint32 {
            fn default() -> Self {
                uint32(::std::default::Default::default())
            }
        }

        impl DafnyPrint for uint32 {
            fn fmt_print(
                &self,
                _formatter: &mut ::std::fmt::Formatter,
                in_seq: bool,
            ) -> ::std::fmt::Result {
                ::dafny_runtime::DafnyPrint::fmt_print(&self.0, _formatter, in_seq)
            }
        }

        impl ::std::ops::Deref for uint32 {
            type Target = u32;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        #[derive(Clone, PartialEq)]
        #[repr(transparent)]
        pub struct uint64(pub u64);

        impl uint64 {
            pub fn is(_source: u64) -> bool {
                return true;
            }
        }

        impl Default for uint64 {
            fn default() -> Self {
                uint64(::std::default::Default::default())
            }
        }

        impl DafnyPrint for uint64 {
            fn fmt_print(
                &self,
                _formatter: &mut ::std::fmt::Formatter,
                in_seq: bool,
            ) -> ::std::fmt::Result {
                ::dafny_runtime::DafnyPrint::fmt_print(&self.0, _formatter, in_seq)
            }
        }

        impl ::std::ops::Deref for uint64 {
            type Target = u64;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        #[derive(Clone, PartialEq)]
        #[repr(transparent)]
        pub struct int32(pub i32);

        impl int32 {
            pub fn is(_source: i32) -> bool {
                return true;
            }
        }

        impl Default for int32 {
            fn default() -> Self {
                int32(::std::default::Default::default())
            }
        }

        impl DafnyPrint for int32 {
            fn fmt_print(
                &self,
                _formatter: &mut ::std::fmt::Formatter,
                in_seq: bool,
            ) -> ::std::fmt::Result {
                ::dafny_runtime::DafnyPrint::fmt_print(&self.0, _formatter, in_seq)
            }
        }

        impl ::std::ops::Deref for int32 {
            type Target = i32;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        #[derive(Clone, PartialEq)]
        #[repr(transparent)]
        pub struct int64(pub i64);

        impl int64 {
            pub fn is(_source: i64) -> bool {
                return true;
            }
        }

        impl Default for int64 {
            fn default() -> Self {
                int64(::std::default::Default::default())
            }
        }

        impl DafnyPrint for int64 {
            fn fmt_print(
                &self,
                _formatter: &mut ::std::fmt::Formatter,
                in_seq: bool,
            ) -> ::std::fmt::Result {
                ::dafny_runtime::DafnyPrint::fmt_print(&self.0, _formatter, in_seq)
            }
        }

        impl ::std::ops::Deref for int64 {
            type Target = i64;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        #[derive(Clone, PartialEq)]
        #[repr(transparent)]
        pub struct posInt64(pub u64);

        impl posInt64 {
            pub fn is(_source: u64) -> bool {
                let mut x: ::dafny_runtime::DafnyInt =
                    ::std::convert::Into::<::dafny_runtime::DafnyInt>::into(_source.clone());
                return ::dafny_runtime::int!(0) < x.clone()
                    && x.clone() < ::dafny_runtime::int!(b"9223372036854775808");
            }
        }

        impl Default for posInt64 {
            fn default() -> Self {
                posInt64(1)
            }
        }

        impl DafnyPrint for posInt64 {
            fn fmt_print(
                &self,
                _formatter: &mut ::std::fmt::Formatter,
                in_seq: bool,
            ) -> ::std::fmt::Result {
                ::dafny_runtime::DafnyPrint::fmt_print(&self.0, _formatter, in_seq)
            }
        }

        impl ::std::ops::Deref for posInt64 {
            type Target = u64;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        pub type seq16<T: ::dafny_runtime::DafnyType> = ::dafny_runtime::Sequence<T>;

        pub type seq32<T: ::dafny_runtime::DafnyType> = ::dafny_runtime::Sequence<T>;

        pub type seq64<T: ::dafny_runtime::DafnyType> = ::dafny_runtime::Sequence<T>;
    }
}
pub mod UTF8 {
    pub struct _default {}

    impl _default {
        pub fn CreateEncodeSuccess(
            bytes: &crate::UTF8::ValidUTF8Bytes,
        ) -> ::std::rc::Rc<
            crate::r#_Wrappers_Compile::Result<
                crate::UTF8::ValidUTF8Bytes,
                ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
            >,
        > {
            ::std::rc::Rc::new(crate::r#_Wrappers_Compile::Result::<
                crate::UTF8::ValidUTF8Bytes,
                ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
            >::Success {
                value: bytes.clone(),
            })
        }
        pub fn CreateEncodeFailure(
            error: &::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
        ) -> ::std::rc::Rc<
            crate::r#_Wrappers_Compile::Result<
                crate::UTF8::ValidUTF8Bytes,
                ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
            >,
        > {
            ::std::rc::Rc::new(crate::r#_Wrappers_Compile::Result::<
                crate::UTF8::ValidUTF8Bytes,
                ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
            >::Failure {
                error: error.clone(),
            })
        }
        pub fn CreateDecodeSuccess(
            s: &::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
        ) -> ::std::rc::Rc<
            crate::r#_Wrappers_Compile::Result<
                ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
            >,
        > {
            ::std::rc::Rc::new(crate::r#_Wrappers_Compile::Result::<
                ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
            >::Success {
                value: s.clone(),
            })
        }
        pub fn CreateDecodeFailure(
            error: &::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
        ) -> ::std::rc::Rc<
            crate::r#_Wrappers_Compile::Result<
                ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
            >,
        > {
            ::std::rc::Rc::new(crate::r#_Wrappers_Compile::Result::<
                ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
            >::Failure {
                error: error.clone(),
            })
        }
        pub fn IsASCIIString(
            s: &::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
        ) -> bool {
            let mut _hresult: bool = <bool as std::default::Default>::default();
            let mut _hi0: ::dafny_runtime::DafnyInt = s.cardinality();
            for i in ::dafny_runtime::integer_range(::dafny_runtime::int!(0), _hi0.clone()) {
                if !(::dafny_runtime::int!(s.get(&i).0) < ::dafny_runtime::int!(128)) {
                    _hresult = false;
                    return _hresult;
                }
            }
            _hresult = true;
            return _hresult;
        }
        pub fn EncodeAscii(
            s: &::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
        ) -> crate::UTF8::ValidUTF8Bytes {
            let mut _accumulator: crate::UTF8::ValidUTF8Bytes =
                ::dafny_runtime::seq![] as ::dafny_runtime::Sequence<u8>;
            let mut s = s.clone();
            let mut _accumulator = _accumulator.clone();
            'TAIL_CALL_START: loop {
                if s.cardinality() == ::dafny_runtime::int!(0) {
                    return _accumulator
                        .concat(&(::dafny_runtime::seq![] as ::dafny_runtime::Sequence<u8>));
                } else {
                    let mut x: ::dafny_runtime::Sequence<u8> =
                        ::dafny_runtime::seq![s.get(&::dafny_runtime::int!(0)).0 as u8];
                    _accumulator = _accumulator.concat(&x);
                    let mut _in11: ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16> =
                        s.drop(&::dafny_runtime::int!(1));
                    s = _in11.clone();
                    continue 'TAIL_CALL_START;
                }
            }
        }
        pub fn Uses1Byte(s: &::dafny_runtime::Sequence<u8>) -> bool {
            0 <= s.get(&::dafny_runtime::int!(0)) && s.get(&::dafny_runtime::int!(0)) <= 127
        }
        pub fn Uses2Bytes(s: &::dafny_runtime::Sequence<u8>) -> bool {
            194 <= s.get(&::dafny_runtime::int!(0))
                && s.get(&::dafny_runtime::int!(0)) <= 223
                && (128 <= s.get(&::dafny_runtime::int!(1))
                    && s.get(&::dafny_runtime::int!(1)) <= 191)
        }
        pub fn Uses3Bytes(s: &::dafny_runtime::Sequence<u8>) -> bool {
            s.get(&::dafny_runtime::int!(0)) == 224
                && (160 <= s.get(&::dafny_runtime::int!(1))
                    && s.get(&::dafny_runtime::int!(1)) <= 191)
                && (128 <= s.get(&::dafny_runtime::int!(2))
                    && s.get(&::dafny_runtime::int!(2)) <= 191)
                || 225 <= s.get(&::dafny_runtime::int!(0))
                    && s.get(&::dafny_runtime::int!(0)) <= 236
                    && (128 <= s.get(&::dafny_runtime::int!(1))
                        && s.get(&::dafny_runtime::int!(1)) <= 191)
                    && (128 <= s.get(&::dafny_runtime::int!(2))
                        && s.get(&::dafny_runtime::int!(2)) <= 191)
                || s.get(&::dafny_runtime::int!(0)) == 237
                    && (128 <= s.get(&::dafny_runtime::int!(1))
                        && s.get(&::dafny_runtime::int!(1)) <= 159)
                    && (128 <= s.get(&::dafny_runtime::int!(2))
                        && s.get(&::dafny_runtime::int!(2)) <= 191)
                || 238 <= s.get(&::dafny_runtime::int!(0))
                    && s.get(&::dafny_runtime::int!(0)) <= 239
                    && (128 <= s.get(&::dafny_runtime::int!(1))
                        && s.get(&::dafny_runtime::int!(1)) <= 191)
                    && (128 <= s.get(&::dafny_runtime::int!(2))
                        && s.get(&::dafny_runtime::int!(2)) <= 191)
        }
        pub fn Uses4Bytes(s: &::dafny_runtime::Sequence<u8>) -> bool {
            s.get(&::dafny_runtime::int!(0)) == 240
                && (144 <= s.get(&::dafny_runtime::int!(1))
                    && s.get(&::dafny_runtime::int!(1)) <= 191)
                && (128 <= s.get(&::dafny_runtime::int!(2))
                    && s.get(&::dafny_runtime::int!(2)) <= 191)
                && (128 <= s.get(&::dafny_runtime::int!(3))
                    && s.get(&::dafny_runtime::int!(3)) <= 191)
                || 241 <= s.get(&::dafny_runtime::int!(0))
                    && s.get(&::dafny_runtime::int!(0)) <= 243
                    && (128 <= s.get(&::dafny_runtime::int!(1))
                        && s.get(&::dafny_runtime::int!(1)) <= 191)
                    && (128 <= s.get(&::dafny_runtime::int!(2))
                        && s.get(&::dafny_runtime::int!(2)) <= 191)
                    && (128 <= s.get(&::dafny_runtime::int!(3))
                        && s.get(&::dafny_runtime::int!(3)) <= 191)
                || s.get(&::dafny_runtime::int!(0)) == 244
                    && (128 <= s.get(&::dafny_runtime::int!(1))
                        && s.get(&::dafny_runtime::int!(1)) <= 143)
                    && (128 <= s.get(&::dafny_runtime::int!(2))
                        && s.get(&::dafny_runtime::int!(2)) <= 191)
                    && (128 <= s.get(&::dafny_runtime::int!(3))
                        && s.get(&::dafny_runtime::int!(3)) <= 191)
        }
        pub fn ValidUTF8Range(
            a: &::dafny_runtime::Sequence<u8>,
            lo: &::dafny_runtime::_System::nat,
            hi: &::dafny_runtime::_System::nat,
        ) -> bool {
            let mut a = a.clone();
            let mut lo = lo.clone();
            let mut hi = hi.clone();
            'TAIL_CALL_START: loop {
                if lo.clone() == hi.clone() {
                    return true;
                } else {
                    let mut r: ::dafny_runtime::Sequence<u8> = a.slice(&lo, &hi);
                    if crate::UTF8::_default::Uses1Byte(&r) {
                        let mut _in12: ::dafny_runtime::Sequence<u8> = a.clone();
                        let mut _in13: ::dafny_runtime::DafnyInt =
                            lo.clone() + ::dafny_runtime::int!(1);
                        let mut _in14: ::dafny_runtime::_System::nat = hi.clone();
                        a = _in12.clone();
                        lo = _in13.clone();
                        hi = _in14.clone();
                        continue 'TAIL_CALL_START;
                    } else {
                        if ::dafny_runtime::int!(2) <= r.cardinality()
                            && crate::UTF8::_default::Uses2Bytes(&r)
                        {
                            let mut _in15: ::dafny_runtime::Sequence<u8> = a.clone();
                            let mut _in16: ::dafny_runtime::DafnyInt =
                                lo.clone() + ::dafny_runtime::int!(2);
                            let mut _in17: ::dafny_runtime::_System::nat = hi.clone();
                            a = _in15.clone();
                            lo = _in16.clone();
                            hi = _in17.clone();
                            continue 'TAIL_CALL_START;
                        } else {
                            if ::dafny_runtime::int!(3) <= r.cardinality()
                                && crate::UTF8::_default::Uses3Bytes(&r)
                            {
                                let mut _in18: ::dafny_runtime::Sequence<u8> = a.clone();
                                let mut _in19: ::dafny_runtime::DafnyInt =
                                    lo.clone() + ::dafny_runtime::int!(3);
                                let mut _in20: ::dafny_runtime::_System::nat = hi.clone();
                                a = _in18.clone();
                                lo = _in19.clone();
                                hi = _in20.clone();
                                continue 'TAIL_CALL_START;
                            } else {
                                if ::dafny_runtime::int!(4) <= r.cardinality()
                                    && crate::UTF8::_default::Uses4Bytes(&r)
                                {
                                    let mut _in21: ::dafny_runtime::Sequence<u8> = a.clone();
                                    let mut _in22: ::dafny_runtime::DafnyInt =
                                        lo.clone() + ::dafny_runtime::int!(4);
                                    let mut _in23: ::dafny_runtime::_System::nat = hi.clone();
                                    a = _in21.clone();
                                    lo = _in22.clone();
                                    hi = _in23.clone();
                                    continue 'TAIL_CALL_START;
                                } else {
                                    return false;
                                }
                            }
                        }
                    }
                }
            }
        }
        pub fn ValidUTF8Seq(s: &::dafny_runtime::Sequence<u8>) -> bool {
            crate::UTF8::_default::ValidUTF8Range(s, &::dafny_runtime::int!(0), &s.cardinality())
        }
    }

    pub type ValidUTF8Bytes = ::dafny_runtime::Sequence<u8>;

    pub fn r#__init_ValidUTF8Bytes() -> ::dafny_runtime::Sequence<u8> {
        ::dafny_runtime::seq![] as ::dafny_runtime::Sequence<u8>
    }
}
pub mod software {
    pub mod amazon {
        pub mod cryptography {
            pub mod services {
                pub mod dynamodb {
                    pub mod internaldafny {
                        pub use dafny_runtime::DafnyPrint;
                        pub use std::cmp::Eq;
                        pub use std::convert::AsRef;
                        pub use std::default::Default;
                        pub use std::fmt::Debug;
                        pub use std::hash::Hash;

                        pub struct _default {}

                        impl _default {
                            pub fn DefaultDynamoDBClientConfigType() -> ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::DynamoDBClientConfigType>{
                                ::std::rc::Rc::new(crate::software::amazon::cryptography::services::dynamodb::internaldafny::DynamoDBClientConfigType::DynamoDBClientConfigType {})
                            }
                            pub fn CreateSuccessOfClient(client: &::dafny_runtime::Object<dyn crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::IDynamoDBClient>) -> ::std::rc::Rc<crate::r#_Wrappers_Compile::Result<::dafny_runtime::Object<dyn crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::IDynamoDBClient>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>>>{
                                ::std::rc::Rc::new(crate::r#_Wrappers_Compile::Result::<::dafny_runtime::Object<dyn crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::IDynamoDBClient>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>>::Success {
                    value: client.clone()
                  })
                            }
                            pub fn CreateFailureOfError(error: &::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>) -> ::std::rc::Rc<crate::r#_Wrappers_Compile::Result<::dafny_runtime::Object<dyn crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::IDynamoDBClient>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>>>{
                                ::std::rc::Rc::new(crate::r#_Wrappers_Compile::Result::<::dafny_runtime::Object<dyn crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::IDynamoDBClient>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>>::Failure {
                    error: error.clone()
                  })
                            }
                        }

                        #[derive(PartialEq, Clone)]
                        pub enum DynamoDBClientConfigType {
                            DynamoDBClientConfigType {},
                        }

                        impl DynamoDBClientConfigType {}

                        impl Debug for DynamoDBClientConfigType {
                            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                            }
                        }

                        impl DafnyPrint for DynamoDBClientConfigType {
                            fn fmt_print(
                                &self,
                                _formatter: &mut ::std::fmt::Formatter,
                                _in_seq: bool,
                            ) -> std::fmt::Result {
                                match self {
                                    DynamoDBClientConfigType::DynamoDBClientConfigType {} => {
                                        write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.DynamoDBClientConfigType.DynamoDBClientConfigType")?;
                                        Ok(())
                                    }
                                }
                            }
                        }

                        impl Eq for DynamoDBClientConfigType {}

                        impl Hash for DynamoDBClientConfigType {
                            fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                match self {
                                    DynamoDBClientConfigType::DynamoDBClientConfigType {} => {}
                                }
                            }
                        }

                        impl Default for DynamoDBClientConfigType {
                            fn default() -> DynamoDBClientConfigType {
                                DynamoDBClientConfigType::DynamoDBClientConfigType {}
                            }
                        }

                        impl AsRef<DynamoDBClientConfigType> for &DynamoDBClientConfigType {
                            fn as_ref(&self) -> Self {
                                self
                            }
                        }

                        pub mod types {
                            pub use dafny_runtime::DafnyPrint;
                            pub use dafny_runtime::UpcastObject;
                            pub use std::any::Any;
                            pub use std::cmp::Eq;
                            pub use std::convert::AsRef;
                            pub use std::default::Default;
                            pub use std::fmt::Debug;
                            pub use std::hash::Hash;

                            pub struct _default {}

                            impl _default {
                                pub fn IsValid_AttributeName(
                                    x: &::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                ) -> bool {
                                    ::dafny_runtime::int!(0) <= x.cardinality()
                                        && x.cardinality() <= ::dafny_runtime::int!(b"65535")
                                }
                                pub fn IsValid_AttributeNameList(
                                    x: &::dafny_runtime::Sequence<
                                        ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                    >,
                                ) -> bool {
                                    ::dafny_runtime::int!(1) <= x.cardinality()
                                }
                                pub fn IsValid_BackupArn(
                                    x: &::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                ) -> bool {
                                    ::dafny_runtime::int!(37) <= x.cardinality()
                                        && x.cardinality() <= ::dafny_runtime::int!(1024)
                                }
                                pub fn IsValid_BatchGetRequestMap(
                                    x: &::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::KeysAndAttributes>>,
                                ) -> bool {
                                    ::dafny_runtime::int!(1) <= x.cardinality()
                                        && x.cardinality() <= ::dafny_runtime::int!(100)
                                }
                                pub fn IsValid_CancellationReasonList(
                                    x: &::dafny_runtime::Sequence<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::CancellationReason>>,
                                ) -> bool {
                                    ::dafny_runtime::int!(1) <= x.cardinality()
                                        && x.cardinality() <= ::dafny_runtime::int!(25)
                                }
                                pub fn IsValid_ClientRequestToken(
                                    x: &::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                ) -> bool {
                                    ::dafny_runtime::int!(1) <= x.cardinality()
                                        && x.cardinality() <= ::dafny_runtime::int!(36)
                                }
                                pub fn IsValid_ConsumedCapacityUnits(
                                    x: &::dafny_runtime::Sequence<u8>,
                                ) -> bool {
                                    ::dafny_runtime::int!(8) <= x.cardinality()
                                        && x.cardinality() <= ::dafny_runtime::int!(8)
                                }
                                pub fn IsValid_IndexName(
                                    x: &::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                ) -> bool {
                                    ::dafny_runtime::int!(3) <= x.cardinality()
                                        && x.cardinality() <= ::dafny_runtime::int!(255)
                                }
                                pub fn IsValid_ItemCollectionSizeEstimateBound(
                                    x: &::dafny_runtime::Sequence<u8>,
                                ) -> bool {
                                    ::dafny_runtime::int!(8) <= x.cardinality()
                                        && x.cardinality() <= ::dafny_runtime::int!(8)
                                }
                                pub fn IsValid_KeyList(
                                    x: &::dafny_runtime::Sequence<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>>,
                                ) -> bool {
                                    ::dafny_runtime::int!(1) <= x.cardinality()
                                        && x.cardinality() <= ::dafny_runtime::int!(100)
                                }
                                pub fn IsValid_KeySchema(
                                    x: &::dafny_runtime::Sequence<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::KeySchemaElement>>,
                                ) -> bool {
                                    ::dafny_runtime::int!(1) <= x.cardinality()
                                        && x.cardinality() <= ::dafny_runtime::int!(2)
                                }
                                pub fn IsValid_KeySchemaAttributeName(
                                    x: &::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                ) -> bool {
                                    ::dafny_runtime::int!(1) <= x.cardinality()
                                        && x.cardinality() <= ::dafny_runtime::int!(255)
                                }
                                pub fn IsValid_NonKeyAttributeName(
                                    x: &::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                ) -> bool {
                                    ::dafny_runtime::int!(1) <= x.cardinality()
                                        && x.cardinality() <= ::dafny_runtime::int!(255)
                                }
                                pub fn IsValid_NonKeyAttributeNameList(
                                    x: &::dafny_runtime::Sequence<
                                        ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                    >,
                                ) -> bool {
                                    ::dafny_runtime::int!(1) <= x.cardinality()
                                        && x.cardinality() <= ::dafny_runtime::int!(20)
                                }
                                pub fn IsValid_NonNegativeLongObject(x: i64) -> bool {
                                    0 <= x
                                }
                                pub fn IsValid_PositiveIntegerObject(x: i32) -> bool {
                                    1 <= x
                                }
                                pub fn IsValid_PositiveLongObject(x: i64) -> bool {
                                    1 <= x
                                }
                                pub fn IsValid_ScanSegment(x: i32) -> bool {
                                    0 <= x && x <= 999999
                                }
                                pub fn IsValid_ScanTotalSegments(x: i32) -> bool {
                                    1 <= x && x <= 1000000
                                }
                                pub fn IsValid_StreamArn(
                                    x: &::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                ) -> bool {
                                    ::dafny_runtime::int!(37) <= x.cardinality()
                                        && x.cardinality() <= ::dafny_runtime::int!(1024)
                                }
                                pub fn IsValid_TableName(
                                    x: &::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                ) -> bool {
                                    ::dafny_runtime::int!(3) <= x.cardinality()
                                        && x.cardinality() <= ::dafny_runtime::int!(255)
                                }
                                pub fn IsValid_TagKeyString(
                                    x: &::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                ) -> bool {
                                    ::dafny_runtime::int!(1) <= x.cardinality()
                                        && x.cardinality() <= ::dafny_runtime::int!(128)
                                }
                                pub fn IsValid_TagValueString(
                                    x: &::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                ) -> bool {
                                    ::dafny_runtime::int!(0) <= x.cardinality()
                                        && x.cardinality() <= ::dafny_runtime::int!(256)
                                }
                                pub fn IsValid_TransactWriteItemList(
                                    x: &::dafny_runtime::Sequence<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::TransactWriteItem>>,
                                ) -> bool {
                                    ::dafny_runtime::int!(1) <= x.cardinality()
                                        && x.cardinality() <= ::dafny_runtime::int!(25)
                                }
                            }

                            #[derive(PartialEq, Clone)]
                            pub enum DafnyCallEvent<
                                I: ::dafny_runtime::DafnyType,
                                O: ::dafny_runtime::DafnyType,
                            > {
                                DafnyCallEvent { input: I, output: O },
                            }

                            impl<I: ::dafny_runtime::DafnyType, O: ::dafny_runtime::DafnyType> DafnyCallEvent<I, O> {
                                pub fn input(&self) -> &I {
                                    match self {
                                        DafnyCallEvent::DafnyCallEvent { input, output } => input,
                                    }
                                }
                                pub fn output(&self) -> &O {
                                    match self {
                                        DafnyCallEvent::DafnyCallEvent { input, output } => output,
                                    }
                                }
                            }

                            impl<I: ::dafny_runtime::DafnyType, O: ::dafny_runtime::DafnyType> Debug for DafnyCallEvent<I, O> {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl<I: ::dafny_runtime::DafnyType, O: ::dafny_runtime::DafnyType>
                                DafnyPrint for DafnyCallEvent<I, O>
                            {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        DafnyCallEvent::DafnyCallEvent { input, output } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.DafnyCallEvent.DafnyCallEvent(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                input, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                output, _formatter, false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl<
                                    I: ::dafny_runtime::DafnyType + Eq,
                                    O: ::dafny_runtime::DafnyType + Eq,
                                > Eq for DafnyCallEvent<I, O>
                            {
                            }

                            impl<
                                    I: ::dafny_runtime::DafnyType + Hash,
                                    O: ::dafny_runtime::DafnyType + Hash,
                                > Hash for DafnyCallEvent<I, O>
                            {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        DafnyCallEvent::DafnyCallEvent { input, output } => {
                                            ::std::hash::Hash::hash(input, _state);
                                            ::std::hash::Hash::hash(output, _state)
                                        }
                                    }
                                }
                            }

                            impl<
                                    I: ::dafny_runtime::DafnyType + Default,
                                    O: ::dafny_runtime::DafnyType + Default,
                                > Default for DafnyCallEvent<I, O>
                            {
                                fn default() -> DafnyCallEvent<I, O> {
                                    DafnyCallEvent::DafnyCallEvent {
                                        input: ::std::default::Default::default(),
                                        output: ::std::default::Default::default(),
                                    }
                                }
                            }

                            impl<I: ::dafny_runtime::DafnyType, O: ::dafny_runtime::DafnyType>
                                AsRef<DafnyCallEvent<I, O>> for &DafnyCallEvent<I, O>
                            {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            #[derive(PartialEq, Clone)]
                            pub enum ArchivalSummary {
                                ArchivalSummary {
                                    ArchivalDateTime: ::std::rc::Rc<
                                        crate::r#_Wrappers_Compile::Option<
                                            ::dafny_runtime::Sequence<
                                                ::dafny_runtime::DafnyCharUTF16,
                                            >,
                                        >,
                                    >,
                                    ArchivalReason: ::std::rc::Rc<
                                        crate::r#_Wrappers_Compile::Option<
                                            ::dafny_runtime::Sequence<
                                                ::dafny_runtime::DafnyCharUTF16,
                                            >,
                                        >,
                                    >,
                                    ArchivalBackupArn: ::std::rc::Rc<
                                        crate::r#_Wrappers_Compile::Option<
                                            ::dafny_runtime::Sequence<
                                                ::dafny_runtime::DafnyCharUTF16,
                                            >,
                                        >,
                                    >,
                                },
                            }

                            impl ArchivalSummary {
                                pub fn ArchivalDateTime(
                                    &self,
                                ) -> &::std::rc::Rc<
                                    crate::r#_Wrappers_Compile::Option<
                                        ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                    >,
                                > {
                                    match self {
                                        ArchivalSummary::ArchivalSummary {
                                            ArchivalDateTime,
                                            ArchivalReason,
                                            ArchivalBackupArn,
                                        } => ArchivalDateTime,
                                    }
                                }
                                pub fn ArchivalReason(
                                    &self,
                                ) -> &::std::rc::Rc<
                                    crate::r#_Wrappers_Compile::Option<
                                        ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                    >,
                                > {
                                    match self {
                                        ArchivalSummary::ArchivalSummary {
                                            ArchivalDateTime,
                                            ArchivalReason,
                                            ArchivalBackupArn,
                                        } => ArchivalReason,
                                    }
                                }
                                pub fn ArchivalBackupArn(
                                    &self,
                                ) -> &::std::rc::Rc<
                                    crate::r#_Wrappers_Compile::Option<
                                        ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                    >,
                                > {
                                    match self {
                                        ArchivalSummary::ArchivalSummary {
                                            ArchivalDateTime,
                                            ArchivalReason,
                                            ArchivalBackupArn,
                                        } => ArchivalBackupArn,
                                    }
                                }
                            }

                            impl Debug for ArchivalSummary {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for ArchivalSummary {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        ArchivalSummary::ArchivalSummary {
                                            ArchivalDateTime,
                                            ArchivalReason,
                                            ArchivalBackupArn,
                                        } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.ArchivalSummary.ArchivalSummary(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ArchivalDateTime,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ArchivalReason,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ArchivalBackupArn,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for ArchivalSummary {}

                            impl Hash for ArchivalSummary {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        ArchivalSummary::ArchivalSummary {
                                            ArchivalDateTime,
                                            ArchivalReason,
                                            ArchivalBackupArn,
                                        } => {
                                            ::std::hash::Hash::hash(ArchivalDateTime, _state);
                                            ::std::hash::Hash::hash(ArchivalReason, _state);
                                            ::std::hash::Hash::hash(ArchivalBackupArn, _state)
                                        }
                                    }
                                }
                            }

                            impl Default for ArchivalSummary {
                                fn default() -> ArchivalSummary {
                                    ArchivalSummary::ArchivalSummary {
                                        ArchivalDateTime: ::std::default::Default::default(),
                                        ArchivalReason: ::std::default::Default::default(),
                                        ArchivalBackupArn: ::std::default::Default::default(),
                                    }
                                }
                            }

                            impl AsRef<ArchivalSummary> for &ArchivalSummary {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            #[derive(PartialEq, Clone)]
                            pub enum AttributeAction {
                                ADD {},
                                PUT {},
                                DELETE {},
                            }

                            impl AttributeAction {}

                            impl Debug for AttributeAction {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for AttributeAction {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        AttributeAction::ADD {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.AttributeAction.ADD")?;
                                            Ok(())
                                        }
                                        AttributeAction::PUT {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.AttributeAction.PUT")?;
                                            Ok(())
                                        }
                                        AttributeAction::DELETE {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.AttributeAction.DELETE")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for AttributeAction {}

                            impl Hash for AttributeAction {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        AttributeAction::ADD {} => {}
                                        AttributeAction::PUT {} => {}
                                        AttributeAction::DELETE {} => {}
                                    }
                                }
                            }

                            impl Default for AttributeAction {
                                fn default() -> AttributeAction {
                                    AttributeAction::ADD {}
                                }
                            }

                            impl AsRef<AttributeAction> for &AttributeAction {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            #[derive(PartialEq, Clone)]
                            pub enum AttributeDefinition {
                                AttributeDefinition {
                  AttributeName: ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                  AttributeType: ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ScalarAttributeType>
                }
              }

                            impl AttributeDefinition {
                                pub fn AttributeName(
                                    &self,
                                ) -> &::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>
                                {
                                    match self {
                                        AttributeDefinition::AttributeDefinition {
                                            AttributeName,
                                            AttributeType,
                                        } => AttributeName,
                                    }
                                }
                                pub fn AttributeType(&self) -> &::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ScalarAttributeType>{
                                    match self {
                                        AttributeDefinition::AttributeDefinition {
                                            AttributeName,
                                            AttributeType,
                                        } => AttributeType,
                                    }
                                }
                            }

                            impl Debug for AttributeDefinition {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for AttributeDefinition {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        AttributeDefinition::AttributeDefinition {
                                            AttributeName,
                                            AttributeType,
                                        } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.AttributeDefinition.AttributeDefinition(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                AttributeName,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                AttributeType,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for AttributeDefinition {}

                            impl Hash for AttributeDefinition {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        AttributeDefinition::AttributeDefinition {
                                            AttributeName,
                                            AttributeType,
                                        } => {
                                            ::std::hash::Hash::hash(AttributeName, _state);
                                            ::std::hash::Hash::hash(AttributeType, _state)
                                        }
                                    }
                                }
                            }

                            impl Default for AttributeDefinition {
                                fn default() -> AttributeDefinition {
                                    AttributeDefinition::AttributeDefinition {
                                        AttributeName: ::std::default::Default::default(),
                                        AttributeType: ::std::default::Default::default(),
                                    }
                                }
                            }

                            impl AsRef<AttributeDefinition> for &AttributeDefinition {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            pub type AttributeName =
                                ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>;

                            pub type AttributeNameList = ::dafny_runtime::Sequence<
                                ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                            >;

                            #[derive(PartialEq, Clone)]
                            pub enum AttributeValue {
                                S {
                  S: ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>
                },
                N {
                  N: ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>
                },
                B {
                  B: ::dafny_runtime::Sequence<u8>
                },
                SS {
                  SS: ::dafny_runtime::Sequence<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>
                },
                NS {
                  NS: ::dafny_runtime::Sequence<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>
                },
                BS {
                  BS: ::dafny_runtime::Sequence<::dafny_runtime::Sequence<u8>>
                },
                M {
                  M: ::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>
                },
                L {
                  L: ::dafny_runtime::Sequence<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>
                },
                NULL {
                  NULL: bool
                },
                BOOL {
                  BOOL: bool
                }
              }

                            impl AttributeValue {
                                pub fn S(
                                    &self,
                                ) -> &::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>
                                {
                                    match self {
                                        AttributeValue::S { S } => S,
                                        AttributeValue::N { N } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::B { B } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::SS { SS } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::NS { NS } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::BS { BS } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::M { M } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::L { L } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::NULL { NULL } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::BOOL { BOOL } => {
                                            panic!("field does not exist on this variant")
                                        }
                                    }
                                }
                                pub fn N(
                                    &self,
                                ) -> &::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>
                                {
                                    match self {
                                        AttributeValue::S { S } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::N { N } => N,
                                        AttributeValue::B { B } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::SS { SS } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::NS { NS } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::BS { BS } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::M { M } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::L { L } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::NULL { NULL } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::BOOL { BOOL } => {
                                            panic!("field does not exist on this variant")
                                        }
                                    }
                                }
                                pub fn B(&self) -> &::dafny_runtime::Sequence<u8> {
                                    match self {
                                        AttributeValue::S { S } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::N { N } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::B { B } => B,
                                        AttributeValue::SS { SS } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::NS { NS } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::BS { BS } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::M { M } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::L { L } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::NULL { NULL } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::BOOL { BOOL } => {
                                            panic!("field does not exist on this variant")
                                        }
                                    }
                                }
                                pub fn SS(
                                    &self,
                                ) -> &::dafny_runtime::Sequence<
                                    ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                > {
                                    match self {
                                        AttributeValue::S { S } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::N { N } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::B { B } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::SS { SS } => SS,
                                        AttributeValue::NS { NS } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::BS { BS } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::M { M } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::L { L } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::NULL { NULL } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::BOOL { BOOL } => {
                                            panic!("field does not exist on this variant")
                                        }
                                    }
                                }
                                pub fn NS(
                                    &self,
                                ) -> &::dafny_runtime::Sequence<
                                    ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                > {
                                    match self {
                                        AttributeValue::S { S } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::N { N } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::B { B } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::SS { SS } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::NS { NS } => NS,
                                        AttributeValue::BS { BS } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::M { M } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::L { L } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::NULL { NULL } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::BOOL { BOOL } => {
                                            panic!("field does not exist on this variant")
                                        }
                                    }
                                }
                                pub fn BS(
                                    &self,
                                ) -> &::dafny_runtime::Sequence<::dafny_runtime::Sequence<u8>>
                                {
                                    match self {
                                        AttributeValue::S { S } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::N { N } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::B { B } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::SS { SS } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::NS { NS } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::BS { BS } => BS,
                                        AttributeValue::M { M } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::L { L } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::NULL { NULL } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::BOOL { BOOL } => {
                                            panic!("field does not exist on this variant")
                                        }
                                    }
                                }
                                pub fn M(&self) -> &::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>{
                                    match self {
                                        AttributeValue::S { S } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::N { N } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::B { B } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::SS { SS } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::NS { NS } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::BS { BS } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::M { M } => M,
                                        AttributeValue::L { L } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::NULL { NULL } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::BOOL { BOOL } => {
                                            panic!("field does not exist on this variant")
                                        }
                                    }
                                }
                                pub fn L(&self) -> &::dafny_runtime::Sequence<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>{
                                    match self {
                                        AttributeValue::S { S } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::N { N } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::B { B } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::SS { SS } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::NS { NS } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::BS { BS } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::M { M } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::L { L } => L,
                                        AttributeValue::NULL { NULL } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::BOOL { BOOL } => {
                                            panic!("field does not exist on this variant")
                                        }
                                    }
                                }
                                pub fn NULL(&self) -> &bool {
                                    match self {
                                        AttributeValue::S { S } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::N { N } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::B { B } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::SS { SS } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::NS { NS } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::BS { BS } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::M { M } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::L { L } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::NULL { NULL } => NULL,
                                        AttributeValue::BOOL { BOOL } => {
                                            panic!("field does not exist on this variant")
                                        }
                                    }
                                }
                                pub fn BOOL(&self) -> &bool {
                                    match self {
                                        AttributeValue::S { S } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::N { N } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::B { B } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::SS { SS } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::NS { NS } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::BS { BS } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::M { M } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::L { L } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::NULL { NULL } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        AttributeValue::BOOL { BOOL } => BOOL,
                                    }
                                }
                            }

                            impl Debug for AttributeValue {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for AttributeValue {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        AttributeValue::S { S } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.AttributeValue.S(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                S, _formatter, false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                        AttributeValue::N { N } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.AttributeValue.N(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                N, _formatter, false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                        AttributeValue::B { B } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.AttributeValue.B(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                B, _formatter, false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                        AttributeValue::SS { SS } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.AttributeValue.SS(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                SS, _formatter, false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                        AttributeValue::NS { NS } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.AttributeValue.NS(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                NS, _formatter, false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                        AttributeValue::BS { BS } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.AttributeValue.BS(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                BS, _formatter, false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                        AttributeValue::M { M } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.AttributeValue.M(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                M, _formatter, false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                        AttributeValue::L { L } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.AttributeValue.L(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                L, _formatter, false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                        AttributeValue::NULL { NULL } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.AttributeValue.NULL(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                NULL, _formatter, false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                        AttributeValue::BOOL { BOOL } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.AttributeValue.BOOL(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                BOOL, _formatter, false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for AttributeValue {}

                            impl Hash for AttributeValue {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        AttributeValue::S { S } => {
                                            ::std::hash::Hash::hash(S, _state)
                                        }
                                        AttributeValue::N { N } => {
                                            ::std::hash::Hash::hash(N, _state)
                                        }
                                        AttributeValue::B { B } => {
                                            ::std::hash::Hash::hash(B, _state)
                                        }
                                        AttributeValue::SS { SS } => {
                                            ::std::hash::Hash::hash(SS, _state)
                                        }
                                        AttributeValue::NS { NS } => {
                                            ::std::hash::Hash::hash(NS, _state)
                                        }
                                        AttributeValue::BS { BS } => {
                                            ::std::hash::Hash::hash(BS, _state)
                                        }
                                        AttributeValue::M { M } => {
                                            ::std::hash::Hash::hash(M, _state)
                                        }
                                        AttributeValue::L { L } => {
                                            ::std::hash::Hash::hash(L, _state)
                                        }
                                        AttributeValue::NULL { NULL } => {
                                            ::std::hash::Hash::hash(NULL, _state)
                                        }
                                        AttributeValue::BOOL { BOOL } => {
                                            ::std::hash::Hash::hash(BOOL, _state)
                                        }
                                    }
                                }
                            }

                            impl Default for AttributeValue {
                                fn default() -> AttributeValue {
                                    AttributeValue::S {
                                        S: ::std::default::Default::default(),
                                    }
                                }
                            }

                            impl AsRef<AttributeValue> for &AttributeValue {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            #[derive(PartialEq, Clone)]
                            pub enum AttributeValueUpdate {
                                AttributeValueUpdate {
                  Value: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>>,
                  Action: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeAction>>>
                }
              }

                            impl AttributeValueUpdate {
                                pub fn Value(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>>{
                                    match self {
                                        AttributeValueUpdate::AttributeValueUpdate {
                                            Value,
                                            Action,
                                        } => Value,
                                    }
                                }
                                pub fn Action(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeAction>>>{
                                    match self {
                                        AttributeValueUpdate::AttributeValueUpdate {
                                            Value,
                                            Action,
                                        } => Action,
                                    }
                                }
                            }

                            impl Debug for AttributeValueUpdate {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for AttributeValueUpdate {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        AttributeValueUpdate::AttributeValueUpdate {
                                            Value,
                                            Action,
                                        } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.AttributeValueUpdate.AttributeValueUpdate(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                Value, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                Action, _formatter, false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for AttributeValueUpdate {}

                            impl Hash for AttributeValueUpdate {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        AttributeValueUpdate::AttributeValueUpdate {
                                            Value,
                                            Action,
                                        } => {
                                            ::std::hash::Hash::hash(Value, _state);
                                            ::std::hash::Hash::hash(Action, _state)
                                        }
                                    }
                                }
                            }

                            impl Default for AttributeValueUpdate {
                                fn default() -> AttributeValueUpdate {
                                    AttributeValueUpdate::AttributeValueUpdate {
                                        Value: ::std::default::Default::default(),
                                        Action: ::std::default::Default::default(),
                                    }
                                }
                            }

                            impl AsRef<AttributeValueUpdate> for &AttributeValueUpdate {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            pub type BackupArn =
                                ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>;

                            #[derive(PartialEq, Clone)]
                            pub enum BatchGetItemInput {
                                BatchGetItemInput {
                  RequestItems: crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::BatchGetRequestMap,
                  ReturnConsumedCapacity: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ReturnConsumedCapacity>>>
                }
              }

                            impl BatchGetItemInput {
                                pub fn RequestItems(&self) -> &crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::BatchGetRequestMap{
                                    match self {
                                        BatchGetItemInput::BatchGetItemInput {
                                            RequestItems,
                                            ReturnConsumedCapacity,
                                        } => RequestItems,
                                    }
                                }
                                pub fn ReturnConsumedCapacity(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ReturnConsumedCapacity>>>{
                                    match self {
                                        BatchGetItemInput::BatchGetItemInput {
                                            RequestItems,
                                            ReturnConsumedCapacity,
                                        } => ReturnConsumedCapacity,
                                    }
                                }
                            }

                            impl Debug for BatchGetItemInput {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for BatchGetItemInput {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        BatchGetItemInput::BatchGetItemInput {
                                            RequestItems,
                                            ReturnConsumedCapacity,
                                        } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.BatchGetItemInput.BatchGetItemInput(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                RequestItems,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ReturnConsumedCapacity,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for BatchGetItemInput {}

                            impl Hash for BatchGetItemInput {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        BatchGetItemInput::BatchGetItemInput {
                                            RequestItems,
                                            ReturnConsumedCapacity,
                                        } => {
                                            ::std::hash::Hash::hash(RequestItems, _state);
                                            ::std::hash::Hash::hash(ReturnConsumedCapacity, _state)
                                        }
                                    }
                                }
                            }

                            impl Default for BatchGetItemInput {
                                fn default() -> BatchGetItemInput {
                                    BatchGetItemInput::BatchGetItemInput {
                                        RequestItems: ::std::default::Default::default(),
                                        ReturnConsumedCapacity: ::std::default::Default::default(),
                                    }
                                }
                            }

                            impl AsRef<BatchGetItemInput> for &BatchGetItemInput {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            #[derive(PartialEq, Clone)]
                            pub enum BatchGetItemOutput {
                                BatchGetItemOutput {
                  Responses: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::dafny_runtime::Sequence<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>>>>>,
                  UnprocessedKeys: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::BatchGetRequestMap>>,
                  ConsumedCapacity: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ConsumedCapacity>>>>
                }
              }

                            impl BatchGetItemOutput {
                                pub fn Responses(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::dafny_runtime::Sequence<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>>>>>{
                                    match self {
                                        BatchGetItemOutput::BatchGetItemOutput {
                                            Responses,
                                            UnprocessedKeys,
                                            ConsumedCapacity,
                                        } => Responses,
                                    }
                                }
                                pub fn UnprocessedKeys(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::BatchGetRequestMap>>{
                                    match self {
                                        BatchGetItemOutput::BatchGetItemOutput {
                                            Responses,
                                            UnprocessedKeys,
                                            ConsumedCapacity,
                                        } => UnprocessedKeys,
                                    }
                                }
                                pub fn ConsumedCapacity(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ConsumedCapacity>>>>{
                                    match self {
                                        BatchGetItemOutput::BatchGetItemOutput {
                                            Responses,
                                            UnprocessedKeys,
                                            ConsumedCapacity,
                                        } => ConsumedCapacity,
                                    }
                                }
                            }

                            impl Debug for BatchGetItemOutput {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for BatchGetItemOutput {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        BatchGetItemOutput::BatchGetItemOutput {
                                            Responses,
                                            UnprocessedKeys,
                                            ConsumedCapacity,
                                        } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.BatchGetItemOutput.BatchGetItemOutput(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                Responses, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                UnprocessedKeys,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ConsumedCapacity,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for BatchGetItemOutput {}

                            impl Hash for BatchGetItemOutput {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        BatchGetItemOutput::BatchGetItemOutput {
                                            Responses,
                                            UnprocessedKeys,
                                            ConsumedCapacity,
                                        } => {
                                            ::std::hash::Hash::hash(Responses, _state);
                                            ::std::hash::Hash::hash(UnprocessedKeys, _state);
                                            ::std::hash::Hash::hash(ConsumedCapacity, _state)
                                        }
                                    }
                                }
                            }

                            impl Default for BatchGetItemOutput {
                                fn default() -> BatchGetItemOutput {
                                    BatchGetItemOutput::BatchGetItemOutput {
                                        Responses: ::std::default::Default::default(),
                                        UnprocessedKeys: ::std::default::Default::default(),
                                        ConsumedCapacity: ::std::default::Default::default(),
                                    }
                                }
                            }

                            impl AsRef<BatchGetItemOutput> for &BatchGetItemOutput {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            pub type BatchGetRequestMap = ::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::KeysAndAttributes>>;

                            #[derive(PartialEq, Clone)]
                            pub enum BillingMode {
                                PROVISIONED {},
                                PAY_PER_REQUEST {},
                            }

                            impl BillingMode {}

                            impl Debug for BillingMode {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for BillingMode {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        BillingMode::PROVISIONED {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.BillingMode.PROVISIONED")?;
                                            Ok(())
                                        }
                                        BillingMode::PAY_PER_REQUEST {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.BillingMode.PAY__PER__REQUEST")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for BillingMode {}

                            impl Hash for BillingMode {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        BillingMode::PROVISIONED {} => {}
                                        BillingMode::PAY_PER_REQUEST {} => {}
                                    }
                                }
                            }

                            impl Default for BillingMode {
                                fn default() -> BillingMode {
                                    BillingMode::PROVISIONED {}
                                }
                            }

                            impl AsRef<BillingMode> for &BillingMode {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            #[derive(PartialEq, Clone)]
                            pub enum BillingModeSummary {
                                BillingModeSummary {
                  BillingMode: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::BillingMode>>>,
                  LastUpdateToPayPerRequestDateTime: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>
                }
              }

                            impl BillingModeSummary {
                                pub fn BillingMode(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::BillingMode>>>{
                                    match self {
                                        BillingModeSummary::BillingModeSummary {
                                            BillingMode,
                                            LastUpdateToPayPerRequestDateTime,
                                        } => BillingMode,
                                    }
                                }
                                pub fn LastUpdateToPayPerRequestDateTime(
                                    &self,
                                ) -> &::std::rc::Rc<
                                    crate::r#_Wrappers_Compile::Option<
                                        ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                    >,
                                > {
                                    match self {
                                        BillingModeSummary::BillingModeSummary {
                                            BillingMode,
                                            LastUpdateToPayPerRequestDateTime,
                                        } => LastUpdateToPayPerRequestDateTime,
                                    }
                                }
                            }

                            impl Debug for BillingModeSummary {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for BillingModeSummary {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        BillingModeSummary::BillingModeSummary {
                                            BillingMode,
                                            LastUpdateToPayPerRequestDateTime,
                                        } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.BillingModeSummary.BillingModeSummary(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                BillingMode,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                LastUpdateToPayPerRequestDateTime,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for BillingModeSummary {}

                            impl Hash for BillingModeSummary {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        BillingModeSummary::BillingModeSummary {
                                            BillingMode,
                                            LastUpdateToPayPerRequestDateTime,
                                        } => {
                                            ::std::hash::Hash::hash(BillingMode, _state);
                                            ::std::hash::Hash::hash(
                                                LastUpdateToPayPerRequestDateTime,
                                                _state,
                                            )
                                        }
                                    }
                                }
                            }

                            impl Default for BillingModeSummary {
                                fn default() -> BillingModeSummary {
                                    BillingModeSummary::BillingModeSummary {
                                        BillingMode: ::std::default::Default::default(),
                                        LastUpdateToPayPerRequestDateTime:
                                            ::std::default::Default::default(),
                                    }
                                }
                            }

                            impl AsRef<BillingModeSummary> for &BillingModeSummary {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            #[derive(PartialEq, Clone)]
                            pub enum CancellationReason {
                                CancellationReason {
                  Item: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>>>,
                  Code: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>,
                  Message: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>
                }
              }

                            impl CancellationReason {
                                pub fn Item(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>>>{
                                    match self {
                                        CancellationReason::CancellationReason {
                                            Item,
                                            Code,
                                            Message,
                                        } => Item,
                                    }
                                }
                                pub fn Code(
                                    &self,
                                ) -> &::std::rc::Rc<
                                    crate::r#_Wrappers_Compile::Option<
                                        ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                    >,
                                > {
                                    match self {
                                        CancellationReason::CancellationReason {
                                            Item,
                                            Code,
                                            Message,
                                        } => Code,
                                    }
                                }
                                pub fn Message(
                                    &self,
                                ) -> &::std::rc::Rc<
                                    crate::r#_Wrappers_Compile::Option<
                                        ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                    >,
                                > {
                                    match self {
                                        CancellationReason::CancellationReason {
                                            Item,
                                            Code,
                                            Message,
                                        } => Message,
                                    }
                                }
                            }

                            impl Debug for CancellationReason {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for CancellationReason {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        CancellationReason::CancellationReason {
                                            Item,
                                            Code,
                                            Message,
                                        } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.CancellationReason.CancellationReason(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                Item, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                Code, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                Message, _formatter, false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for CancellationReason {}

                            impl Hash for CancellationReason {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        CancellationReason::CancellationReason {
                                            Item,
                                            Code,
                                            Message,
                                        } => {
                                            ::std::hash::Hash::hash(Item, _state);
                                            ::std::hash::Hash::hash(Code, _state);
                                            ::std::hash::Hash::hash(Message, _state)
                                        }
                                    }
                                }
                            }

                            impl Default for CancellationReason {
                                fn default() -> CancellationReason {
                                    CancellationReason::CancellationReason {
                                        Item: ::std::default::Default::default(),
                                        Code: ::std::default::Default::default(),
                                        Message: ::std::default::Default::default(),
                                    }
                                }
                            }

                            impl AsRef<CancellationReason> for &CancellationReason {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            pub type CancellationReasonList = ::dafny_runtime::Sequence<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::CancellationReason>>;

                            #[derive(PartialEq, Clone)]
                            pub enum Capacity {
                                Capacity {
                  ReadCapacityUnits: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ConsumedCapacityUnits>>,
                  WriteCapacityUnits: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ConsumedCapacityUnits>>,
                  CapacityUnits: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ConsumedCapacityUnits>>
                }
              }

                            impl Capacity {
                                pub fn ReadCapacityUnits(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ConsumedCapacityUnits>>{
                                    match self {
                                        Capacity::Capacity {
                                            ReadCapacityUnits,
                                            WriteCapacityUnits,
                                            CapacityUnits,
                                        } => ReadCapacityUnits,
                                    }
                                }
                                pub fn WriteCapacityUnits(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ConsumedCapacityUnits>>{
                                    match self {
                                        Capacity::Capacity {
                                            ReadCapacityUnits,
                                            WriteCapacityUnits,
                                            CapacityUnits,
                                        } => WriteCapacityUnits,
                                    }
                                }
                                pub fn CapacityUnits(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ConsumedCapacityUnits>>{
                                    match self {
                                        Capacity::Capacity {
                                            ReadCapacityUnits,
                                            WriteCapacityUnits,
                                            CapacityUnits,
                                        } => CapacityUnits,
                                    }
                                }
                            }

                            impl Debug for Capacity {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for Capacity {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        Capacity::Capacity {
                                            ReadCapacityUnits,
                                            WriteCapacityUnits,
                                            CapacityUnits,
                                        } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.Capacity.Capacity(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ReadCapacityUnits,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                WriteCapacityUnits,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                CapacityUnits,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for Capacity {}

                            impl Hash for Capacity {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        Capacity::Capacity {
                                            ReadCapacityUnits,
                                            WriteCapacityUnits,
                                            CapacityUnits,
                                        } => {
                                            ::std::hash::Hash::hash(ReadCapacityUnits, _state);
                                            ::std::hash::Hash::hash(WriteCapacityUnits, _state);
                                            ::std::hash::Hash::hash(CapacityUnits, _state)
                                        }
                                    }
                                }
                            }

                            impl Default for Capacity {
                                fn default() -> Capacity {
                                    Capacity::Capacity {
                                        ReadCapacityUnits: ::std::default::Default::default(),
                                        WriteCapacityUnits: ::std::default::Default::default(),
                                        CapacityUnits: ::std::default::Default::default(),
                                    }
                                }
                            }

                            impl AsRef<Capacity> for &Capacity {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            pub type ClientRequestToken =
                                ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>;

                            #[derive(PartialEq, Clone)]
                            pub enum ComparisonOperator {
                                EQ {},
                                NE {},
                                IN {},
                                LE {},
                                LT {},
                                GE {},
                                GT {},
                                BETWEEN {},
                                NOT_NULL {},
                                NULL {},
                                CONTAINS {},
                                NOT_CONTAINS {},
                                BEGINS_WITH {},
                            }

                            impl ComparisonOperator {}

                            impl Debug for ComparisonOperator {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for ComparisonOperator {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        ComparisonOperator::EQ {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.ComparisonOperator.EQ")?;
                                            Ok(())
                                        }
                                        ComparisonOperator::NE {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.ComparisonOperator.NE")?;
                                            Ok(())
                                        }
                                        ComparisonOperator::IN {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.ComparisonOperator.IN")?;
                                            Ok(())
                                        }
                                        ComparisonOperator::LE {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.ComparisonOperator.LE")?;
                                            Ok(())
                                        }
                                        ComparisonOperator::LT {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.ComparisonOperator.LT")?;
                                            Ok(())
                                        }
                                        ComparisonOperator::GE {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.ComparisonOperator.GE")?;
                                            Ok(())
                                        }
                                        ComparisonOperator::GT {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.ComparisonOperator.GT")?;
                                            Ok(())
                                        }
                                        ComparisonOperator::BETWEEN {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.ComparisonOperator.BETWEEN")?;
                                            Ok(())
                                        }
                                        ComparisonOperator::NOT_NULL {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.ComparisonOperator.NOT__NULL")?;
                                            Ok(())
                                        }
                                        ComparisonOperator::NULL {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.ComparisonOperator.NULL")?;
                                            Ok(())
                                        }
                                        ComparisonOperator::CONTAINS {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.ComparisonOperator.CONTAINS")?;
                                            Ok(())
                                        }
                                        ComparisonOperator::NOT_CONTAINS {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.ComparisonOperator.NOT__CONTAINS")?;
                                            Ok(())
                                        }
                                        ComparisonOperator::BEGINS_WITH {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.ComparisonOperator.BEGINS__WITH")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for ComparisonOperator {}

                            impl Hash for ComparisonOperator {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        ComparisonOperator::EQ {} => {}
                                        ComparisonOperator::NE {} => {}
                                        ComparisonOperator::IN {} => {}
                                        ComparisonOperator::LE {} => {}
                                        ComparisonOperator::LT {} => {}
                                        ComparisonOperator::GE {} => {}
                                        ComparisonOperator::GT {} => {}
                                        ComparisonOperator::BETWEEN {} => {}
                                        ComparisonOperator::NOT_NULL {} => {}
                                        ComparisonOperator::NULL {} => {}
                                        ComparisonOperator::CONTAINS {} => {}
                                        ComparisonOperator::NOT_CONTAINS {} => {}
                                        ComparisonOperator::BEGINS_WITH {} => {}
                                    }
                                }
                            }

                            impl Default for ComparisonOperator {
                                fn default() -> ComparisonOperator {
                                    ComparisonOperator::EQ {}
                                }
                            }

                            impl AsRef<ComparisonOperator> for &ComparisonOperator {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            #[derive(PartialEq, Clone)]
                            pub enum Condition {
                                Condition {
                  AttributeValueList: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>>>,
                  ComparisonOperator: ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ComparisonOperator>
                }
              }

                            impl Condition {
                                pub fn AttributeValueList(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>>>{
                                    match self {
                                        Condition::Condition {
                                            AttributeValueList,
                                            ComparisonOperator,
                                        } => AttributeValueList,
                                    }
                                }
                                pub fn ComparisonOperator(&self) -> &::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ComparisonOperator>{
                                    match self {
                                        Condition::Condition {
                                            AttributeValueList,
                                            ComparisonOperator,
                                        } => ComparisonOperator,
                                    }
                                }
                            }

                            impl Debug for Condition {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for Condition {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        Condition::Condition {
                                            AttributeValueList,
                                            ComparisonOperator,
                                        } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.Condition.Condition(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                AttributeValueList,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ComparisonOperator,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for Condition {}

                            impl Hash for Condition {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        Condition::Condition {
                                            AttributeValueList,
                                            ComparisonOperator,
                                        } => {
                                            ::std::hash::Hash::hash(AttributeValueList, _state);
                                            ::std::hash::Hash::hash(ComparisonOperator, _state)
                                        }
                                    }
                                }
                            }

                            impl Default for Condition {
                                fn default() -> Condition {
                                    Condition::Condition {
                                        AttributeValueList: ::std::default::Default::default(),
                                        ComparisonOperator: ::std::default::Default::default(),
                                    }
                                }
                            }

                            impl AsRef<Condition> for &Condition {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            #[derive(PartialEq, Clone)]
                            pub enum ConditionalOperator {
                                AND {},
                                OR {},
                            }

                            impl ConditionalOperator {}

                            impl Debug for ConditionalOperator {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for ConditionalOperator {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        ConditionalOperator::AND {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.ConditionalOperator.AND")?;
                                            Ok(())
                                        }
                                        ConditionalOperator::OR {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.ConditionalOperator.OR")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for ConditionalOperator {}

                            impl Hash for ConditionalOperator {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        ConditionalOperator::AND {} => {}
                                        ConditionalOperator::OR {} => {}
                                    }
                                }
                            }

                            impl Default for ConditionalOperator {
                                fn default() -> ConditionalOperator {
                                    ConditionalOperator::AND {}
                                }
                            }

                            impl AsRef<ConditionalOperator> for &ConditionalOperator {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            #[derive(PartialEq, Clone)]
                            pub enum ConditionCheck {
                                ConditionCheck {
                  Key: ::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>,
                  TableName: ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                  ConditionExpression: ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                  ExpressionAttributeNames: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>>,
                  ExpressionAttributeValues: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>>>,
                  ReturnValuesOnConditionCheckFailure: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ReturnValuesOnConditionCheckFailure>>>
                }
              }

                            impl ConditionCheck {
                                pub fn Key(&self) -> &::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>{
                                    match self {
                                        ConditionCheck::ConditionCheck {
                                            Key,
                                            TableName,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                            ReturnValuesOnConditionCheckFailure,
                                        } => Key,
                                    }
                                }
                                pub fn TableName(
                                    &self,
                                ) -> &::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>
                                {
                                    match self {
                                        ConditionCheck::ConditionCheck {
                                            Key,
                                            TableName,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                            ReturnValuesOnConditionCheckFailure,
                                        } => TableName,
                                    }
                                }
                                pub fn ConditionExpression(
                                    &self,
                                ) -> &::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>
                                {
                                    match self {
                                        ConditionCheck::ConditionCheck {
                                            Key,
                                            TableName,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                            ReturnValuesOnConditionCheckFailure,
                                        } => ConditionExpression,
                                    }
                                }
                                pub fn ExpressionAttributeNames(
                                    &self,
                                ) -> &::std::rc::Rc<
                                    crate::r#_Wrappers_Compile::Option<
                                        ::dafny_runtime::Map<
                                            ::dafny_runtime::Sequence<
                                                ::dafny_runtime::DafnyCharUTF16,
                                            >,
                                            ::dafny_runtime::Sequence<
                                                ::dafny_runtime::DafnyCharUTF16,
                                            >,
                                        >,
                                    >,
                                > {
                                    match self {
                                        ConditionCheck::ConditionCheck {
                                            Key,
                                            TableName,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                            ReturnValuesOnConditionCheckFailure,
                                        } => ExpressionAttributeNames,
                                    }
                                }
                                pub fn ExpressionAttributeValues(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>>>{
                                    match self {
                                        ConditionCheck::ConditionCheck {
                                            Key,
                                            TableName,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                            ReturnValuesOnConditionCheckFailure,
                                        } => ExpressionAttributeValues,
                                    }
                                }
                                pub fn ReturnValuesOnConditionCheckFailure(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ReturnValuesOnConditionCheckFailure>>>{
                                    match self {
                                        ConditionCheck::ConditionCheck {
                                            Key,
                                            TableName,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                            ReturnValuesOnConditionCheckFailure,
                                        } => ReturnValuesOnConditionCheckFailure,
                                    }
                                }
                            }

                            impl Debug for ConditionCheck {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for ConditionCheck {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        ConditionCheck::ConditionCheck {
                                            Key,
                                            TableName,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                            ReturnValuesOnConditionCheckFailure,
                                        } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.ConditionCheck.ConditionCheck(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                Key, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                TableName, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ConditionExpression,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ExpressionAttributeNames,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ExpressionAttributeValues,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ReturnValuesOnConditionCheckFailure,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for ConditionCheck {}

                            impl Hash for ConditionCheck {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        ConditionCheck::ConditionCheck {
                                            Key,
                                            TableName,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                            ReturnValuesOnConditionCheckFailure,
                                        } => {
                                            ::std::hash::Hash::hash(Key, _state);
                                            ::std::hash::Hash::hash(TableName, _state);
                                            ::std::hash::Hash::hash(ConditionExpression, _state);
                                            ::std::hash::Hash::hash(
                                                ExpressionAttributeNames,
                                                _state,
                                            );
                                            ::std::hash::Hash::hash(
                                                ExpressionAttributeValues,
                                                _state,
                                            );
                                            ::std::hash::Hash::hash(
                                                ReturnValuesOnConditionCheckFailure,
                                                _state,
                                            )
                                        }
                                    }
                                }
                            }

                            impl Default for ConditionCheck {
                                fn default() -> ConditionCheck {
                                    ConditionCheck::ConditionCheck {
                                        Key: ::std::default::Default::default(),
                                        TableName: ::std::default::Default::default(),
                                        ConditionExpression: ::std::default::Default::default(),
                                        ExpressionAttributeNames: ::std::default::Default::default(
                                        ),
                                        ExpressionAttributeValues: ::std::default::Default::default(
                                        ),
                                        ReturnValuesOnConditionCheckFailure:
                                            ::std::default::Default::default(),
                                    }
                                }
                            }

                            impl AsRef<ConditionCheck> for &ConditionCheck {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            #[derive(PartialEq, Clone)]
                            pub enum ConsumedCapacity {
                                ConsumedCapacity {
                  TableName: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>,
                  CapacityUnits: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ConsumedCapacityUnits>>,
                  ReadCapacityUnits: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ConsumedCapacityUnits>>,
                  WriteCapacityUnits: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ConsumedCapacityUnits>>,
                  Table: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Capacity>>>,
                  LocalSecondaryIndexes: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Capacity>>>>,
                  GlobalSecondaryIndexes: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Capacity>>>>
                }
              }

                            impl ConsumedCapacity {
                                pub fn TableName(
                                    &self,
                                ) -> &::std::rc::Rc<
                                    crate::r#_Wrappers_Compile::Option<
                                        ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                    >,
                                > {
                                    match self {
                                        ConsumedCapacity::ConsumedCapacity {
                                            TableName,
                                            CapacityUnits,
                                            ReadCapacityUnits,
                                            WriteCapacityUnits,
                                            Table,
                                            LocalSecondaryIndexes,
                                            GlobalSecondaryIndexes,
                                        } => TableName,
                                    }
                                }
                                pub fn CapacityUnits(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ConsumedCapacityUnits>>{
                                    match self {
                                        ConsumedCapacity::ConsumedCapacity {
                                            TableName,
                                            CapacityUnits,
                                            ReadCapacityUnits,
                                            WriteCapacityUnits,
                                            Table,
                                            LocalSecondaryIndexes,
                                            GlobalSecondaryIndexes,
                                        } => CapacityUnits,
                                    }
                                }
                                pub fn ReadCapacityUnits(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ConsumedCapacityUnits>>{
                                    match self {
                                        ConsumedCapacity::ConsumedCapacity {
                                            TableName,
                                            CapacityUnits,
                                            ReadCapacityUnits,
                                            WriteCapacityUnits,
                                            Table,
                                            LocalSecondaryIndexes,
                                            GlobalSecondaryIndexes,
                                        } => ReadCapacityUnits,
                                    }
                                }
                                pub fn WriteCapacityUnits(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ConsumedCapacityUnits>>{
                                    match self {
                                        ConsumedCapacity::ConsumedCapacity {
                                            TableName,
                                            CapacityUnits,
                                            ReadCapacityUnits,
                                            WriteCapacityUnits,
                                            Table,
                                            LocalSecondaryIndexes,
                                            GlobalSecondaryIndexes,
                                        } => WriteCapacityUnits,
                                    }
                                }
                                pub fn Table(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Capacity>>>{
                                    match self {
                                        ConsumedCapacity::ConsumedCapacity {
                                            TableName,
                                            CapacityUnits,
                                            ReadCapacityUnits,
                                            WriteCapacityUnits,
                                            Table,
                                            LocalSecondaryIndexes,
                                            GlobalSecondaryIndexes,
                                        } => Table,
                                    }
                                }
                                pub fn LocalSecondaryIndexes(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Capacity>>>>{
                                    match self {
                                        ConsumedCapacity::ConsumedCapacity {
                                            TableName,
                                            CapacityUnits,
                                            ReadCapacityUnits,
                                            WriteCapacityUnits,
                                            Table,
                                            LocalSecondaryIndexes,
                                            GlobalSecondaryIndexes,
                                        } => LocalSecondaryIndexes,
                                    }
                                }
                                pub fn GlobalSecondaryIndexes(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Capacity>>>>{
                                    match self {
                                        ConsumedCapacity::ConsumedCapacity {
                                            TableName,
                                            CapacityUnits,
                                            ReadCapacityUnits,
                                            WriteCapacityUnits,
                                            Table,
                                            LocalSecondaryIndexes,
                                            GlobalSecondaryIndexes,
                                        } => GlobalSecondaryIndexes,
                                    }
                                }
                            }

                            impl Debug for ConsumedCapacity {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for ConsumedCapacity {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        ConsumedCapacity::ConsumedCapacity {
                                            TableName,
                                            CapacityUnits,
                                            ReadCapacityUnits,
                                            WriteCapacityUnits,
                                            Table,
                                            LocalSecondaryIndexes,
                                            GlobalSecondaryIndexes,
                                        } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.ConsumedCapacity.ConsumedCapacity(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                TableName, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                CapacityUnits,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ReadCapacityUnits,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                WriteCapacityUnits,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                Table, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                LocalSecondaryIndexes,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                GlobalSecondaryIndexes,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for ConsumedCapacity {}

                            impl Hash for ConsumedCapacity {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        ConsumedCapacity::ConsumedCapacity {
                                            TableName,
                                            CapacityUnits,
                                            ReadCapacityUnits,
                                            WriteCapacityUnits,
                                            Table,
                                            LocalSecondaryIndexes,
                                            GlobalSecondaryIndexes,
                                        } => {
                                            ::std::hash::Hash::hash(TableName, _state);
                                            ::std::hash::Hash::hash(CapacityUnits, _state);
                                            ::std::hash::Hash::hash(ReadCapacityUnits, _state);
                                            ::std::hash::Hash::hash(WriteCapacityUnits, _state);
                                            ::std::hash::Hash::hash(Table, _state);
                                            ::std::hash::Hash::hash(LocalSecondaryIndexes, _state);
                                            ::std::hash::Hash::hash(GlobalSecondaryIndexes, _state)
                                        }
                                    }
                                }
                            }

                            impl Default for ConsumedCapacity {
                                fn default() -> ConsumedCapacity {
                                    ConsumedCapacity::ConsumedCapacity {
                                        TableName: ::std::default::Default::default(),
                                        CapacityUnits: ::std::default::Default::default(),
                                        ReadCapacityUnits: ::std::default::Default::default(),
                                        WriteCapacityUnits: ::std::default::Default::default(),
                                        Table: ::std::default::Default::default(),
                                        LocalSecondaryIndexes: ::std::default::Default::default(),
                                        GlobalSecondaryIndexes: ::std::default::Default::default(),
                                    }
                                }
                            }

                            impl AsRef<ConsumedCapacity> for &ConsumedCapacity {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            pub type ConsumedCapacityUnits = ::dafny_runtime::Sequence<u8>;

                            #[derive(PartialEq, Clone)]
                            pub enum CreateTableInput {
                                CreateTableInput {
                  AttributeDefinitions: ::dafny_runtime::Sequence<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeDefinition>>,
                  TableName: ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                  KeySchema: crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::KeySchema,
                  LocalSecondaryIndexes: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::LocalSecondaryIndex>>>>,
                  GlobalSecondaryIndexes: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::GlobalSecondaryIndex>>>>,
                  BillingMode: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::BillingMode>>>,
                  ProvisionedThroughput: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ProvisionedThroughput>>>,
                  StreamSpecification: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::StreamSpecification>>>,
                  SSESpecification: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::SSESpecification>>>,
                  Tags: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Tag>>>>,
                  TableClass: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::TableClass>>>
                }
              }

                            impl CreateTableInput {
                                pub fn AttributeDefinitions(&self) -> &::dafny_runtime::Sequence<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeDefinition>>{
                                    match self {
                                        CreateTableInput::CreateTableInput {
                                            AttributeDefinitions,
                                            TableName,
                                            KeySchema,
                                            LocalSecondaryIndexes,
                                            GlobalSecondaryIndexes,
                                            BillingMode,
                                            ProvisionedThroughput,
                                            StreamSpecification,
                                            SSESpecification,
                                            Tags,
                                            TableClass,
                                        } => AttributeDefinitions,
                                    }
                                }
                                pub fn TableName(
                                    &self,
                                ) -> &::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>
                                {
                                    match self {
                                        CreateTableInput::CreateTableInput {
                                            AttributeDefinitions,
                                            TableName,
                                            KeySchema,
                                            LocalSecondaryIndexes,
                                            GlobalSecondaryIndexes,
                                            BillingMode,
                                            ProvisionedThroughput,
                                            StreamSpecification,
                                            SSESpecification,
                                            Tags,
                                            TableClass,
                                        } => TableName,
                                    }
                                }
                                pub fn KeySchema(&self) -> &crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::KeySchema{
                                    match self {
                                        CreateTableInput::CreateTableInput {
                                            AttributeDefinitions,
                                            TableName,
                                            KeySchema,
                                            LocalSecondaryIndexes,
                                            GlobalSecondaryIndexes,
                                            BillingMode,
                                            ProvisionedThroughput,
                                            StreamSpecification,
                                            SSESpecification,
                                            Tags,
                                            TableClass,
                                        } => KeySchema,
                                    }
                                }
                                pub fn LocalSecondaryIndexes(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::LocalSecondaryIndex>>>>{
                                    match self {
                                        CreateTableInput::CreateTableInput {
                                            AttributeDefinitions,
                                            TableName,
                                            KeySchema,
                                            LocalSecondaryIndexes,
                                            GlobalSecondaryIndexes,
                                            BillingMode,
                                            ProvisionedThroughput,
                                            StreamSpecification,
                                            SSESpecification,
                                            Tags,
                                            TableClass,
                                        } => LocalSecondaryIndexes,
                                    }
                                }
                                pub fn GlobalSecondaryIndexes(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::GlobalSecondaryIndex>>>>{
                                    match self {
                                        CreateTableInput::CreateTableInput {
                                            AttributeDefinitions,
                                            TableName,
                                            KeySchema,
                                            LocalSecondaryIndexes,
                                            GlobalSecondaryIndexes,
                                            BillingMode,
                                            ProvisionedThroughput,
                                            StreamSpecification,
                                            SSESpecification,
                                            Tags,
                                            TableClass,
                                        } => GlobalSecondaryIndexes,
                                    }
                                }
                                pub fn BillingMode(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::BillingMode>>>{
                                    match self {
                                        CreateTableInput::CreateTableInput {
                                            AttributeDefinitions,
                                            TableName,
                                            KeySchema,
                                            LocalSecondaryIndexes,
                                            GlobalSecondaryIndexes,
                                            BillingMode,
                                            ProvisionedThroughput,
                                            StreamSpecification,
                                            SSESpecification,
                                            Tags,
                                            TableClass,
                                        } => BillingMode,
                                    }
                                }
                                pub fn ProvisionedThroughput(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ProvisionedThroughput>>>{
                                    match self {
                                        CreateTableInput::CreateTableInput {
                                            AttributeDefinitions,
                                            TableName,
                                            KeySchema,
                                            LocalSecondaryIndexes,
                                            GlobalSecondaryIndexes,
                                            BillingMode,
                                            ProvisionedThroughput,
                                            StreamSpecification,
                                            SSESpecification,
                                            Tags,
                                            TableClass,
                                        } => ProvisionedThroughput,
                                    }
                                }
                                pub fn StreamSpecification(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::StreamSpecification>>>{
                                    match self {
                                        CreateTableInput::CreateTableInput {
                                            AttributeDefinitions,
                                            TableName,
                                            KeySchema,
                                            LocalSecondaryIndexes,
                                            GlobalSecondaryIndexes,
                                            BillingMode,
                                            ProvisionedThroughput,
                                            StreamSpecification,
                                            SSESpecification,
                                            Tags,
                                            TableClass,
                                        } => StreamSpecification,
                                    }
                                }
                                pub fn SSESpecification(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::SSESpecification>>>{
                                    match self {
                                        CreateTableInput::CreateTableInput {
                                            AttributeDefinitions,
                                            TableName,
                                            KeySchema,
                                            LocalSecondaryIndexes,
                                            GlobalSecondaryIndexes,
                                            BillingMode,
                                            ProvisionedThroughput,
                                            StreamSpecification,
                                            SSESpecification,
                                            Tags,
                                            TableClass,
                                        } => SSESpecification,
                                    }
                                }
                                pub fn Tags(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Tag>>>>{
                                    match self {
                                        CreateTableInput::CreateTableInput {
                                            AttributeDefinitions,
                                            TableName,
                                            KeySchema,
                                            LocalSecondaryIndexes,
                                            GlobalSecondaryIndexes,
                                            BillingMode,
                                            ProvisionedThroughput,
                                            StreamSpecification,
                                            SSESpecification,
                                            Tags,
                                            TableClass,
                                        } => Tags,
                                    }
                                }
                                pub fn TableClass(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::TableClass>>>{
                                    match self {
                                        CreateTableInput::CreateTableInput {
                                            AttributeDefinitions,
                                            TableName,
                                            KeySchema,
                                            LocalSecondaryIndexes,
                                            GlobalSecondaryIndexes,
                                            BillingMode,
                                            ProvisionedThroughput,
                                            StreamSpecification,
                                            SSESpecification,
                                            Tags,
                                            TableClass,
                                        } => TableClass,
                                    }
                                }
                            }

                            impl Debug for CreateTableInput {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for CreateTableInput {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        CreateTableInput::CreateTableInput {
                                            AttributeDefinitions,
                                            TableName,
                                            KeySchema,
                                            LocalSecondaryIndexes,
                                            GlobalSecondaryIndexes,
                                            BillingMode,
                                            ProvisionedThroughput,
                                            StreamSpecification,
                                            SSESpecification,
                                            Tags,
                                            TableClass,
                                        } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.CreateTableInput.CreateTableInput(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                AttributeDefinitions,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                TableName, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                KeySchema, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                LocalSecondaryIndexes,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                GlobalSecondaryIndexes,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                BillingMode,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ProvisionedThroughput,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                StreamSpecification,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                SSESpecification,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                Tags, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                TableClass, _formatter, false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for CreateTableInput {}

                            impl Hash for CreateTableInput {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        CreateTableInput::CreateTableInput {
                                            AttributeDefinitions,
                                            TableName,
                                            KeySchema,
                                            LocalSecondaryIndexes,
                                            GlobalSecondaryIndexes,
                                            BillingMode,
                                            ProvisionedThroughput,
                                            StreamSpecification,
                                            SSESpecification,
                                            Tags,
                                            TableClass,
                                        } => {
                                            ::std::hash::Hash::hash(AttributeDefinitions, _state);
                                            ::std::hash::Hash::hash(TableName, _state);
                                            ::std::hash::Hash::hash(KeySchema, _state);
                                            ::std::hash::Hash::hash(LocalSecondaryIndexes, _state);
                                            ::std::hash::Hash::hash(GlobalSecondaryIndexes, _state);
                                            ::std::hash::Hash::hash(BillingMode, _state);
                                            ::std::hash::Hash::hash(ProvisionedThroughput, _state);
                                            ::std::hash::Hash::hash(StreamSpecification, _state);
                                            ::std::hash::Hash::hash(SSESpecification, _state);
                                            ::std::hash::Hash::hash(Tags, _state);
                                            ::std::hash::Hash::hash(TableClass, _state)
                                        }
                                    }
                                }
                            }

                            impl Default for CreateTableInput {
                                fn default() -> CreateTableInput {
                                    CreateTableInput::CreateTableInput {
                                        AttributeDefinitions: ::std::default::Default::default(),
                                        TableName: ::std::default::Default::default(),
                                        KeySchema: ::std::default::Default::default(),
                                        LocalSecondaryIndexes: ::std::default::Default::default(),
                                        GlobalSecondaryIndexes: ::std::default::Default::default(),
                                        BillingMode: ::std::default::Default::default(),
                                        ProvisionedThroughput: ::std::default::Default::default(),
                                        StreamSpecification: ::std::default::Default::default(),
                                        SSESpecification: ::std::default::Default::default(),
                                        Tags: ::std::default::Default::default(),
                                        TableClass: ::std::default::Default::default(),
                                    }
                                }
                            }

                            impl AsRef<CreateTableInput> for &CreateTableInput {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            #[derive(PartialEq, Clone)]
                            pub enum CreateTableOutput {
                                CreateTableOutput {
                  TableDescription: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::TableDescription>>>
                }
              }

                            impl CreateTableOutput {
                                pub fn TableDescription(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::TableDescription>>>{
                                    match self {
                                        CreateTableOutput::CreateTableOutput {
                                            TableDescription,
                                        } => TableDescription,
                                    }
                                }
                            }

                            impl Debug for CreateTableOutput {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for CreateTableOutput {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        CreateTableOutput::CreateTableOutput {
                                            TableDescription,
                                        } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.CreateTableOutput.CreateTableOutput(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                TableDescription,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for CreateTableOutput {}

                            impl Hash for CreateTableOutput {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        CreateTableOutput::CreateTableOutput {
                                            TableDescription,
                                        } => ::std::hash::Hash::hash(TableDescription, _state),
                                    }
                                }
                            }

                            impl Default for CreateTableOutput {
                                fn default() -> CreateTableOutput {
                                    CreateTableOutput::CreateTableOutput {
                                        TableDescription: ::std::default::Default::default(),
                                    }
                                }
                            }

                            impl AsRef<CreateTableOutput> for &CreateTableOutput {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            #[derive(PartialEq, Clone)]
                            pub enum Delete {
                                Delete {
                  Key: ::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>,
                  TableName: ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                  ConditionExpression: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>,
                  ExpressionAttributeNames: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>>,
                  ExpressionAttributeValues: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>>>,
                  ReturnValuesOnConditionCheckFailure: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ReturnValuesOnConditionCheckFailure>>>
                }
              }

                            impl Delete {
                                pub fn Key(&self) -> &::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>{
                                    match self {
                                        Delete::Delete {
                                            Key,
                                            TableName,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                            ReturnValuesOnConditionCheckFailure,
                                        } => Key,
                                    }
                                }
                                pub fn TableName(
                                    &self,
                                ) -> &::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>
                                {
                                    match self {
                                        Delete::Delete {
                                            Key,
                                            TableName,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                            ReturnValuesOnConditionCheckFailure,
                                        } => TableName,
                                    }
                                }
                                pub fn ConditionExpression(
                                    &self,
                                ) -> &::std::rc::Rc<
                                    crate::r#_Wrappers_Compile::Option<
                                        ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                    >,
                                > {
                                    match self {
                                        Delete::Delete {
                                            Key,
                                            TableName,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                            ReturnValuesOnConditionCheckFailure,
                                        } => ConditionExpression,
                                    }
                                }
                                pub fn ExpressionAttributeNames(
                                    &self,
                                ) -> &::std::rc::Rc<
                                    crate::r#_Wrappers_Compile::Option<
                                        ::dafny_runtime::Map<
                                            ::dafny_runtime::Sequence<
                                                ::dafny_runtime::DafnyCharUTF16,
                                            >,
                                            ::dafny_runtime::Sequence<
                                                ::dafny_runtime::DafnyCharUTF16,
                                            >,
                                        >,
                                    >,
                                > {
                                    match self {
                                        Delete::Delete {
                                            Key,
                                            TableName,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                            ReturnValuesOnConditionCheckFailure,
                                        } => ExpressionAttributeNames,
                                    }
                                }
                                pub fn ExpressionAttributeValues(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>>>{
                                    match self {
                                        Delete::Delete {
                                            Key,
                                            TableName,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                            ReturnValuesOnConditionCheckFailure,
                                        } => ExpressionAttributeValues,
                                    }
                                }
                                pub fn ReturnValuesOnConditionCheckFailure(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ReturnValuesOnConditionCheckFailure>>>{
                                    match self {
                                        Delete::Delete {
                                            Key,
                                            TableName,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                            ReturnValuesOnConditionCheckFailure,
                                        } => ReturnValuesOnConditionCheckFailure,
                                    }
                                }
                            }

                            impl Debug for Delete {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for Delete {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        Delete::Delete {
                                            Key,
                                            TableName,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                            ReturnValuesOnConditionCheckFailure,
                                        } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.Delete.Delete(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                Key, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                TableName, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ConditionExpression,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ExpressionAttributeNames,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ExpressionAttributeValues,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ReturnValuesOnConditionCheckFailure,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for Delete {}

                            impl Hash for Delete {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        Delete::Delete {
                                            Key,
                                            TableName,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                            ReturnValuesOnConditionCheckFailure,
                                        } => {
                                            ::std::hash::Hash::hash(Key, _state);
                                            ::std::hash::Hash::hash(TableName, _state);
                                            ::std::hash::Hash::hash(ConditionExpression, _state);
                                            ::std::hash::Hash::hash(
                                                ExpressionAttributeNames,
                                                _state,
                                            );
                                            ::std::hash::Hash::hash(
                                                ExpressionAttributeValues,
                                                _state,
                                            );
                                            ::std::hash::Hash::hash(
                                                ReturnValuesOnConditionCheckFailure,
                                                _state,
                                            )
                                        }
                                    }
                                }
                            }

                            impl Default for Delete {
                                fn default() -> Delete {
                                    Delete::Delete {
                                        Key: ::std::default::Default::default(),
                                        TableName: ::std::default::Default::default(),
                                        ConditionExpression: ::std::default::Default::default(),
                                        ExpressionAttributeNames: ::std::default::Default::default(
                                        ),
                                        ExpressionAttributeValues: ::std::default::Default::default(
                                        ),
                                        ReturnValuesOnConditionCheckFailure:
                                            ::std::default::Default::default(),
                                    }
                                }
                            }

                            impl AsRef<Delete> for &Delete {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            #[derive(PartialEq, Clone)]
                            pub enum DeleteItemInput {
                                DeleteItemInput {
                  TableName: ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                  Key: ::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>,
                  Expected: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ExpectedAttributeValue>>>>,
                  ConditionalOperator: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ConditionalOperator>>>,
                  ReturnValues: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ReturnValue>>>,
                  ReturnConsumedCapacity: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ReturnConsumedCapacity>>>,
                  ReturnItemCollectionMetrics: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ReturnItemCollectionMetrics>>>,
                  ConditionExpression: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>,
                  ExpressionAttributeNames: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>>,
                  ExpressionAttributeValues: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>>>
                }
              }

                            impl DeleteItemInput {
                                pub fn TableName(
                                    &self,
                                ) -> &::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>
                                {
                                    match self {
                                        DeleteItemInput::DeleteItemInput {
                                            TableName,
                                            Key,
                                            Expected,
                                            ConditionalOperator,
                                            ReturnValues,
                                            ReturnConsumedCapacity,
                                            ReturnItemCollectionMetrics,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                        } => TableName,
                                    }
                                }
                                pub fn Key(&self) -> &::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>{
                                    match self {
                                        DeleteItemInput::DeleteItemInput {
                                            TableName,
                                            Key,
                                            Expected,
                                            ConditionalOperator,
                                            ReturnValues,
                                            ReturnConsumedCapacity,
                                            ReturnItemCollectionMetrics,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                        } => Key,
                                    }
                                }
                                pub fn Expected(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ExpectedAttributeValue>>>>{
                                    match self {
                                        DeleteItemInput::DeleteItemInput {
                                            TableName,
                                            Key,
                                            Expected,
                                            ConditionalOperator,
                                            ReturnValues,
                                            ReturnConsumedCapacity,
                                            ReturnItemCollectionMetrics,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                        } => Expected,
                                    }
                                }
                                pub fn ConditionalOperator(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ConditionalOperator>>>{
                                    match self {
                                        DeleteItemInput::DeleteItemInput {
                                            TableName,
                                            Key,
                                            Expected,
                                            ConditionalOperator,
                                            ReturnValues,
                                            ReturnConsumedCapacity,
                                            ReturnItemCollectionMetrics,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                        } => ConditionalOperator,
                                    }
                                }
                                pub fn ReturnValues(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ReturnValue>>>{
                                    match self {
                                        DeleteItemInput::DeleteItemInput {
                                            TableName,
                                            Key,
                                            Expected,
                                            ConditionalOperator,
                                            ReturnValues,
                                            ReturnConsumedCapacity,
                                            ReturnItemCollectionMetrics,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                        } => ReturnValues,
                                    }
                                }
                                pub fn ReturnConsumedCapacity(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ReturnConsumedCapacity>>>{
                                    match self {
                                        DeleteItemInput::DeleteItemInput {
                                            TableName,
                                            Key,
                                            Expected,
                                            ConditionalOperator,
                                            ReturnValues,
                                            ReturnConsumedCapacity,
                                            ReturnItemCollectionMetrics,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                        } => ReturnConsumedCapacity,
                                    }
                                }
                                pub fn ReturnItemCollectionMetrics(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ReturnItemCollectionMetrics>>>{
                                    match self {
                                        DeleteItemInput::DeleteItemInput {
                                            TableName,
                                            Key,
                                            Expected,
                                            ConditionalOperator,
                                            ReturnValues,
                                            ReturnConsumedCapacity,
                                            ReturnItemCollectionMetrics,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                        } => ReturnItemCollectionMetrics,
                                    }
                                }
                                pub fn ConditionExpression(
                                    &self,
                                ) -> &::std::rc::Rc<
                                    crate::r#_Wrappers_Compile::Option<
                                        ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                    >,
                                > {
                                    match self {
                                        DeleteItemInput::DeleteItemInput {
                                            TableName,
                                            Key,
                                            Expected,
                                            ConditionalOperator,
                                            ReturnValues,
                                            ReturnConsumedCapacity,
                                            ReturnItemCollectionMetrics,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                        } => ConditionExpression,
                                    }
                                }
                                pub fn ExpressionAttributeNames(
                                    &self,
                                ) -> &::std::rc::Rc<
                                    crate::r#_Wrappers_Compile::Option<
                                        ::dafny_runtime::Map<
                                            ::dafny_runtime::Sequence<
                                                ::dafny_runtime::DafnyCharUTF16,
                                            >,
                                            ::dafny_runtime::Sequence<
                                                ::dafny_runtime::DafnyCharUTF16,
                                            >,
                                        >,
                                    >,
                                > {
                                    match self {
                                        DeleteItemInput::DeleteItemInput {
                                            TableName,
                                            Key,
                                            Expected,
                                            ConditionalOperator,
                                            ReturnValues,
                                            ReturnConsumedCapacity,
                                            ReturnItemCollectionMetrics,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                        } => ExpressionAttributeNames,
                                    }
                                }
                                pub fn ExpressionAttributeValues(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>>>{
                                    match self {
                                        DeleteItemInput::DeleteItemInput {
                                            TableName,
                                            Key,
                                            Expected,
                                            ConditionalOperator,
                                            ReturnValues,
                                            ReturnConsumedCapacity,
                                            ReturnItemCollectionMetrics,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                        } => ExpressionAttributeValues,
                                    }
                                }
                            }

                            impl Debug for DeleteItemInput {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for DeleteItemInput {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        DeleteItemInput::DeleteItemInput {
                                            TableName,
                                            Key,
                                            Expected,
                                            ConditionalOperator,
                                            ReturnValues,
                                            ReturnConsumedCapacity,
                                            ReturnItemCollectionMetrics,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                        } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.DeleteItemInput.DeleteItemInput(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                TableName, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                Key, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                Expected, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ConditionalOperator,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ReturnValues,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ReturnConsumedCapacity,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ReturnItemCollectionMetrics,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ConditionExpression,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ExpressionAttributeNames,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ExpressionAttributeValues,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for DeleteItemInput {}

                            impl Hash for DeleteItemInput {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        DeleteItemInput::DeleteItemInput {
                                            TableName,
                                            Key,
                                            Expected,
                                            ConditionalOperator,
                                            ReturnValues,
                                            ReturnConsumedCapacity,
                                            ReturnItemCollectionMetrics,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                        } => {
                                            ::std::hash::Hash::hash(TableName, _state);
                                            ::std::hash::Hash::hash(Key, _state);
                                            ::std::hash::Hash::hash(Expected, _state);
                                            ::std::hash::Hash::hash(ConditionalOperator, _state);
                                            ::std::hash::Hash::hash(ReturnValues, _state);
                                            ::std::hash::Hash::hash(ReturnConsumedCapacity, _state);
                                            ::std::hash::Hash::hash(
                                                ReturnItemCollectionMetrics,
                                                _state,
                                            );
                                            ::std::hash::Hash::hash(ConditionExpression, _state);
                                            ::std::hash::Hash::hash(
                                                ExpressionAttributeNames,
                                                _state,
                                            );
                                            ::std::hash::Hash::hash(
                                                ExpressionAttributeValues,
                                                _state,
                                            )
                                        }
                                    }
                                }
                            }

                            impl Default for DeleteItemInput {
                                fn default() -> DeleteItemInput {
                                    DeleteItemInput::DeleteItemInput {
                                        TableName: ::std::default::Default::default(),
                                        Key: ::std::default::Default::default(),
                                        Expected: ::std::default::Default::default(),
                                        ConditionalOperator: ::std::default::Default::default(),
                                        ReturnValues: ::std::default::Default::default(),
                                        ReturnConsumedCapacity: ::std::default::Default::default(),
                                        ReturnItemCollectionMetrics:
                                            ::std::default::Default::default(),
                                        ConditionExpression: ::std::default::Default::default(),
                                        ExpressionAttributeNames: ::std::default::Default::default(
                                        ),
                                        ExpressionAttributeValues: ::std::default::Default::default(
                                        ),
                                    }
                                }
                            }

                            impl AsRef<DeleteItemInput> for &DeleteItemInput {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            #[derive(PartialEq, Clone)]
                            pub enum DeleteItemOutput {
                                DeleteItemOutput {
                  Attributes: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>>>,
                  ConsumedCapacity: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ConsumedCapacity>>>,
                  ItemCollectionMetrics: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ItemCollectionMetrics>>>
                }
              }

                            impl DeleteItemOutput {
                                pub fn Attributes(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>>>{
                                    match self {
                                        DeleteItemOutput::DeleteItemOutput {
                                            Attributes,
                                            ConsumedCapacity,
                                            ItemCollectionMetrics,
                                        } => Attributes,
                                    }
                                }
                                pub fn ConsumedCapacity(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ConsumedCapacity>>>{
                                    match self {
                                        DeleteItemOutput::DeleteItemOutput {
                                            Attributes,
                                            ConsumedCapacity,
                                            ItemCollectionMetrics,
                                        } => ConsumedCapacity,
                                    }
                                }
                                pub fn ItemCollectionMetrics(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ItemCollectionMetrics>>>{
                                    match self {
                                        DeleteItemOutput::DeleteItemOutput {
                                            Attributes,
                                            ConsumedCapacity,
                                            ItemCollectionMetrics,
                                        } => ItemCollectionMetrics,
                                    }
                                }
                            }

                            impl Debug for DeleteItemOutput {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for DeleteItemOutput {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        DeleteItemOutput::DeleteItemOutput {
                                            Attributes,
                                            ConsumedCapacity,
                                            ItemCollectionMetrics,
                                        } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.DeleteItemOutput.DeleteItemOutput(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                Attributes, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ConsumedCapacity,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ItemCollectionMetrics,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for DeleteItemOutput {}

                            impl Hash for DeleteItemOutput {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        DeleteItemOutput::DeleteItemOutput {
                                            Attributes,
                                            ConsumedCapacity,
                                            ItemCollectionMetrics,
                                        } => {
                                            ::std::hash::Hash::hash(Attributes, _state);
                                            ::std::hash::Hash::hash(ConsumedCapacity, _state);
                                            ::std::hash::Hash::hash(ItemCollectionMetrics, _state)
                                        }
                                    }
                                }
                            }

                            impl Default for DeleteItemOutput {
                                fn default() -> DeleteItemOutput {
                                    DeleteItemOutput::DeleteItemOutput {
                                        Attributes: ::std::default::Default::default(),
                                        ConsumedCapacity: ::std::default::Default::default(),
                                        ItemCollectionMetrics: ::std::default::Default::default(),
                                    }
                                }
                            }

                            impl AsRef<DeleteItemOutput> for &DeleteItemOutput {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            #[derive(PartialEq, Clone)]
                            pub enum DescribeTableInput {
                                DescribeTableInput {
                                    TableName:
                                        ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                },
                            }

                            impl DescribeTableInput {
                                pub fn TableName(
                                    &self,
                                ) -> &::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>
                                {
                                    match self {
                                        DescribeTableInput::DescribeTableInput { TableName } => {
                                            TableName
                                        }
                                    }
                                }
                            }

                            impl Debug for DescribeTableInput {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for DescribeTableInput {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        DescribeTableInput::DescribeTableInput { TableName } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.DescribeTableInput.DescribeTableInput(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                TableName, _formatter, false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for DescribeTableInput {}

                            impl Hash for DescribeTableInput {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        DescribeTableInput::DescribeTableInput { TableName } => {
                                            ::std::hash::Hash::hash(TableName, _state)
                                        }
                                    }
                                }
                            }

                            impl Default for DescribeTableInput {
                                fn default() -> DescribeTableInput {
                                    DescribeTableInput::DescribeTableInput {
                                        TableName: ::std::default::Default::default(),
                                    }
                                }
                            }

                            impl AsRef<DescribeTableInput> for &DescribeTableInput {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            #[derive(PartialEq, Clone)]
                            pub enum DescribeTableOutput {
                                DescribeTableOutput {
                  Table: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::TableDescription>>>
                }
              }

                            impl DescribeTableOutput {
                                pub fn Table(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::TableDescription>>>{
                                    match self {
                                        DescribeTableOutput::DescribeTableOutput { Table } => Table,
                                    }
                                }
                            }

                            impl Debug for DescribeTableOutput {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for DescribeTableOutput {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        DescribeTableOutput::DescribeTableOutput { Table } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.DescribeTableOutput.DescribeTableOutput(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                Table, _formatter, false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for DescribeTableOutput {}

                            impl Hash for DescribeTableOutput {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        DescribeTableOutput::DescribeTableOutput { Table } => {
                                            ::std::hash::Hash::hash(Table, _state)
                                        }
                                    }
                                }
                            }

                            impl Default for DescribeTableOutput {
                                fn default() -> DescribeTableOutput {
                                    DescribeTableOutput::DescribeTableOutput {
                                        Table: ::std::default::Default::default(),
                                    }
                                }
                            }

                            impl AsRef<DescribeTableOutput> for &DescribeTableOutput {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            pub struct IDynamoDBClientCallHistory {}

                            impl IDynamoDBClientCallHistory {
                                pub fn _allocate_object() -> ::dafny_runtime::Object<Self> {
                                    ::dafny_runtime::allocate_object::<Self>()
                                }
                            }

                            impl UpcastObject<dyn Any>
                for crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::IDynamoDBClientCallHistory {
                ::dafny_runtime::UpcastObjectFn!(dyn ::std::any::Any);
              }

                            pub trait IDynamoDBClient:
                                ::std::any::Any
                                + ::dafny_runtime::UpcastObject<dyn::std::any::Any>
                            {
                                fn BatchGetItem(&mut self, input: &::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::BatchGetItemInput>) -> ::std::rc::Rc<crate::r#_Wrappers_Compile::Result<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::BatchGetItemOutput>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>>>;
                                fn CreateTable(&mut self, input: &::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::CreateTableInput>) -> ::std::rc::Rc<crate::r#_Wrappers_Compile::Result<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::CreateTableOutput>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>>>;
                                fn DeleteItem(&mut self, input: &::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::DeleteItemInput>) -> ::std::rc::Rc<crate::r#_Wrappers_Compile::Result<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::DeleteItemOutput>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>>>;
                                fn DescribeTable(&mut self, input: &::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::DescribeTableInput>) -> ::std::rc::Rc<crate::r#_Wrappers_Compile::Result<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::DescribeTableOutput>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>>>;
                                fn GetItem(&mut self, input: &::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::GetItemInput>) -> ::std::rc::Rc<crate::r#_Wrappers_Compile::Result<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::GetItemOutput>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>>>;
                                fn PutItem(&mut self, input: &::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::PutItemInput>) -> ::std::rc::Rc<crate::r#_Wrappers_Compile::Result<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::PutItemOutput>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>>>;
                                fn Query(&mut self, input: &::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::QueryInput>) -> ::std::rc::Rc<crate::r#_Wrappers_Compile::Result<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::QueryOutput>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>>>;
                                fn Scan(&mut self, input: &::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ScanInput>) -> ::std::rc::Rc<crate::r#_Wrappers_Compile::Result<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ScanOutput>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>>>;
                                fn TransactWriteItems(&mut self, input: &::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::TransactWriteItemsInput>) -> ::std::rc::Rc<crate::r#_Wrappers_Compile::Result<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::TransactWriteItemsOutput>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>>>;
                                fn UpdateItem(&mut self, input: &::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::UpdateItemInput>) -> ::std::rc::Rc<crate::r#_Wrappers_Compile::Result<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::UpdateItemOutput>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>>>;
                            }

                            #[derive(PartialEq, Clone)]
                            pub enum ExpectedAttributeValue {
                                ExpectedAttributeValue {
                  Value: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>>,
                  Exists: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<bool>>,
                  ComparisonOperator: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ComparisonOperator>>>,
                  AttributeValueList: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>>>
                }
              }

                            impl ExpectedAttributeValue {
                                pub fn Value(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>>{
                                    match self {
                                        ExpectedAttributeValue::ExpectedAttributeValue {
                                            Value,
                                            Exists,
                                            ComparisonOperator,
                                            AttributeValueList,
                                        } => Value,
                                    }
                                }
                                pub fn Exists(
                                    &self,
                                ) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<bool>>
                                {
                                    match self {
                                        ExpectedAttributeValue::ExpectedAttributeValue {
                                            Value,
                                            Exists,
                                            ComparisonOperator,
                                            AttributeValueList,
                                        } => Exists,
                                    }
                                }
                                pub fn ComparisonOperator(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ComparisonOperator>>>{
                                    match self {
                                        ExpectedAttributeValue::ExpectedAttributeValue {
                                            Value,
                                            Exists,
                                            ComparisonOperator,
                                            AttributeValueList,
                                        } => ComparisonOperator,
                                    }
                                }
                                pub fn AttributeValueList(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>>>{
                                    match self {
                                        ExpectedAttributeValue::ExpectedAttributeValue {
                                            Value,
                                            Exists,
                                            ComparisonOperator,
                                            AttributeValueList,
                                        } => AttributeValueList,
                                    }
                                }
                            }

                            impl Debug for ExpectedAttributeValue {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for ExpectedAttributeValue {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        ExpectedAttributeValue::ExpectedAttributeValue {
                                            Value,
                                            Exists,
                                            ComparisonOperator,
                                            AttributeValueList,
                                        } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.ExpectedAttributeValue.ExpectedAttributeValue(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                Value, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                Exists, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ComparisonOperator,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                AttributeValueList,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for ExpectedAttributeValue {}

                            impl Hash for ExpectedAttributeValue {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        ExpectedAttributeValue::ExpectedAttributeValue {
                                            Value,
                                            Exists,
                                            ComparisonOperator,
                                            AttributeValueList,
                                        } => {
                                            ::std::hash::Hash::hash(Value, _state);
                                            ::std::hash::Hash::hash(Exists, _state);
                                            ::std::hash::Hash::hash(ComparisonOperator, _state);
                                            ::std::hash::Hash::hash(AttributeValueList, _state)
                                        }
                                    }
                                }
                            }

                            impl Default for ExpectedAttributeValue {
                                fn default() -> ExpectedAttributeValue {
                                    ExpectedAttributeValue::ExpectedAttributeValue {
                                        Value: ::std::default::Default::default(),
                                        Exists: ::std::default::Default::default(),
                                        ComparisonOperator: ::std::default::Default::default(),
                                        AttributeValueList: ::std::default::Default::default(),
                                    }
                                }
                            }

                            impl AsRef<ExpectedAttributeValue> for &ExpectedAttributeValue {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            #[derive(PartialEq, Clone)]
                            pub enum GetItemInput {
                                GetItemInput {
                  TableName: ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                  Key: ::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>,
                  AttributesToGet: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeNameList>>,
                  ConsistentRead: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<bool>>,
                  ReturnConsumedCapacity: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ReturnConsumedCapacity>>>,
                  ProjectionExpression: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>,
                  ExpressionAttributeNames: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>>
                }
              }

                            impl GetItemInput {
                                pub fn TableName(
                                    &self,
                                ) -> &::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>
                                {
                                    match self {
                                        GetItemInput::GetItemInput {
                                            TableName,
                                            Key,
                                            AttributesToGet,
                                            ConsistentRead,
                                            ReturnConsumedCapacity,
                                            ProjectionExpression,
                                            ExpressionAttributeNames,
                                        } => TableName,
                                    }
                                }
                                pub fn Key(&self) -> &::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>{
                                    match self {
                                        GetItemInput::GetItemInput {
                                            TableName,
                                            Key,
                                            AttributesToGet,
                                            ConsistentRead,
                                            ReturnConsumedCapacity,
                                            ProjectionExpression,
                                            ExpressionAttributeNames,
                                        } => Key,
                                    }
                                }
                                pub fn AttributesToGet(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeNameList>>{
                                    match self {
                                        GetItemInput::GetItemInput {
                                            TableName,
                                            Key,
                                            AttributesToGet,
                                            ConsistentRead,
                                            ReturnConsumedCapacity,
                                            ProjectionExpression,
                                            ExpressionAttributeNames,
                                        } => AttributesToGet,
                                    }
                                }
                                pub fn ConsistentRead(
                                    &self,
                                ) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<bool>>
                                {
                                    match self {
                                        GetItemInput::GetItemInput {
                                            TableName,
                                            Key,
                                            AttributesToGet,
                                            ConsistentRead,
                                            ReturnConsumedCapacity,
                                            ProjectionExpression,
                                            ExpressionAttributeNames,
                                        } => ConsistentRead,
                                    }
                                }
                                pub fn ReturnConsumedCapacity(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ReturnConsumedCapacity>>>{
                                    match self {
                                        GetItemInput::GetItemInput {
                                            TableName,
                                            Key,
                                            AttributesToGet,
                                            ConsistentRead,
                                            ReturnConsumedCapacity,
                                            ProjectionExpression,
                                            ExpressionAttributeNames,
                                        } => ReturnConsumedCapacity,
                                    }
                                }
                                pub fn ProjectionExpression(
                                    &self,
                                ) -> &::std::rc::Rc<
                                    crate::r#_Wrappers_Compile::Option<
                                        ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                    >,
                                > {
                                    match self {
                                        GetItemInput::GetItemInput {
                                            TableName,
                                            Key,
                                            AttributesToGet,
                                            ConsistentRead,
                                            ReturnConsumedCapacity,
                                            ProjectionExpression,
                                            ExpressionAttributeNames,
                                        } => ProjectionExpression,
                                    }
                                }
                                pub fn ExpressionAttributeNames(
                                    &self,
                                ) -> &::std::rc::Rc<
                                    crate::r#_Wrappers_Compile::Option<
                                        ::dafny_runtime::Map<
                                            ::dafny_runtime::Sequence<
                                                ::dafny_runtime::DafnyCharUTF16,
                                            >,
                                            ::dafny_runtime::Sequence<
                                                ::dafny_runtime::DafnyCharUTF16,
                                            >,
                                        >,
                                    >,
                                > {
                                    match self {
                                        GetItemInput::GetItemInput {
                                            TableName,
                                            Key,
                                            AttributesToGet,
                                            ConsistentRead,
                                            ReturnConsumedCapacity,
                                            ProjectionExpression,
                                            ExpressionAttributeNames,
                                        } => ExpressionAttributeNames,
                                    }
                                }
                            }

                            impl Debug for GetItemInput {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for GetItemInput {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        GetItemInput::GetItemInput {
                                            TableName,
                                            Key,
                                            AttributesToGet,
                                            ConsistentRead,
                                            ReturnConsumedCapacity,
                                            ProjectionExpression,
                                            ExpressionAttributeNames,
                                        } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.GetItemInput.GetItemInput(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                TableName, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                Key, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                AttributesToGet,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ConsistentRead,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ReturnConsumedCapacity,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ProjectionExpression,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ExpressionAttributeNames,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for GetItemInput {}

                            impl Hash for GetItemInput {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        GetItemInput::GetItemInput {
                                            TableName,
                                            Key,
                                            AttributesToGet,
                                            ConsistentRead,
                                            ReturnConsumedCapacity,
                                            ProjectionExpression,
                                            ExpressionAttributeNames,
                                        } => {
                                            ::std::hash::Hash::hash(TableName, _state);
                                            ::std::hash::Hash::hash(Key, _state);
                                            ::std::hash::Hash::hash(AttributesToGet, _state);
                                            ::std::hash::Hash::hash(ConsistentRead, _state);
                                            ::std::hash::Hash::hash(ReturnConsumedCapacity, _state);
                                            ::std::hash::Hash::hash(ProjectionExpression, _state);
                                            ::std::hash::Hash::hash(
                                                ExpressionAttributeNames,
                                                _state,
                                            )
                                        }
                                    }
                                }
                            }

                            impl Default for GetItemInput {
                                fn default() -> GetItemInput {
                                    GetItemInput::GetItemInput {
                                        TableName: ::std::default::Default::default(),
                                        Key: ::std::default::Default::default(),
                                        AttributesToGet: ::std::default::Default::default(),
                                        ConsistentRead: ::std::default::Default::default(),
                                        ReturnConsumedCapacity: ::std::default::Default::default(),
                                        ProjectionExpression: ::std::default::Default::default(),
                                        ExpressionAttributeNames: ::std::default::Default::default(
                                        ),
                                    }
                                }
                            }

                            impl AsRef<GetItemInput> for &GetItemInput {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            #[derive(PartialEq, Clone)]
                            pub enum GetItemOutput {
                                GetItemOutput {
                  Item: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>>>,
                  ConsumedCapacity: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ConsumedCapacity>>>
                }
              }

                            impl GetItemOutput {
                                pub fn Item(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>>>{
                                    match self {
                                        GetItemOutput::GetItemOutput {
                                            Item,
                                            ConsumedCapacity,
                                        } => Item,
                                    }
                                }
                                pub fn ConsumedCapacity(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ConsumedCapacity>>>{
                                    match self {
                                        GetItemOutput::GetItemOutput {
                                            Item,
                                            ConsumedCapacity,
                                        } => ConsumedCapacity,
                                    }
                                }
                            }

                            impl Debug for GetItemOutput {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for GetItemOutput {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        GetItemOutput::GetItemOutput {
                                            Item,
                                            ConsumedCapacity,
                                        } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.GetItemOutput.GetItemOutput(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                Item, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ConsumedCapacity,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for GetItemOutput {}

                            impl Hash for GetItemOutput {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        GetItemOutput::GetItemOutput {
                                            Item,
                                            ConsumedCapacity,
                                        } => {
                                            ::std::hash::Hash::hash(Item, _state);
                                            ::std::hash::Hash::hash(ConsumedCapacity, _state)
                                        }
                                    }
                                }
                            }

                            impl Default for GetItemOutput {
                                fn default() -> GetItemOutput {
                                    GetItemOutput::GetItemOutput {
                                        Item: ::std::default::Default::default(),
                                        ConsumedCapacity: ::std::default::Default::default(),
                                    }
                                }
                            }

                            impl AsRef<GetItemOutput> for &GetItemOutput {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            #[derive(PartialEq, Clone)]
                            pub enum GlobalSecondaryIndex {
                                GlobalSecondaryIndex {
                  IndexName: ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                  KeySchema: crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::KeySchema,
                  Projection: ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Projection>,
                  ProvisionedThroughput: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ProvisionedThroughput>>>
                }
              }

                            impl GlobalSecondaryIndex {
                                pub fn IndexName(
                                    &self,
                                ) -> &::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>
                                {
                                    match self {
                                        GlobalSecondaryIndex::GlobalSecondaryIndex {
                                            IndexName,
                                            KeySchema,
                                            Projection,
                                            ProvisionedThroughput,
                                        } => IndexName,
                                    }
                                }
                                pub fn KeySchema(&self) -> &crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::KeySchema{
                                    match self {
                                        GlobalSecondaryIndex::GlobalSecondaryIndex {
                                            IndexName,
                                            KeySchema,
                                            Projection,
                                            ProvisionedThroughput,
                                        } => KeySchema,
                                    }
                                }
                                pub fn Projection(&self) -> &::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Projection>{
                                    match self {
                                        GlobalSecondaryIndex::GlobalSecondaryIndex {
                                            IndexName,
                                            KeySchema,
                                            Projection,
                                            ProvisionedThroughput,
                                        } => Projection,
                                    }
                                }
                                pub fn ProvisionedThroughput(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ProvisionedThroughput>>>{
                                    match self {
                                        GlobalSecondaryIndex::GlobalSecondaryIndex {
                                            IndexName,
                                            KeySchema,
                                            Projection,
                                            ProvisionedThroughput,
                                        } => ProvisionedThroughput,
                                    }
                                }
                            }

                            impl Debug for GlobalSecondaryIndex {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for GlobalSecondaryIndex {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        GlobalSecondaryIndex::GlobalSecondaryIndex {
                                            IndexName,
                                            KeySchema,
                                            Projection,
                                            ProvisionedThroughput,
                                        } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.GlobalSecondaryIndex.GlobalSecondaryIndex(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                IndexName, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                KeySchema, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                Projection, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ProvisionedThroughput,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for GlobalSecondaryIndex {}

                            impl Hash for GlobalSecondaryIndex {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        GlobalSecondaryIndex::GlobalSecondaryIndex {
                                            IndexName,
                                            KeySchema,
                                            Projection,
                                            ProvisionedThroughput,
                                        } => {
                                            ::std::hash::Hash::hash(IndexName, _state);
                                            ::std::hash::Hash::hash(KeySchema, _state);
                                            ::std::hash::Hash::hash(Projection, _state);
                                            ::std::hash::Hash::hash(ProvisionedThroughput, _state)
                                        }
                                    }
                                }
                            }

                            impl Default for GlobalSecondaryIndex {
                                fn default() -> GlobalSecondaryIndex {
                                    GlobalSecondaryIndex::GlobalSecondaryIndex {
                                        IndexName: ::std::default::Default::default(),
                                        KeySchema: ::std::default::Default::default(),
                                        Projection: ::std::default::Default::default(),
                                        ProvisionedThroughput: ::std::default::Default::default(),
                                    }
                                }
                            }

                            impl AsRef<GlobalSecondaryIndex> for &GlobalSecondaryIndex {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            #[derive(PartialEq, Clone)]
                            pub enum GlobalSecondaryIndexDescription {
                                GlobalSecondaryIndexDescription {
                  IndexName: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>,
                  KeySchema: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::KeySchema>>,
                  Projection: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Projection>>>,
                  IndexStatus: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::IndexStatus>>>,
                  Backfilling: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<bool>>,
                  ProvisionedThroughput: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ProvisionedThroughputDescription>>>,
                  IndexSizeBytes: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<i64>>,
                  ItemCount: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<i64>>,
                  IndexArn: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>
                }
              }

                            impl GlobalSecondaryIndexDescription {
                                pub fn IndexName(
                                    &self,
                                ) -> &::std::rc::Rc<
                                    crate::r#_Wrappers_Compile::Option<
                                        ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                    >,
                                > {
                                    match self {
                    GlobalSecondaryIndexDescription::GlobalSecondaryIndexDescription{IndexName, KeySchema, Projection, IndexStatus, Backfilling, ProvisionedThroughput, IndexSizeBytes, ItemCount, IndexArn, } => IndexName,
                  }
                                }
                                pub fn KeySchema(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::KeySchema>>{
                                    match self {
                    GlobalSecondaryIndexDescription::GlobalSecondaryIndexDescription{IndexName, KeySchema, Projection, IndexStatus, Backfilling, ProvisionedThroughput, IndexSizeBytes, ItemCount, IndexArn, } => KeySchema,
                  }
                                }
                                pub fn Projection(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Projection>>>{
                                    match self {
                    GlobalSecondaryIndexDescription::GlobalSecondaryIndexDescription{IndexName, KeySchema, Projection, IndexStatus, Backfilling, ProvisionedThroughput, IndexSizeBytes, ItemCount, IndexArn, } => Projection,
                  }
                                }
                                pub fn IndexStatus(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::IndexStatus>>>{
                                    match self {
                    GlobalSecondaryIndexDescription::GlobalSecondaryIndexDescription{IndexName, KeySchema, Projection, IndexStatus, Backfilling, ProvisionedThroughput, IndexSizeBytes, ItemCount, IndexArn, } => IndexStatus,
                  }
                                }
                                pub fn Backfilling(
                                    &self,
                                ) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<bool>>
                                {
                                    match self {
                    GlobalSecondaryIndexDescription::GlobalSecondaryIndexDescription{IndexName, KeySchema, Projection, IndexStatus, Backfilling, ProvisionedThroughput, IndexSizeBytes, ItemCount, IndexArn, } => Backfilling,
                  }
                                }
                                pub fn ProvisionedThroughput(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ProvisionedThroughputDescription>>>{
                                    match self {
                    GlobalSecondaryIndexDescription::GlobalSecondaryIndexDescription{IndexName, KeySchema, Projection, IndexStatus, Backfilling, ProvisionedThroughput, IndexSizeBytes, ItemCount, IndexArn, } => ProvisionedThroughput,
                  }
                                }
                                pub fn IndexSizeBytes(
                                    &self,
                                ) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<i64>>
                                {
                                    match self {
                    GlobalSecondaryIndexDescription::GlobalSecondaryIndexDescription{IndexName, KeySchema, Projection, IndexStatus, Backfilling, ProvisionedThroughput, IndexSizeBytes, ItemCount, IndexArn, } => IndexSizeBytes,
                  }
                                }
                                pub fn ItemCount(
                                    &self,
                                ) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<i64>>
                                {
                                    match self {
                    GlobalSecondaryIndexDescription::GlobalSecondaryIndexDescription{IndexName, KeySchema, Projection, IndexStatus, Backfilling, ProvisionedThroughput, IndexSizeBytes, ItemCount, IndexArn, } => ItemCount,
                  }
                                }
                                pub fn IndexArn(
                                    &self,
                                ) -> &::std::rc::Rc<
                                    crate::r#_Wrappers_Compile::Option<
                                        ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                    >,
                                > {
                                    match self {
                    GlobalSecondaryIndexDescription::GlobalSecondaryIndexDescription{IndexName, KeySchema, Projection, IndexStatus, Backfilling, ProvisionedThroughput, IndexSizeBytes, ItemCount, IndexArn, } => IndexArn,
                  }
                                }
                            }

                            impl Debug for GlobalSecondaryIndexDescription {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for GlobalSecondaryIndexDescription {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                    GlobalSecondaryIndexDescription::GlobalSecondaryIndexDescription{IndexName, KeySchema, Projection, IndexStatus, Backfilling, ProvisionedThroughput, IndexSizeBytes, ItemCount, IndexArn, } => {
                      write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.GlobalSecondaryIndexDescription.GlobalSecondaryIndexDescription(")?;
                      ::dafny_runtime::DafnyPrint::fmt_print(IndexName, _formatter, false)?;
                      write!(_formatter, ", ")?;
                      ::dafny_runtime::DafnyPrint::fmt_print(KeySchema, _formatter, false)?;
                      write!(_formatter, ", ")?;
                      ::dafny_runtime::DafnyPrint::fmt_print(Projection, _formatter, false)?;
                      write!(_formatter, ", ")?;
                      ::dafny_runtime::DafnyPrint::fmt_print(IndexStatus, _formatter, false)?;
                      write!(_formatter, ", ")?;
                      ::dafny_runtime::DafnyPrint::fmt_print(Backfilling, _formatter, false)?;
                      write!(_formatter, ", ")?;
                      ::dafny_runtime::DafnyPrint::fmt_print(ProvisionedThroughput, _formatter, false)?;
                      write!(_formatter, ", ")?;
                      ::dafny_runtime::DafnyPrint::fmt_print(IndexSizeBytes, _formatter, false)?;
                      write!(_formatter, ", ")?;
                      ::dafny_runtime::DafnyPrint::fmt_print(ItemCount, _formatter, false)?;
                      write!(_formatter, ", ")?;
                      ::dafny_runtime::DafnyPrint::fmt_print(IndexArn, _formatter, false)?;
                      write!(_formatter, ")")?;
                      Ok(())
                    },
                  }
                                }
                            }

                            impl Eq for GlobalSecondaryIndexDescription {}

                            impl Hash for GlobalSecondaryIndexDescription {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                    GlobalSecondaryIndexDescription::GlobalSecondaryIndexDescription{IndexName, KeySchema, Projection, IndexStatus, Backfilling, ProvisionedThroughput, IndexSizeBytes, ItemCount, IndexArn, } => {
                      ::std::hash::Hash::hash(IndexName, _state);
                      ::std::hash::Hash::hash(KeySchema, _state);
                      ::std::hash::Hash::hash(Projection, _state);
                      ::std::hash::Hash::hash(IndexStatus, _state);
                      ::std::hash::Hash::hash(Backfilling, _state);
                      ::std::hash::Hash::hash(ProvisionedThroughput, _state);
                      ::std::hash::Hash::hash(IndexSizeBytes, _state);
                      ::std::hash::Hash::hash(ItemCount, _state);
                      ::std::hash::Hash::hash(IndexArn, _state)
                    },
                  }
                                }
                            }

                            impl Default for GlobalSecondaryIndexDescription {
                                fn default() -> GlobalSecondaryIndexDescription {
                                    GlobalSecondaryIndexDescription::GlobalSecondaryIndexDescription {
                    IndexName: ::std::default::Default::default(),
                    KeySchema: ::std::default::Default::default(),
                    Projection: ::std::default::Default::default(),
                    IndexStatus: ::std::default::Default::default(),
                    Backfilling: ::std::default::Default::default(),
                    ProvisionedThroughput: ::std::default::Default::default(),
                    IndexSizeBytes: ::std::default::Default::default(),
                    ItemCount: ::std::default::Default::default(),
                    IndexArn: ::std::default::Default::default()
                  }
                                }
                            }

                            impl AsRef<GlobalSecondaryIndexDescription> for &GlobalSecondaryIndexDescription {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            pub type IndexName =
                                ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>;

                            #[derive(PartialEq, Clone)]
                            pub enum IndexStatus {
                                CREATING {},
                                UPDATING {},
                                DELETING {},
                                ACTIVE {},
                            }

                            impl IndexStatus {}

                            impl Debug for IndexStatus {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for IndexStatus {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        IndexStatus::CREATING {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.IndexStatus.CREATING")?;
                                            Ok(())
                                        }
                                        IndexStatus::UPDATING {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.IndexStatus.UPDATING")?;
                                            Ok(())
                                        }
                                        IndexStatus::DELETING {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.IndexStatus.DELETING")?;
                                            Ok(())
                                        }
                                        IndexStatus::ACTIVE {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.IndexStatus.ACTIVE")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for IndexStatus {}

                            impl Hash for IndexStatus {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        IndexStatus::CREATING {} => {}
                                        IndexStatus::UPDATING {} => {}
                                        IndexStatus::DELETING {} => {}
                                        IndexStatus::ACTIVE {} => {}
                                    }
                                }
                            }

                            impl Default for IndexStatus {
                                fn default() -> IndexStatus {
                                    IndexStatus::CREATING {}
                                }
                            }

                            impl AsRef<IndexStatus> for &IndexStatus {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            #[derive(PartialEq, Clone)]
                            pub enum ItemCollectionMetrics {
                                ItemCollectionMetrics {
                  ItemCollectionKey: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>>>,
                  SizeEstimateRangeGB: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ItemCollectionSizeEstimateBound>>>
                }
              }

                            impl ItemCollectionMetrics {
                                pub fn ItemCollectionKey(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>>>{
                                    match self {
                                        ItemCollectionMetrics::ItemCollectionMetrics {
                                            ItemCollectionKey,
                                            SizeEstimateRangeGB,
                                        } => ItemCollectionKey,
                                    }
                                }
                                pub fn SizeEstimateRangeGB(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ItemCollectionSizeEstimateBound>>>{
                                    match self {
                                        ItemCollectionMetrics::ItemCollectionMetrics {
                                            ItemCollectionKey,
                                            SizeEstimateRangeGB,
                                        } => SizeEstimateRangeGB,
                                    }
                                }
                            }

                            impl Debug for ItemCollectionMetrics {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for ItemCollectionMetrics {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        ItemCollectionMetrics::ItemCollectionMetrics {
                                            ItemCollectionKey,
                                            SizeEstimateRangeGB,
                                        } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.ItemCollectionMetrics.ItemCollectionMetrics(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ItemCollectionKey,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                SizeEstimateRangeGB,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for ItemCollectionMetrics {}

                            impl Hash for ItemCollectionMetrics {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        ItemCollectionMetrics::ItemCollectionMetrics {
                                            ItemCollectionKey,
                                            SizeEstimateRangeGB,
                                        } => {
                                            ::std::hash::Hash::hash(ItemCollectionKey, _state);
                                            ::std::hash::Hash::hash(SizeEstimateRangeGB, _state)
                                        }
                                    }
                                }
                            }

                            impl Default for ItemCollectionMetrics {
                                fn default() -> ItemCollectionMetrics {
                                    ItemCollectionMetrics::ItemCollectionMetrics {
                                        ItemCollectionKey: ::std::default::Default::default(),
                                        SizeEstimateRangeGB: ::std::default::Default::default(),
                                    }
                                }
                            }

                            impl AsRef<ItemCollectionMetrics> for &ItemCollectionMetrics {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            pub type ItemCollectionSizeEstimateBound =
                                ::dafny_runtime::Sequence<u8>;

                            pub type KeyList = ::dafny_runtime::Sequence<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>>;

                            #[derive(PartialEq, Clone)]
                            pub enum KeysAndAttributes {
                                KeysAndAttributes {
                  Keys: crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::KeyList,
                  AttributesToGet: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeNameList>>,
                  ConsistentRead: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<bool>>,
                  ProjectionExpression: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>,
                  ExpressionAttributeNames: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>>
                }
              }

                            impl KeysAndAttributes {
                                pub fn Keys(&self) -> &crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::KeyList{
                                    match self {
                                        KeysAndAttributes::KeysAndAttributes {
                                            Keys,
                                            AttributesToGet,
                                            ConsistentRead,
                                            ProjectionExpression,
                                            ExpressionAttributeNames,
                                        } => Keys,
                                    }
                                }
                                pub fn AttributesToGet(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeNameList>>{
                                    match self {
                                        KeysAndAttributes::KeysAndAttributes {
                                            Keys,
                                            AttributesToGet,
                                            ConsistentRead,
                                            ProjectionExpression,
                                            ExpressionAttributeNames,
                                        } => AttributesToGet,
                                    }
                                }
                                pub fn ConsistentRead(
                                    &self,
                                ) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<bool>>
                                {
                                    match self {
                                        KeysAndAttributes::KeysAndAttributes {
                                            Keys,
                                            AttributesToGet,
                                            ConsistentRead,
                                            ProjectionExpression,
                                            ExpressionAttributeNames,
                                        } => ConsistentRead,
                                    }
                                }
                                pub fn ProjectionExpression(
                                    &self,
                                ) -> &::std::rc::Rc<
                                    crate::r#_Wrappers_Compile::Option<
                                        ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                    >,
                                > {
                                    match self {
                                        KeysAndAttributes::KeysAndAttributes {
                                            Keys,
                                            AttributesToGet,
                                            ConsistentRead,
                                            ProjectionExpression,
                                            ExpressionAttributeNames,
                                        } => ProjectionExpression,
                                    }
                                }
                                pub fn ExpressionAttributeNames(
                                    &self,
                                ) -> &::std::rc::Rc<
                                    crate::r#_Wrappers_Compile::Option<
                                        ::dafny_runtime::Map<
                                            ::dafny_runtime::Sequence<
                                                ::dafny_runtime::DafnyCharUTF16,
                                            >,
                                            ::dafny_runtime::Sequence<
                                                ::dafny_runtime::DafnyCharUTF16,
                                            >,
                                        >,
                                    >,
                                > {
                                    match self {
                                        KeysAndAttributes::KeysAndAttributes {
                                            Keys,
                                            AttributesToGet,
                                            ConsistentRead,
                                            ProjectionExpression,
                                            ExpressionAttributeNames,
                                        } => ExpressionAttributeNames,
                                    }
                                }
                            }

                            impl Debug for KeysAndAttributes {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for KeysAndAttributes {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        KeysAndAttributes::KeysAndAttributes {
                                            Keys,
                                            AttributesToGet,
                                            ConsistentRead,
                                            ProjectionExpression,
                                            ExpressionAttributeNames,
                                        } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.KeysAndAttributes.KeysAndAttributes(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                Keys, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                AttributesToGet,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ConsistentRead,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ProjectionExpression,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ExpressionAttributeNames,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for KeysAndAttributes {}

                            impl Hash for KeysAndAttributes {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        KeysAndAttributes::KeysAndAttributes {
                                            Keys,
                                            AttributesToGet,
                                            ConsistentRead,
                                            ProjectionExpression,
                                            ExpressionAttributeNames,
                                        } => {
                                            ::std::hash::Hash::hash(Keys, _state);
                                            ::std::hash::Hash::hash(AttributesToGet, _state);
                                            ::std::hash::Hash::hash(ConsistentRead, _state);
                                            ::std::hash::Hash::hash(ProjectionExpression, _state);
                                            ::std::hash::Hash::hash(
                                                ExpressionAttributeNames,
                                                _state,
                                            )
                                        }
                                    }
                                }
                            }

                            impl Default for KeysAndAttributes {
                                fn default() -> KeysAndAttributes {
                                    KeysAndAttributes::KeysAndAttributes {
                                        Keys: ::std::default::Default::default(),
                                        AttributesToGet: ::std::default::Default::default(),
                                        ConsistentRead: ::std::default::Default::default(),
                                        ProjectionExpression: ::std::default::Default::default(),
                                        ExpressionAttributeNames: ::std::default::Default::default(
                                        ),
                                    }
                                }
                            }

                            impl AsRef<KeysAndAttributes> for &KeysAndAttributes {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            pub type KeySchema = ::dafny_runtime::Sequence<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::KeySchemaElement>>;

                            pub type KeySchemaAttributeName =
                                ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>;

                            #[derive(PartialEq, Clone)]
                            pub enum KeySchemaElement {
                                KeySchemaElement {
                  AttributeName: ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                  KeyType: ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::KeyType>
                }
              }

                            impl KeySchemaElement {
                                pub fn AttributeName(
                                    &self,
                                ) -> &::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>
                                {
                                    match self {
                                        KeySchemaElement::KeySchemaElement {
                                            AttributeName,
                                            KeyType,
                                        } => AttributeName,
                                    }
                                }
                                pub fn KeyType(&self) -> &::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::KeyType>{
                                    match self {
                                        KeySchemaElement::KeySchemaElement {
                                            AttributeName,
                                            KeyType,
                                        } => KeyType,
                                    }
                                }
                            }

                            impl Debug for KeySchemaElement {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for KeySchemaElement {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        KeySchemaElement::KeySchemaElement {
                                            AttributeName,
                                            KeyType,
                                        } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.KeySchemaElement.KeySchemaElement(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                AttributeName,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                KeyType, _formatter, false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for KeySchemaElement {}

                            impl Hash for KeySchemaElement {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        KeySchemaElement::KeySchemaElement {
                                            AttributeName,
                                            KeyType,
                                        } => {
                                            ::std::hash::Hash::hash(AttributeName, _state);
                                            ::std::hash::Hash::hash(KeyType, _state)
                                        }
                                    }
                                }
                            }

                            impl Default for KeySchemaElement {
                                fn default() -> KeySchemaElement {
                                    KeySchemaElement::KeySchemaElement {
                                        AttributeName: ::std::default::Default::default(),
                                        KeyType: ::std::default::Default::default(),
                                    }
                                }
                            }

                            impl AsRef<KeySchemaElement> for &KeySchemaElement {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            #[derive(PartialEq, Clone)]
                            pub enum KeyType {
                                HASH {},
                                RANGE {},
                            }

                            impl KeyType {}

                            impl Debug for KeyType {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for KeyType {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        KeyType::HASH {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.KeyType.HASH")?;
                                            Ok(())
                                        }
                                        KeyType::RANGE {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.KeyType.RANGE")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for KeyType {}

                            impl Hash for KeyType {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        KeyType::HASH {} => {}
                                        KeyType::RANGE {} => {}
                                    }
                                }
                            }

                            impl Default for KeyType {
                                fn default() -> KeyType {
                                    KeyType::HASH {}
                                }
                            }

                            impl AsRef<KeyType> for &KeyType {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            #[derive(PartialEq, Clone)]
                            pub enum LocalSecondaryIndex {
                                LocalSecondaryIndex {
                  IndexName: ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                  KeySchema: crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::KeySchema,
                  Projection: ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Projection>
                }
              }

                            impl LocalSecondaryIndex {
                                pub fn IndexName(
                                    &self,
                                ) -> &::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>
                                {
                                    match self {
                                        LocalSecondaryIndex::LocalSecondaryIndex {
                                            IndexName,
                                            KeySchema,
                                            Projection,
                                        } => IndexName,
                                    }
                                }
                                pub fn KeySchema(&self) -> &crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::KeySchema{
                                    match self {
                                        LocalSecondaryIndex::LocalSecondaryIndex {
                                            IndexName,
                                            KeySchema,
                                            Projection,
                                        } => KeySchema,
                                    }
                                }
                                pub fn Projection(&self) -> &::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Projection>{
                                    match self {
                                        LocalSecondaryIndex::LocalSecondaryIndex {
                                            IndexName,
                                            KeySchema,
                                            Projection,
                                        } => Projection,
                                    }
                                }
                            }

                            impl Debug for LocalSecondaryIndex {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for LocalSecondaryIndex {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        LocalSecondaryIndex::LocalSecondaryIndex {
                                            IndexName,
                                            KeySchema,
                                            Projection,
                                        } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.LocalSecondaryIndex.LocalSecondaryIndex(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                IndexName, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                KeySchema, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                Projection, _formatter, false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for LocalSecondaryIndex {}

                            impl Hash for LocalSecondaryIndex {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        LocalSecondaryIndex::LocalSecondaryIndex {
                                            IndexName,
                                            KeySchema,
                                            Projection,
                                        } => {
                                            ::std::hash::Hash::hash(IndexName, _state);
                                            ::std::hash::Hash::hash(KeySchema, _state);
                                            ::std::hash::Hash::hash(Projection, _state)
                                        }
                                    }
                                }
                            }

                            impl Default for LocalSecondaryIndex {
                                fn default() -> LocalSecondaryIndex {
                                    LocalSecondaryIndex::LocalSecondaryIndex {
                                        IndexName: ::std::default::Default::default(),
                                        KeySchema: ::std::default::Default::default(),
                                        Projection: ::std::default::Default::default(),
                                    }
                                }
                            }

                            impl AsRef<LocalSecondaryIndex> for &LocalSecondaryIndex {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            #[derive(PartialEq, Clone)]
                            pub enum LocalSecondaryIndexDescription {
                                LocalSecondaryIndexDescription {
                  IndexName: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>,
                  KeySchema: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::KeySchema>>,
                  Projection: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Projection>>>,
                  IndexSizeBytes: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<i64>>,
                  ItemCount: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<i64>>,
                  IndexArn: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>
                }
              }

                            impl LocalSecondaryIndexDescription {
                                pub fn IndexName(
                                    &self,
                                ) -> &::std::rc::Rc<
                                    crate::r#_Wrappers_Compile::Option<
                                        ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                    >,
                                > {
                                    match self {
                    LocalSecondaryIndexDescription::LocalSecondaryIndexDescription{IndexName, KeySchema, Projection, IndexSizeBytes, ItemCount, IndexArn, } => IndexName,
                  }
                                }
                                pub fn KeySchema(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::KeySchema>>{
                                    match self {
                    LocalSecondaryIndexDescription::LocalSecondaryIndexDescription{IndexName, KeySchema, Projection, IndexSizeBytes, ItemCount, IndexArn, } => KeySchema,
                  }
                                }
                                pub fn Projection(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Projection>>>{
                                    match self {
                    LocalSecondaryIndexDescription::LocalSecondaryIndexDescription{IndexName, KeySchema, Projection, IndexSizeBytes, ItemCount, IndexArn, } => Projection,
                  }
                                }
                                pub fn IndexSizeBytes(
                                    &self,
                                ) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<i64>>
                                {
                                    match self {
                    LocalSecondaryIndexDescription::LocalSecondaryIndexDescription{IndexName, KeySchema, Projection, IndexSizeBytes, ItemCount, IndexArn, } => IndexSizeBytes,
                  }
                                }
                                pub fn ItemCount(
                                    &self,
                                ) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<i64>>
                                {
                                    match self {
                    LocalSecondaryIndexDescription::LocalSecondaryIndexDescription{IndexName, KeySchema, Projection, IndexSizeBytes, ItemCount, IndexArn, } => ItemCount,
                  }
                                }
                                pub fn IndexArn(
                                    &self,
                                ) -> &::std::rc::Rc<
                                    crate::r#_Wrappers_Compile::Option<
                                        ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                    >,
                                > {
                                    match self {
                    LocalSecondaryIndexDescription::LocalSecondaryIndexDescription{IndexName, KeySchema, Projection, IndexSizeBytes, ItemCount, IndexArn, } => IndexArn,
                  }
                                }
                            }

                            impl Debug for LocalSecondaryIndexDescription {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for LocalSecondaryIndexDescription {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                    LocalSecondaryIndexDescription::LocalSecondaryIndexDescription{IndexName, KeySchema, Projection, IndexSizeBytes, ItemCount, IndexArn, } => {
                      write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.LocalSecondaryIndexDescription.LocalSecondaryIndexDescription(")?;
                      ::dafny_runtime::DafnyPrint::fmt_print(IndexName, _formatter, false)?;
                      write!(_formatter, ", ")?;
                      ::dafny_runtime::DafnyPrint::fmt_print(KeySchema, _formatter, false)?;
                      write!(_formatter, ", ")?;
                      ::dafny_runtime::DafnyPrint::fmt_print(Projection, _formatter, false)?;
                      write!(_formatter, ", ")?;
                      ::dafny_runtime::DafnyPrint::fmt_print(IndexSizeBytes, _formatter, false)?;
                      write!(_formatter, ", ")?;
                      ::dafny_runtime::DafnyPrint::fmt_print(ItemCount, _formatter, false)?;
                      write!(_formatter, ", ")?;
                      ::dafny_runtime::DafnyPrint::fmt_print(IndexArn, _formatter, false)?;
                      write!(_formatter, ")")?;
                      Ok(())
                    },
                  }
                                }
                            }

                            impl Eq for LocalSecondaryIndexDescription {}

                            impl Hash for LocalSecondaryIndexDescription {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                    LocalSecondaryIndexDescription::LocalSecondaryIndexDescription{IndexName, KeySchema, Projection, IndexSizeBytes, ItemCount, IndexArn, } => {
                      ::std::hash::Hash::hash(IndexName, _state);
                      ::std::hash::Hash::hash(KeySchema, _state);
                      ::std::hash::Hash::hash(Projection, _state);
                      ::std::hash::Hash::hash(IndexSizeBytes, _state);
                      ::std::hash::Hash::hash(ItemCount, _state);
                      ::std::hash::Hash::hash(IndexArn, _state)
                    },
                  }
                                }
                            }

                            impl Default for LocalSecondaryIndexDescription {
                                fn default() -> LocalSecondaryIndexDescription {
                                    LocalSecondaryIndexDescription::LocalSecondaryIndexDescription {
                                        IndexName: ::std::default::Default::default(),
                                        KeySchema: ::std::default::Default::default(),
                                        Projection: ::std::default::Default::default(),
                                        IndexSizeBytes: ::std::default::Default::default(),
                                        ItemCount: ::std::default::Default::default(),
                                        IndexArn: ::std::default::Default::default(),
                                    }
                                }
                            }

                            impl AsRef<LocalSecondaryIndexDescription> for &LocalSecondaryIndexDescription {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            pub type NonKeyAttributeName =
                                ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>;

                            pub type NonKeyAttributeNameList = ::dafny_runtime::Sequence<
                                ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                            >;

                            pub type NonNegativeLongObject = i64;

                            pub type PositiveIntegerObject = i32;

                            pub type PositiveLongObject = i64;

                            #[derive(PartialEq, Clone)]
                            pub enum Projection {
                                Projection {
                  ProjectionType: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ProjectionType>>>,
                  NonKeyAttributes: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::NonKeyAttributeNameList>>
                }
              }

                            impl Projection {
                                pub fn ProjectionType(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ProjectionType>>>{
                                    match self {
                                        Projection::Projection {
                                            ProjectionType,
                                            NonKeyAttributes,
                                        } => ProjectionType,
                                    }
                                }
                                pub fn NonKeyAttributes(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::NonKeyAttributeNameList>>{
                                    match self {
                                        Projection::Projection {
                                            ProjectionType,
                                            NonKeyAttributes,
                                        } => NonKeyAttributes,
                                    }
                                }
                            }

                            impl Debug for Projection {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for Projection {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        Projection::Projection {
                                            ProjectionType,
                                            NonKeyAttributes,
                                        } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.Projection.Projection(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ProjectionType,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                NonKeyAttributes,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for Projection {}

                            impl Hash for Projection {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        Projection::Projection {
                                            ProjectionType,
                                            NonKeyAttributes,
                                        } => {
                                            ::std::hash::Hash::hash(ProjectionType, _state);
                                            ::std::hash::Hash::hash(NonKeyAttributes, _state)
                                        }
                                    }
                                }
                            }

                            impl Default for Projection {
                                fn default() -> Projection {
                                    Projection::Projection {
                                        ProjectionType: ::std::default::Default::default(),
                                        NonKeyAttributes: ::std::default::Default::default(),
                                    }
                                }
                            }

                            impl AsRef<Projection> for &Projection {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            #[derive(PartialEq, Clone)]
                            pub enum ProjectionType {
                                ALL {},
                                KEYS_ONLY {},
                                INCLUDE {},
                            }

                            impl ProjectionType {}

                            impl Debug for ProjectionType {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for ProjectionType {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        ProjectionType::ALL {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.ProjectionType.ALL")?;
                                            Ok(())
                                        }
                                        ProjectionType::KEYS_ONLY {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.ProjectionType.KEYS__ONLY")?;
                                            Ok(())
                                        }
                                        ProjectionType::INCLUDE {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.ProjectionType.INCLUDE")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for ProjectionType {}

                            impl Hash for ProjectionType {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        ProjectionType::ALL {} => {}
                                        ProjectionType::KEYS_ONLY {} => {}
                                        ProjectionType::INCLUDE {} => {}
                                    }
                                }
                            }

                            impl Default for ProjectionType {
                                fn default() -> ProjectionType {
                                    ProjectionType::ALL {}
                                }
                            }

                            impl AsRef<ProjectionType> for &ProjectionType {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            #[derive(PartialEq, Clone)]
                            pub enum ProvisionedThroughput {
                                ProvisionedThroughput {
                  ReadCapacityUnits: crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::PositiveLongObject,
                  WriteCapacityUnits: crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::PositiveLongObject
                }
              }

                            impl ProvisionedThroughput {
                                pub fn ReadCapacityUnits(&self) -> &crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::PositiveLongObject{
                                    match self {
                                        ProvisionedThroughput::ProvisionedThroughput {
                                            ReadCapacityUnits,
                                            WriteCapacityUnits,
                                        } => ReadCapacityUnits,
                                    }
                                }
                                pub fn WriteCapacityUnits(&self) -> &crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::PositiveLongObject{
                                    match self {
                                        ProvisionedThroughput::ProvisionedThroughput {
                                            ReadCapacityUnits,
                                            WriteCapacityUnits,
                                        } => WriteCapacityUnits,
                                    }
                                }
                            }

                            impl Debug for ProvisionedThroughput {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for ProvisionedThroughput {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        ProvisionedThroughput::ProvisionedThroughput {
                                            ReadCapacityUnits,
                                            WriteCapacityUnits,
                                        } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.ProvisionedThroughput.ProvisionedThroughput(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ReadCapacityUnits,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                WriteCapacityUnits,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for ProvisionedThroughput {}

                            impl Hash for ProvisionedThroughput {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        ProvisionedThroughput::ProvisionedThroughput {
                                            ReadCapacityUnits,
                                            WriteCapacityUnits,
                                        } => {
                                            ::std::hash::Hash::hash(ReadCapacityUnits, _state);
                                            ::std::hash::Hash::hash(WriteCapacityUnits, _state)
                                        }
                                    }
                                }
                            }

                            impl Default for ProvisionedThroughput {
                                fn default() -> ProvisionedThroughput {
                                    ProvisionedThroughput::ProvisionedThroughput {
                                        ReadCapacityUnits: ::std::default::Default::default(),
                                        WriteCapacityUnits: ::std::default::Default::default(),
                                    }
                                }
                            }

                            impl AsRef<ProvisionedThroughput> for &ProvisionedThroughput {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            #[derive(PartialEq, Clone)]
                            pub enum ProvisionedThroughputDescription {
                                ProvisionedThroughputDescription {
                  LastIncreaseDateTime: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>,
                  LastDecreaseDateTime: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>,
                  NumberOfDecreasesToday: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::PositiveLongObject>>,
                  ReadCapacityUnits: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::NonNegativeLongObject>>,
                  WriteCapacityUnits: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::NonNegativeLongObject>>
                }
              }

                            impl ProvisionedThroughputDescription {
                                pub fn LastIncreaseDateTime(
                                    &self,
                                ) -> &::std::rc::Rc<
                                    crate::r#_Wrappers_Compile::Option<
                                        ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                    >,
                                > {
                                    match self {
                    ProvisionedThroughputDescription::ProvisionedThroughputDescription{LastIncreaseDateTime, LastDecreaseDateTime, NumberOfDecreasesToday, ReadCapacityUnits, WriteCapacityUnits, } => LastIncreaseDateTime,
                  }
                                }
                                pub fn LastDecreaseDateTime(
                                    &self,
                                ) -> &::std::rc::Rc<
                                    crate::r#_Wrappers_Compile::Option<
                                        ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                    >,
                                > {
                                    match self {
                    ProvisionedThroughputDescription::ProvisionedThroughputDescription{LastIncreaseDateTime, LastDecreaseDateTime, NumberOfDecreasesToday, ReadCapacityUnits, WriteCapacityUnits, } => LastDecreaseDateTime,
                  }
                                }
                                pub fn NumberOfDecreasesToday(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::PositiveLongObject>>{
                                    match self {
                    ProvisionedThroughputDescription::ProvisionedThroughputDescription{LastIncreaseDateTime, LastDecreaseDateTime, NumberOfDecreasesToday, ReadCapacityUnits, WriteCapacityUnits, } => NumberOfDecreasesToday,
                  }
                                }
                                pub fn ReadCapacityUnits(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::NonNegativeLongObject>>{
                                    match self {
                    ProvisionedThroughputDescription::ProvisionedThroughputDescription{LastIncreaseDateTime, LastDecreaseDateTime, NumberOfDecreasesToday, ReadCapacityUnits, WriteCapacityUnits, } => ReadCapacityUnits,
                  }
                                }
                                pub fn WriteCapacityUnits(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::NonNegativeLongObject>>{
                                    match self {
                    ProvisionedThroughputDescription::ProvisionedThroughputDescription{LastIncreaseDateTime, LastDecreaseDateTime, NumberOfDecreasesToday, ReadCapacityUnits, WriteCapacityUnits, } => WriteCapacityUnits,
                  }
                                }
                            }

                            impl Debug for ProvisionedThroughputDescription {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for ProvisionedThroughputDescription {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                    ProvisionedThroughputDescription::ProvisionedThroughputDescription{LastIncreaseDateTime, LastDecreaseDateTime, NumberOfDecreasesToday, ReadCapacityUnits, WriteCapacityUnits, } => {
                      write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.ProvisionedThroughputDescription.ProvisionedThroughputDescription(")?;
                      ::dafny_runtime::DafnyPrint::fmt_print(LastIncreaseDateTime, _formatter, false)?;
                      write!(_formatter, ", ")?;
                      ::dafny_runtime::DafnyPrint::fmt_print(LastDecreaseDateTime, _formatter, false)?;
                      write!(_formatter, ", ")?;
                      ::dafny_runtime::DafnyPrint::fmt_print(NumberOfDecreasesToday, _formatter, false)?;
                      write!(_formatter, ", ")?;
                      ::dafny_runtime::DafnyPrint::fmt_print(ReadCapacityUnits, _formatter, false)?;
                      write!(_formatter, ", ")?;
                      ::dafny_runtime::DafnyPrint::fmt_print(WriteCapacityUnits, _formatter, false)?;
                      write!(_formatter, ")")?;
                      Ok(())
                    },
                  }
                                }
                            }

                            impl Eq for ProvisionedThroughputDescription {}

                            impl Hash for ProvisionedThroughputDescription {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                    ProvisionedThroughputDescription::ProvisionedThroughputDescription{LastIncreaseDateTime, LastDecreaseDateTime, NumberOfDecreasesToday, ReadCapacityUnits, WriteCapacityUnits, } => {
                      ::std::hash::Hash::hash(LastIncreaseDateTime, _state);
                      ::std::hash::Hash::hash(LastDecreaseDateTime, _state);
                      ::std::hash::Hash::hash(NumberOfDecreasesToday, _state);
                      ::std::hash::Hash::hash(ReadCapacityUnits, _state);
                      ::std::hash::Hash::hash(WriteCapacityUnits, _state)
                    },
                  }
                                }
                            }

                            impl Default for ProvisionedThroughputDescription {
                                fn default() -> ProvisionedThroughputDescription {
                                    ProvisionedThroughputDescription::ProvisionedThroughputDescription {
                    LastIncreaseDateTime: ::std::default::Default::default(),
                    LastDecreaseDateTime: ::std::default::Default::default(),
                    NumberOfDecreasesToday: ::std::default::Default::default(),
                    ReadCapacityUnits: ::std::default::Default::default(),
                    WriteCapacityUnits: ::std::default::Default::default()
                  }
                                }
                            }

                            impl AsRef<ProvisionedThroughputDescription> for &ProvisionedThroughputDescription {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            #[derive(PartialEq, Clone)]
                            pub enum ProvisionedThroughputOverride {
                                ProvisionedThroughputOverride {
                  ReadCapacityUnits: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::PositiveLongObject>>
                }
              }

                            impl ProvisionedThroughputOverride {
                                pub fn ReadCapacityUnits(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::PositiveLongObject>>{
                                    match self {
                    ProvisionedThroughputOverride::ProvisionedThroughputOverride{ReadCapacityUnits, } => ReadCapacityUnits,
                  }
                                }
                            }

                            impl Debug for ProvisionedThroughputOverride {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for ProvisionedThroughputOverride {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                    ProvisionedThroughputOverride::ProvisionedThroughputOverride{ReadCapacityUnits, } => {
                      write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.ProvisionedThroughputOverride.ProvisionedThroughputOverride(")?;
                      ::dafny_runtime::DafnyPrint::fmt_print(ReadCapacityUnits, _formatter, false)?;
                      write!(_formatter, ")")?;
                      Ok(())
                    },
                  }
                                }
                            }

                            impl Eq for ProvisionedThroughputOverride {}

                            impl Hash for ProvisionedThroughputOverride {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                    ProvisionedThroughputOverride::ProvisionedThroughputOverride{ReadCapacityUnits, } => {
                      ::std::hash::Hash::hash(ReadCapacityUnits, _state)
                    },
                  }
                                }
                            }

                            impl Default for ProvisionedThroughputOverride {
                                fn default() -> ProvisionedThroughputOverride {
                                    ProvisionedThroughputOverride::ProvisionedThroughputOverride {
                                        ReadCapacityUnits: ::std::default::Default::default(),
                                    }
                                }
                            }

                            impl AsRef<ProvisionedThroughputOverride> for &ProvisionedThroughputOverride {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            #[derive(PartialEq, Clone)]
                            pub enum Put {
                                Put {
                  Item: ::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>,
                  TableName: ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                  ConditionExpression: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>,
                  ExpressionAttributeNames: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>>,
                  ExpressionAttributeValues: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>>>,
                  ReturnValuesOnConditionCheckFailure: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ReturnValuesOnConditionCheckFailure>>>
                }
              }

                            impl Put {
                                pub fn Item(&self) -> &::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>{
                                    match self {
                                        Put::Put {
                                            Item,
                                            TableName,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                            ReturnValuesOnConditionCheckFailure,
                                        } => Item,
                                    }
                                }
                                pub fn TableName(
                                    &self,
                                ) -> &::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>
                                {
                                    match self {
                                        Put::Put {
                                            Item,
                                            TableName,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                            ReturnValuesOnConditionCheckFailure,
                                        } => TableName,
                                    }
                                }
                                pub fn ConditionExpression(
                                    &self,
                                ) -> &::std::rc::Rc<
                                    crate::r#_Wrappers_Compile::Option<
                                        ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                    >,
                                > {
                                    match self {
                                        Put::Put {
                                            Item,
                                            TableName,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                            ReturnValuesOnConditionCheckFailure,
                                        } => ConditionExpression,
                                    }
                                }
                                pub fn ExpressionAttributeNames(
                                    &self,
                                ) -> &::std::rc::Rc<
                                    crate::r#_Wrappers_Compile::Option<
                                        ::dafny_runtime::Map<
                                            ::dafny_runtime::Sequence<
                                                ::dafny_runtime::DafnyCharUTF16,
                                            >,
                                            ::dafny_runtime::Sequence<
                                                ::dafny_runtime::DafnyCharUTF16,
                                            >,
                                        >,
                                    >,
                                > {
                                    match self {
                                        Put::Put {
                                            Item,
                                            TableName,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                            ReturnValuesOnConditionCheckFailure,
                                        } => ExpressionAttributeNames,
                                    }
                                }
                                pub fn ExpressionAttributeValues(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>>>{
                                    match self {
                                        Put::Put {
                                            Item,
                                            TableName,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                            ReturnValuesOnConditionCheckFailure,
                                        } => ExpressionAttributeValues,
                                    }
                                }
                                pub fn ReturnValuesOnConditionCheckFailure(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ReturnValuesOnConditionCheckFailure>>>{
                                    match self {
                                        Put::Put {
                                            Item,
                                            TableName,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                            ReturnValuesOnConditionCheckFailure,
                                        } => ReturnValuesOnConditionCheckFailure,
                                    }
                                }
                            }

                            impl Debug for Put {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for Put {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        Put::Put {
                                            Item,
                                            TableName,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                            ReturnValuesOnConditionCheckFailure,
                                        } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.Put.Put(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                Item, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                TableName, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ConditionExpression,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ExpressionAttributeNames,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ExpressionAttributeValues,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ReturnValuesOnConditionCheckFailure,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for Put {}

                            impl Hash for Put {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        Put::Put {
                                            Item,
                                            TableName,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                            ReturnValuesOnConditionCheckFailure,
                                        } => {
                                            ::std::hash::Hash::hash(Item, _state);
                                            ::std::hash::Hash::hash(TableName, _state);
                                            ::std::hash::Hash::hash(ConditionExpression, _state);
                                            ::std::hash::Hash::hash(
                                                ExpressionAttributeNames,
                                                _state,
                                            );
                                            ::std::hash::Hash::hash(
                                                ExpressionAttributeValues,
                                                _state,
                                            );
                                            ::std::hash::Hash::hash(
                                                ReturnValuesOnConditionCheckFailure,
                                                _state,
                                            )
                                        }
                                    }
                                }
                            }

                            impl Default for Put {
                                fn default() -> Put {
                                    Put::Put {
                                        Item: ::std::default::Default::default(),
                                        TableName: ::std::default::Default::default(),
                                        ConditionExpression: ::std::default::Default::default(),
                                        ExpressionAttributeNames: ::std::default::Default::default(
                                        ),
                                        ExpressionAttributeValues: ::std::default::Default::default(
                                        ),
                                        ReturnValuesOnConditionCheckFailure:
                                            ::std::default::Default::default(),
                                    }
                                }
                            }

                            impl AsRef<Put> for &Put {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            #[derive(PartialEq, Clone)]
                            pub enum PutItemInput {
                                PutItemInput {
                  TableName: ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                  Item: ::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>,
                  Expected: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ExpectedAttributeValue>>>>,
                  ReturnValues: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ReturnValue>>>,
                  ReturnConsumedCapacity: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ReturnConsumedCapacity>>>,
                  ReturnItemCollectionMetrics: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ReturnItemCollectionMetrics>>>,
                  ConditionalOperator: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ConditionalOperator>>>,
                  ConditionExpression: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>,
                  ExpressionAttributeNames: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>>,
                  ExpressionAttributeValues: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>>>
                }
              }

                            impl PutItemInput {
                                pub fn TableName(
                                    &self,
                                ) -> &::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>
                                {
                                    match self {
                                        PutItemInput::PutItemInput {
                                            TableName,
                                            Item,
                                            Expected,
                                            ReturnValues,
                                            ReturnConsumedCapacity,
                                            ReturnItemCollectionMetrics,
                                            ConditionalOperator,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                        } => TableName,
                                    }
                                }
                                pub fn Item(&self) -> &::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>{
                                    match self {
                                        PutItemInput::PutItemInput {
                                            TableName,
                                            Item,
                                            Expected,
                                            ReturnValues,
                                            ReturnConsumedCapacity,
                                            ReturnItemCollectionMetrics,
                                            ConditionalOperator,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                        } => Item,
                                    }
                                }
                                pub fn Expected(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ExpectedAttributeValue>>>>{
                                    match self {
                                        PutItemInput::PutItemInput {
                                            TableName,
                                            Item,
                                            Expected,
                                            ReturnValues,
                                            ReturnConsumedCapacity,
                                            ReturnItemCollectionMetrics,
                                            ConditionalOperator,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                        } => Expected,
                                    }
                                }
                                pub fn ReturnValues(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ReturnValue>>>{
                                    match self {
                                        PutItemInput::PutItemInput {
                                            TableName,
                                            Item,
                                            Expected,
                                            ReturnValues,
                                            ReturnConsumedCapacity,
                                            ReturnItemCollectionMetrics,
                                            ConditionalOperator,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                        } => ReturnValues,
                                    }
                                }
                                pub fn ReturnConsumedCapacity(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ReturnConsumedCapacity>>>{
                                    match self {
                                        PutItemInput::PutItemInput {
                                            TableName,
                                            Item,
                                            Expected,
                                            ReturnValues,
                                            ReturnConsumedCapacity,
                                            ReturnItemCollectionMetrics,
                                            ConditionalOperator,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                        } => ReturnConsumedCapacity,
                                    }
                                }
                                pub fn ReturnItemCollectionMetrics(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ReturnItemCollectionMetrics>>>{
                                    match self {
                                        PutItemInput::PutItemInput {
                                            TableName,
                                            Item,
                                            Expected,
                                            ReturnValues,
                                            ReturnConsumedCapacity,
                                            ReturnItemCollectionMetrics,
                                            ConditionalOperator,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                        } => ReturnItemCollectionMetrics,
                                    }
                                }
                                pub fn ConditionalOperator(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ConditionalOperator>>>{
                                    match self {
                                        PutItemInput::PutItemInput {
                                            TableName,
                                            Item,
                                            Expected,
                                            ReturnValues,
                                            ReturnConsumedCapacity,
                                            ReturnItemCollectionMetrics,
                                            ConditionalOperator,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                        } => ConditionalOperator,
                                    }
                                }
                                pub fn ConditionExpression(
                                    &self,
                                ) -> &::std::rc::Rc<
                                    crate::r#_Wrappers_Compile::Option<
                                        ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                    >,
                                > {
                                    match self {
                                        PutItemInput::PutItemInput {
                                            TableName,
                                            Item,
                                            Expected,
                                            ReturnValues,
                                            ReturnConsumedCapacity,
                                            ReturnItemCollectionMetrics,
                                            ConditionalOperator,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                        } => ConditionExpression,
                                    }
                                }
                                pub fn ExpressionAttributeNames(
                                    &self,
                                ) -> &::std::rc::Rc<
                                    crate::r#_Wrappers_Compile::Option<
                                        ::dafny_runtime::Map<
                                            ::dafny_runtime::Sequence<
                                                ::dafny_runtime::DafnyCharUTF16,
                                            >,
                                            ::dafny_runtime::Sequence<
                                                ::dafny_runtime::DafnyCharUTF16,
                                            >,
                                        >,
                                    >,
                                > {
                                    match self {
                                        PutItemInput::PutItemInput {
                                            TableName,
                                            Item,
                                            Expected,
                                            ReturnValues,
                                            ReturnConsumedCapacity,
                                            ReturnItemCollectionMetrics,
                                            ConditionalOperator,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                        } => ExpressionAttributeNames,
                                    }
                                }
                                pub fn ExpressionAttributeValues(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>>>{
                                    match self {
                                        PutItemInput::PutItemInput {
                                            TableName,
                                            Item,
                                            Expected,
                                            ReturnValues,
                                            ReturnConsumedCapacity,
                                            ReturnItemCollectionMetrics,
                                            ConditionalOperator,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                        } => ExpressionAttributeValues,
                                    }
                                }
                            }

                            impl Debug for PutItemInput {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for PutItemInput {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        PutItemInput::PutItemInput {
                                            TableName,
                                            Item,
                                            Expected,
                                            ReturnValues,
                                            ReturnConsumedCapacity,
                                            ReturnItemCollectionMetrics,
                                            ConditionalOperator,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                        } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.PutItemInput.PutItemInput(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                TableName, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                Item, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                Expected, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ReturnValues,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ReturnConsumedCapacity,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ReturnItemCollectionMetrics,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ConditionalOperator,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ConditionExpression,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ExpressionAttributeNames,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ExpressionAttributeValues,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for PutItemInput {}

                            impl Hash for PutItemInput {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        PutItemInput::PutItemInput {
                                            TableName,
                                            Item,
                                            Expected,
                                            ReturnValues,
                                            ReturnConsumedCapacity,
                                            ReturnItemCollectionMetrics,
                                            ConditionalOperator,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                        } => {
                                            ::std::hash::Hash::hash(TableName, _state);
                                            ::std::hash::Hash::hash(Item, _state);
                                            ::std::hash::Hash::hash(Expected, _state);
                                            ::std::hash::Hash::hash(ReturnValues, _state);
                                            ::std::hash::Hash::hash(ReturnConsumedCapacity, _state);
                                            ::std::hash::Hash::hash(
                                                ReturnItemCollectionMetrics,
                                                _state,
                                            );
                                            ::std::hash::Hash::hash(ConditionalOperator, _state);
                                            ::std::hash::Hash::hash(ConditionExpression, _state);
                                            ::std::hash::Hash::hash(
                                                ExpressionAttributeNames,
                                                _state,
                                            );
                                            ::std::hash::Hash::hash(
                                                ExpressionAttributeValues,
                                                _state,
                                            )
                                        }
                                    }
                                }
                            }

                            impl Default for PutItemInput {
                                fn default() -> PutItemInput {
                                    PutItemInput::PutItemInput {
                                        TableName: ::std::default::Default::default(),
                                        Item: ::std::default::Default::default(),
                                        Expected: ::std::default::Default::default(),
                                        ReturnValues: ::std::default::Default::default(),
                                        ReturnConsumedCapacity: ::std::default::Default::default(),
                                        ReturnItemCollectionMetrics:
                                            ::std::default::Default::default(),
                                        ConditionalOperator: ::std::default::Default::default(),
                                        ConditionExpression: ::std::default::Default::default(),
                                        ExpressionAttributeNames: ::std::default::Default::default(
                                        ),
                                        ExpressionAttributeValues: ::std::default::Default::default(
                                        ),
                                    }
                                }
                            }

                            impl AsRef<PutItemInput> for &PutItemInput {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            #[derive(PartialEq, Clone)]
                            pub enum PutItemOutput {
                                PutItemOutput {
                  Attributes: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>>>,
                  ConsumedCapacity: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ConsumedCapacity>>>,
                  ItemCollectionMetrics: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ItemCollectionMetrics>>>
                }
              }

                            impl PutItemOutput {
                                pub fn Attributes(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>>>{
                                    match self {
                                        PutItemOutput::PutItemOutput {
                                            Attributes,
                                            ConsumedCapacity,
                                            ItemCollectionMetrics,
                                        } => Attributes,
                                    }
                                }
                                pub fn ConsumedCapacity(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ConsumedCapacity>>>{
                                    match self {
                                        PutItemOutput::PutItemOutput {
                                            Attributes,
                                            ConsumedCapacity,
                                            ItemCollectionMetrics,
                                        } => ConsumedCapacity,
                                    }
                                }
                                pub fn ItemCollectionMetrics(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ItemCollectionMetrics>>>{
                                    match self {
                                        PutItemOutput::PutItemOutput {
                                            Attributes,
                                            ConsumedCapacity,
                                            ItemCollectionMetrics,
                                        } => ItemCollectionMetrics,
                                    }
                                }
                            }

                            impl Debug for PutItemOutput {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for PutItemOutput {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        PutItemOutput::PutItemOutput {
                                            Attributes,
                                            ConsumedCapacity,
                                            ItemCollectionMetrics,
                                        } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.PutItemOutput.PutItemOutput(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                Attributes, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ConsumedCapacity,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ItemCollectionMetrics,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for PutItemOutput {}

                            impl Hash for PutItemOutput {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        PutItemOutput::PutItemOutput {
                                            Attributes,
                                            ConsumedCapacity,
                                            ItemCollectionMetrics,
                                        } => {
                                            ::std::hash::Hash::hash(Attributes, _state);
                                            ::std::hash::Hash::hash(ConsumedCapacity, _state);
                                            ::std::hash::Hash::hash(ItemCollectionMetrics, _state)
                                        }
                                    }
                                }
                            }

                            impl Default for PutItemOutput {
                                fn default() -> PutItemOutput {
                                    PutItemOutput::PutItemOutput {
                                        Attributes: ::std::default::Default::default(),
                                        ConsumedCapacity: ::std::default::Default::default(),
                                        ItemCollectionMetrics: ::std::default::Default::default(),
                                    }
                                }
                            }

                            impl AsRef<PutItemOutput> for &PutItemOutput {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            #[derive(PartialEq, Clone)]
                            pub enum QueryInput {
                                QueryInput {
                  TableName: ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                  IndexName: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>,
                  Select: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Select>>>,
                  AttributesToGet: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeNameList>>,
                  Limit: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::PositiveIntegerObject>>,
                  ConsistentRead: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<bool>>,
                  KeyConditions: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Condition>>>>,
                  QueryFilter: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Condition>>>>,
                  ConditionalOperator: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ConditionalOperator>>>,
                  ScanIndexForward: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<bool>>,
                  ExclusiveStartKey: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>>>,
                  ReturnConsumedCapacity: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ReturnConsumedCapacity>>>,
                  ProjectionExpression: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>,
                  FilterExpression: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>,
                  KeyConditionExpression: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>,
                  ExpressionAttributeNames: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>>,
                  ExpressionAttributeValues: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>>>
                }
              }

                            impl QueryInput {
                                pub fn TableName(
                                    &self,
                                ) -> &::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>
                                {
                                    match self {
                                        QueryInput::QueryInput {
                                            TableName,
                                            IndexName,
                                            Select,
                                            AttributesToGet,
                                            Limit,
                                            ConsistentRead,
                                            KeyConditions,
                                            QueryFilter,
                                            ConditionalOperator,
                                            ScanIndexForward,
                                            ExclusiveStartKey,
                                            ReturnConsumedCapacity,
                                            ProjectionExpression,
                                            FilterExpression,
                                            KeyConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                        } => TableName,
                                    }
                                }
                                pub fn IndexName(
                                    &self,
                                ) -> &::std::rc::Rc<
                                    crate::r#_Wrappers_Compile::Option<
                                        ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                    >,
                                > {
                                    match self {
                                        QueryInput::QueryInput {
                                            TableName,
                                            IndexName,
                                            Select,
                                            AttributesToGet,
                                            Limit,
                                            ConsistentRead,
                                            KeyConditions,
                                            QueryFilter,
                                            ConditionalOperator,
                                            ScanIndexForward,
                                            ExclusiveStartKey,
                                            ReturnConsumedCapacity,
                                            ProjectionExpression,
                                            FilterExpression,
                                            KeyConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                        } => IndexName,
                                    }
                                }
                                pub fn Select(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Select>>>{
                                    match self {
                                        QueryInput::QueryInput {
                                            TableName,
                                            IndexName,
                                            Select,
                                            AttributesToGet,
                                            Limit,
                                            ConsistentRead,
                                            KeyConditions,
                                            QueryFilter,
                                            ConditionalOperator,
                                            ScanIndexForward,
                                            ExclusiveStartKey,
                                            ReturnConsumedCapacity,
                                            ProjectionExpression,
                                            FilterExpression,
                                            KeyConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                        } => Select,
                                    }
                                }
                                pub fn AttributesToGet(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeNameList>>{
                                    match self {
                                        QueryInput::QueryInput {
                                            TableName,
                                            IndexName,
                                            Select,
                                            AttributesToGet,
                                            Limit,
                                            ConsistentRead,
                                            KeyConditions,
                                            QueryFilter,
                                            ConditionalOperator,
                                            ScanIndexForward,
                                            ExclusiveStartKey,
                                            ReturnConsumedCapacity,
                                            ProjectionExpression,
                                            FilterExpression,
                                            KeyConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                        } => AttributesToGet,
                                    }
                                }
                                pub fn Limit(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::PositiveIntegerObject>>{
                                    match self {
                                        QueryInput::QueryInput {
                                            TableName,
                                            IndexName,
                                            Select,
                                            AttributesToGet,
                                            Limit,
                                            ConsistentRead,
                                            KeyConditions,
                                            QueryFilter,
                                            ConditionalOperator,
                                            ScanIndexForward,
                                            ExclusiveStartKey,
                                            ReturnConsumedCapacity,
                                            ProjectionExpression,
                                            FilterExpression,
                                            KeyConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                        } => Limit,
                                    }
                                }
                                pub fn ConsistentRead(
                                    &self,
                                ) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<bool>>
                                {
                                    match self {
                                        QueryInput::QueryInput {
                                            TableName,
                                            IndexName,
                                            Select,
                                            AttributesToGet,
                                            Limit,
                                            ConsistentRead,
                                            KeyConditions,
                                            QueryFilter,
                                            ConditionalOperator,
                                            ScanIndexForward,
                                            ExclusiveStartKey,
                                            ReturnConsumedCapacity,
                                            ProjectionExpression,
                                            FilterExpression,
                                            KeyConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                        } => ConsistentRead,
                                    }
                                }
                                pub fn KeyConditions(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Condition>>>>{
                                    match self {
                                        QueryInput::QueryInput {
                                            TableName,
                                            IndexName,
                                            Select,
                                            AttributesToGet,
                                            Limit,
                                            ConsistentRead,
                                            KeyConditions,
                                            QueryFilter,
                                            ConditionalOperator,
                                            ScanIndexForward,
                                            ExclusiveStartKey,
                                            ReturnConsumedCapacity,
                                            ProjectionExpression,
                                            FilterExpression,
                                            KeyConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                        } => KeyConditions,
                                    }
                                }
                                pub fn QueryFilter(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Condition>>>>{
                                    match self {
                                        QueryInput::QueryInput {
                                            TableName,
                                            IndexName,
                                            Select,
                                            AttributesToGet,
                                            Limit,
                                            ConsistentRead,
                                            KeyConditions,
                                            QueryFilter,
                                            ConditionalOperator,
                                            ScanIndexForward,
                                            ExclusiveStartKey,
                                            ReturnConsumedCapacity,
                                            ProjectionExpression,
                                            FilterExpression,
                                            KeyConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                        } => QueryFilter,
                                    }
                                }
                                pub fn ConditionalOperator(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ConditionalOperator>>>{
                                    match self {
                                        QueryInput::QueryInput {
                                            TableName,
                                            IndexName,
                                            Select,
                                            AttributesToGet,
                                            Limit,
                                            ConsistentRead,
                                            KeyConditions,
                                            QueryFilter,
                                            ConditionalOperator,
                                            ScanIndexForward,
                                            ExclusiveStartKey,
                                            ReturnConsumedCapacity,
                                            ProjectionExpression,
                                            FilterExpression,
                                            KeyConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                        } => ConditionalOperator,
                                    }
                                }
                                pub fn ScanIndexForward(
                                    &self,
                                ) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<bool>>
                                {
                                    match self {
                                        QueryInput::QueryInput {
                                            TableName,
                                            IndexName,
                                            Select,
                                            AttributesToGet,
                                            Limit,
                                            ConsistentRead,
                                            KeyConditions,
                                            QueryFilter,
                                            ConditionalOperator,
                                            ScanIndexForward,
                                            ExclusiveStartKey,
                                            ReturnConsumedCapacity,
                                            ProjectionExpression,
                                            FilterExpression,
                                            KeyConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                        } => ScanIndexForward,
                                    }
                                }
                                pub fn ExclusiveStartKey(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>>>{
                                    match self {
                                        QueryInput::QueryInput {
                                            TableName,
                                            IndexName,
                                            Select,
                                            AttributesToGet,
                                            Limit,
                                            ConsistentRead,
                                            KeyConditions,
                                            QueryFilter,
                                            ConditionalOperator,
                                            ScanIndexForward,
                                            ExclusiveStartKey,
                                            ReturnConsumedCapacity,
                                            ProjectionExpression,
                                            FilterExpression,
                                            KeyConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                        } => ExclusiveStartKey,
                                    }
                                }
                                pub fn ReturnConsumedCapacity(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ReturnConsumedCapacity>>>{
                                    match self {
                                        QueryInput::QueryInput {
                                            TableName,
                                            IndexName,
                                            Select,
                                            AttributesToGet,
                                            Limit,
                                            ConsistentRead,
                                            KeyConditions,
                                            QueryFilter,
                                            ConditionalOperator,
                                            ScanIndexForward,
                                            ExclusiveStartKey,
                                            ReturnConsumedCapacity,
                                            ProjectionExpression,
                                            FilterExpression,
                                            KeyConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                        } => ReturnConsumedCapacity,
                                    }
                                }
                                pub fn ProjectionExpression(
                                    &self,
                                ) -> &::std::rc::Rc<
                                    crate::r#_Wrappers_Compile::Option<
                                        ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                    >,
                                > {
                                    match self {
                                        QueryInput::QueryInput {
                                            TableName,
                                            IndexName,
                                            Select,
                                            AttributesToGet,
                                            Limit,
                                            ConsistentRead,
                                            KeyConditions,
                                            QueryFilter,
                                            ConditionalOperator,
                                            ScanIndexForward,
                                            ExclusiveStartKey,
                                            ReturnConsumedCapacity,
                                            ProjectionExpression,
                                            FilterExpression,
                                            KeyConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                        } => ProjectionExpression,
                                    }
                                }
                                pub fn FilterExpression(
                                    &self,
                                ) -> &::std::rc::Rc<
                                    crate::r#_Wrappers_Compile::Option<
                                        ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                    >,
                                > {
                                    match self {
                                        QueryInput::QueryInput {
                                            TableName,
                                            IndexName,
                                            Select,
                                            AttributesToGet,
                                            Limit,
                                            ConsistentRead,
                                            KeyConditions,
                                            QueryFilter,
                                            ConditionalOperator,
                                            ScanIndexForward,
                                            ExclusiveStartKey,
                                            ReturnConsumedCapacity,
                                            ProjectionExpression,
                                            FilterExpression,
                                            KeyConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                        } => FilterExpression,
                                    }
                                }
                                pub fn KeyConditionExpression(
                                    &self,
                                ) -> &::std::rc::Rc<
                                    crate::r#_Wrappers_Compile::Option<
                                        ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                    >,
                                > {
                                    match self {
                                        QueryInput::QueryInput {
                                            TableName,
                                            IndexName,
                                            Select,
                                            AttributesToGet,
                                            Limit,
                                            ConsistentRead,
                                            KeyConditions,
                                            QueryFilter,
                                            ConditionalOperator,
                                            ScanIndexForward,
                                            ExclusiveStartKey,
                                            ReturnConsumedCapacity,
                                            ProjectionExpression,
                                            FilterExpression,
                                            KeyConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                        } => KeyConditionExpression,
                                    }
                                }
                                pub fn ExpressionAttributeNames(
                                    &self,
                                ) -> &::std::rc::Rc<
                                    crate::r#_Wrappers_Compile::Option<
                                        ::dafny_runtime::Map<
                                            ::dafny_runtime::Sequence<
                                                ::dafny_runtime::DafnyCharUTF16,
                                            >,
                                            ::dafny_runtime::Sequence<
                                                ::dafny_runtime::DafnyCharUTF16,
                                            >,
                                        >,
                                    >,
                                > {
                                    match self {
                                        QueryInput::QueryInput {
                                            TableName,
                                            IndexName,
                                            Select,
                                            AttributesToGet,
                                            Limit,
                                            ConsistentRead,
                                            KeyConditions,
                                            QueryFilter,
                                            ConditionalOperator,
                                            ScanIndexForward,
                                            ExclusiveStartKey,
                                            ReturnConsumedCapacity,
                                            ProjectionExpression,
                                            FilterExpression,
                                            KeyConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                        } => ExpressionAttributeNames,
                                    }
                                }
                                pub fn ExpressionAttributeValues(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>>>{
                                    match self {
                                        QueryInput::QueryInput {
                                            TableName,
                                            IndexName,
                                            Select,
                                            AttributesToGet,
                                            Limit,
                                            ConsistentRead,
                                            KeyConditions,
                                            QueryFilter,
                                            ConditionalOperator,
                                            ScanIndexForward,
                                            ExclusiveStartKey,
                                            ReturnConsumedCapacity,
                                            ProjectionExpression,
                                            FilterExpression,
                                            KeyConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                        } => ExpressionAttributeValues,
                                    }
                                }
                            }

                            impl Debug for QueryInput {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for QueryInput {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        QueryInput::QueryInput {
                                            TableName,
                                            IndexName,
                                            Select,
                                            AttributesToGet,
                                            Limit,
                                            ConsistentRead,
                                            KeyConditions,
                                            QueryFilter,
                                            ConditionalOperator,
                                            ScanIndexForward,
                                            ExclusiveStartKey,
                                            ReturnConsumedCapacity,
                                            ProjectionExpression,
                                            FilterExpression,
                                            KeyConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                        } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.QueryInput.QueryInput(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                TableName, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                IndexName, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                Select, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                AttributesToGet,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                Limit, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ConsistentRead,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                KeyConditions,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                QueryFilter,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ConditionalOperator,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ScanIndexForward,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ExclusiveStartKey,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ReturnConsumedCapacity,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ProjectionExpression,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                FilterExpression,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                KeyConditionExpression,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ExpressionAttributeNames,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ExpressionAttributeValues,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for QueryInput {}

                            impl Hash for QueryInput {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        QueryInput::QueryInput {
                                            TableName,
                                            IndexName,
                                            Select,
                                            AttributesToGet,
                                            Limit,
                                            ConsistentRead,
                                            KeyConditions,
                                            QueryFilter,
                                            ConditionalOperator,
                                            ScanIndexForward,
                                            ExclusiveStartKey,
                                            ReturnConsumedCapacity,
                                            ProjectionExpression,
                                            FilterExpression,
                                            KeyConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                        } => {
                                            ::std::hash::Hash::hash(TableName, _state);
                                            ::std::hash::Hash::hash(IndexName, _state);
                                            ::std::hash::Hash::hash(Select, _state);
                                            ::std::hash::Hash::hash(AttributesToGet, _state);
                                            ::std::hash::Hash::hash(Limit, _state);
                                            ::std::hash::Hash::hash(ConsistentRead, _state);
                                            ::std::hash::Hash::hash(KeyConditions, _state);
                                            ::std::hash::Hash::hash(QueryFilter, _state);
                                            ::std::hash::Hash::hash(ConditionalOperator, _state);
                                            ::std::hash::Hash::hash(ScanIndexForward, _state);
                                            ::std::hash::Hash::hash(ExclusiveStartKey, _state);
                                            ::std::hash::Hash::hash(ReturnConsumedCapacity, _state);
                                            ::std::hash::Hash::hash(ProjectionExpression, _state);
                                            ::std::hash::Hash::hash(FilterExpression, _state);
                                            ::std::hash::Hash::hash(KeyConditionExpression, _state);
                                            ::std::hash::Hash::hash(
                                                ExpressionAttributeNames,
                                                _state,
                                            );
                                            ::std::hash::Hash::hash(
                                                ExpressionAttributeValues,
                                                _state,
                                            )
                                        }
                                    }
                                }
                            }

                            impl Default for QueryInput {
                                fn default() -> QueryInput {
                                    QueryInput::QueryInput {
                                        TableName: ::std::default::Default::default(),
                                        IndexName: ::std::default::Default::default(),
                                        Select: ::std::default::Default::default(),
                                        AttributesToGet: ::std::default::Default::default(),
                                        Limit: ::std::default::Default::default(),
                                        ConsistentRead: ::std::default::Default::default(),
                                        KeyConditions: ::std::default::Default::default(),
                                        QueryFilter: ::std::default::Default::default(),
                                        ConditionalOperator: ::std::default::Default::default(),
                                        ScanIndexForward: ::std::default::Default::default(),
                                        ExclusiveStartKey: ::std::default::Default::default(),
                                        ReturnConsumedCapacity: ::std::default::Default::default(),
                                        ProjectionExpression: ::std::default::Default::default(),
                                        FilterExpression: ::std::default::Default::default(),
                                        KeyConditionExpression: ::std::default::Default::default(),
                                        ExpressionAttributeNames: ::std::default::Default::default(
                                        ),
                                        ExpressionAttributeValues: ::std::default::Default::default(
                                        ),
                                    }
                                }
                            }

                            impl AsRef<QueryInput> for &QueryInput {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            #[derive(PartialEq, Clone)]
                            pub enum QueryOutput {
                                QueryOutput {
                  Items: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>>>>,
                  Count: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<i32>>,
                  ScannedCount: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<i32>>,
                  LastEvaluatedKey: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>>>,
                  ConsumedCapacity: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ConsumedCapacity>>>
                }
              }

                            impl QueryOutput {
                                pub fn Items(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>>>>{
                                    match self {
                                        QueryOutput::QueryOutput {
                                            Items,
                                            Count,
                                            ScannedCount,
                                            LastEvaluatedKey,
                                            ConsumedCapacity,
                                        } => Items,
                                    }
                                }
                                pub fn Count(
                                    &self,
                                ) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<i32>>
                                {
                                    match self {
                                        QueryOutput::QueryOutput {
                                            Items,
                                            Count,
                                            ScannedCount,
                                            LastEvaluatedKey,
                                            ConsumedCapacity,
                                        } => Count,
                                    }
                                }
                                pub fn ScannedCount(
                                    &self,
                                ) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<i32>>
                                {
                                    match self {
                                        QueryOutput::QueryOutput {
                                            Items,
                                            Count,
                                            ScannedCount,
                                            LastEvaluatedKey,
                                            ConsumedCapacity,
                                        } => ScannedCount,
                                    }
                                }
                                pub fn LastEvaluatedKey(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>>>{
                                    match self {
                                        QueryOutput::QueryOutput {
                                            Items,
                                            Count,
                                            ScannedCount,
                                            LastEvaluatedKey,
                                            ConsumedCapacity,
                                        } => LastEvaluatedKey,
                                    }
                                }
                                pub fn ConsumedCapacity(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ConsumedCapacity>>>{
                                    match self {
                                        QueryOutput::QueryOutput {
                                            Items,
                                            Count,
                                            ScannedCount,
                                            LastEvaluatedKey,
                                            ConsumedCapacity,
                                        } => ConsumedCapacity,
                                    }
                                }
                            }

                            impl Debug for QueryOutput {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for QueryOutput {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        QueryOutput::QueryOutput {
                                            Items,
                                            Count,
                                            ScannedCount,
                                            LastEvaluatedKey,
                                            ConsumedCapacity,
                                        } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.QueryOutput.QueryOutput(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                Items, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                Count, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ScannedCount,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                LastEvaluatedKey,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ConsumedCapacity,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for QueryOutput {}

                            impl Hash for QueryOutput {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        QueryOutput::QueryOutput {
                                            Items,
                                            Count,
                                            ScannedCount,
                                            LastEvaluatedKey,
                                            ConsumedCapacity,
                                        } => {
                                            ::std::hash::Hash::hash(Items, _state);
                                            ::std::hash::Hash::hash(Count, _state);
                                            ::std::hash::Hash::hash(ScannedCount, _state);
                                            ::std::hash::Hash::hash(LastEvaluatedKey, _state);
                                            ::std::hash::Hash::hash(ConsumedCapacity, _state)
                                        }
                                    }
                                }
                            }

                            impl Default for QueryOutput {
                                fn default() -> QueryOutput {
                                    QueryOutput::QueryOutput {
                                        Items: ::std::default::Default::default(),
                                        Count: ::std::default::Default::default(),
                                        ScannedCount: ::std::default::Default::default(),
                                        LastEvaluatedKey: ::std::default::Default::default(),
                                        ConsumedCapacity: ::std::default::Default::default(),
                                    }
                                }
                            }

                            impl AsRef<QueryOutput> for &QueryOutput {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            #[derive(PartialEq, Clone)]
                            pub enum ReplicaDescription {
                                ReplicaDescription {
                  RegionName: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>,
                  ReplicaStatus: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ReplicaStatus>>>,
                  ReplicaStatusDescription: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>,
                  ReplicaStatusPercentProgress: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>,
                  KMSMasterKeyId: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>,
                  ProvisionedThroughputOverride: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ProvisionedThroughputOverride>>>,
                  GlobalSecondaryIndexes: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ReplicaGlobalSecondaryIndexDescription>>>>,
                  ReplicaInaccessibleDateTime: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>,
                  ReplicaTableClassSummary: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::TableClassSummary>>>
                }
              }

                            impl ReplicaDescription {
                                pub fn RegionName(
                                    &self,
                                ) -> &::std::rc::Rc<
                                    crate::r#_Wrappers_Compile::Option<
                                        ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                    >,
                                > {
                                    match self {
                                        ReplicaDescription::ReplicaDescription {
                                            RegionName,
                                            ReplicaStatus,
                                            ReplicaStatusDescription,
                                            ReplicaStatusPercentProgress,
                                            KMSMasterKeyId,
                                            ProvisionedThroughputOverride,
                                            GlobalSecondaryIndexes,
                                            ReplicaInaccessibleDateTime,
                                            ReplicaTableClassSummary,
                                        } => RegionName,
                                    }
                                }
                                pub fn ReplicaStatus(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ReplicaStatus>>>{
                                    match self {
                                        ReplicaDescription::ReplicaDescription {
                                            RegionName,
                                            ReplicaStatus,
                                            ReplicaStatusDescription,
                                            ReplicaStatusPercentProgress,
                                            KMSMasterKeyId,
                                            ProvisionedThroughputOverride,
                                            GlobalSecondaryIndexes,
                                            ReplicaInaccessibleDateTime,
                                            ReplicaTableClassSummary,
                                        } => ReplicaStatus,
                                    }
                                }
                                pub fn ReplicaStatusDescription(
                                    &self,
                                ) -> &::std::rc::Rc<
                                    crate::r#_Wrappers_Compile::Option<
                                        ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                    >,
                                > {
                                    match self {
                                        ReplicaDescription::ReplicaDescription {
                                            RegionName,
                                            ReplicaStatus,
                                            ReplicaStatusDescription,
                                            ReplicaStatusPercentProgress,
                                            KMSMasterKeyId,
                                            ProvisionedThroughputOverride,
                                            GlobalSecondaryIndexes,
                                            ReplicaInaccessibleDateTime,
                                            ReplicaTableClassSummary,
                                        } => ReplicaStatusDescription,
                                    }
                                }
                                pub fn ReplicaStatusPercentProgress(
                                    &self,
                                ) -> &::std::rc::Rc<
                                    crate::r#_Wrappers_Compile::Option<
                                        ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                    >,
                                > {
                                    match self {
                                        ReplicaDescription::ReplicaDescription {
                                            RegionName,
                                            ReplicaStatus,
                                            ReplicaStatusDescription,
                                            ReplicaStatusPercentProgress,
                                            KMSMasterKeyId,
                                            ProvisionedThroughputOverride,
                                            GlobalSecondaryIndexes,
                                            ReplicaInaccessibleDateTime,
                                            ReplicaTableClassSummary,
                                        } => ReplicaStatusPercentProgress,
                                    }
                                }
                                pub fn KMSMasterKeyId(
                                    &self,
                                ) -> &::std::rc::Rc<
                                    crate::r#_Wrappers_Compile::Option<
                                        ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                    >,
                                > {
                                    match self {
                                        ReplicaDescription::ReplicaDescription {
                                            RegionName,
                                            ReplicaStatus,
                                            ReplicaStatusDescription,
                                            ReplicaStatusPercentProgress,
                                            KMSMasterKeyId,
                                            ProvisionedThroughputOverride,
                                            GlobalSecondaryIndexes,
                                            ReplicaInaccessibleDateTime,
                                            ReplicaTableClassSummary,
                                        } => KMSMasterKeyId,
                                    }
                                }
                                pub fn ProvisionedThroughputOverride(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ProvisionedThroughputOverride>>>{
                                    match self {
                                        ReplicaDescription::ReplicaDescription {
                                            RegionName,
                                            ReplicaStatus,
                                            ReplicaStatusDescription,
                                            ReplicaStatusPercentProgress,
                                            KMSMasterKeyId,
                                            ProvisionedThroughputOverride,
                                            GlobalSecondaryIndexes,
                                            ReplicaInaccessibleDateTime,
                                            ReplicaTableClassSummary,
                                        } => ProvisionedThroughputOverride,
                                    }
                                }
                                pub fn GlobalSecondaryIndexes(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ReplicaGlobalSecondaryIndexDescription>>>>{
                                    match self {
                                        ReplicaDescription::ReplicaDescription {
                                            RegionName,
                                            ReplicaStatus,
                                            ReplicaStatusDescription,
                                            ReplicaStatusPercentProgress,
                                            KMSMasterKeyId,
                                            ProvisionedThroughputOverride,
                                            GlobalSecondaryIndexes,
                                            ReplicaInaccessibleDateTime,
                                            ReplicaTableClassSummary,
                                        } => GlobalSecondaryIndexes,
                                    }
                                }
                                pub fn ReplicaInaccessibleDateTime(
                                    &self,
                                ) -> &::std::rc::Rc<
                                    crate::r#_Wrappers_Compile::Option<
                                        ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                    >,
                                > {
                                    match self {
                                        ReplicaDescription::ReplicaDescription {
                                            RegionName,
                                            ReplicaStatus,
                                            ReplicaStatusDescription,
                                            ReplicaStatusPercentProgress,
                                            KMSMasterKeyId,
                                            ProvisionedThroughputOverride,
                                            GlobalSecondaryIndexes,
                                            ReplicaInaccessibleDateTime,
                                            ReplicaTableClassSummary,
                                        } => ReplicaInaccessibleDateTime,
                                    }
                                }
                                pub fn ReplicaTableClassSummary(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::TableClassSummary>>>{
                                    match self {
                                        ReplicaDescription::ReplicaDescription {
                                            RegionName,
                                            ReplicaStatus,
                                            ReplicaStatusDescription,
                                            ReplicaStatusPercentProgress,
                                            KMSMasterKeyId,
                                            ProvisionedThroughputOverride,
                                            GlobalSecondaryIndexes,
                                            ReplicaInaccessibleDateTime,
                                            ReplicaTableClassSummary,
                                        } => ReplicaTableClassSummary,
                                    }
                                }
                            }

                            impl Debug for ReplicaDescription {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for ReplicaDescription {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        ReplicaDescription::ReplicaDescription {
                                            RegionName,
                                            ReplicaStatus,
                                            ReplicaStatusDescription,
                                            ReplicaStatusPercentProgress,
                                            KMSMasterKeyId,
                                            ProvisionedThroughputOverride,
                                            GlobalSecondaryIndexes,
                                            ReplicaInaccessibleDateTime,
                                            ReplicaTableClassSummary,
                                        } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.ReplicaDescription.ReplicaDescription(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                RegionName, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ReplicaStatus,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ReplicaStatusDescription,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ReplicaStatusPercentProgress,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                KMSMasterKeyId,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ProvisionedThroughputOverride,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                GlobalSecondaryIndexes,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ReplicaInaccessibleDateTime,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ReplicaTableClassSummary,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for ReplicaDescription {}

                            impl Hash for ReplicaDescription {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        ReplicaDescription::ReplicaDescription {
                                            RegionName,
                                            ReplicaStatus,
                                            ReplicaStatusDescription,
                                            ReplicaStatusPercentProgress,
                                            KMSMasterKeyId,
                                            ProvisionedThroughputOverride,
                                            GlobalSecondaryIndexes,
                                            ReplicaInaccessibleDateTime,
                                            ReplicaTableClassSummary,
                                        } => {
                                            ::std::hash::Hash::hash(RegionName, _state);
                                            ::std::hash::Hash::hash(ReplicaStatus, _state);
                                            ::std::hash::Hash::hash(
                                                ReplicaStatusDescription,
                                                _state,
                                            );
                                            ::std::hash::Hash::hash(
                                                ReplicaStatusPercentProgress,
                                                _state,
                                            );
                                            ::std::hash::Hash::hash(KMSMasterKeyId, _state);
                                            ::std::hash::Hash::hash(
                                                ProvisionedThroughputOverride,
                                                _state,
                                            );
                                            ::std::hash::Hash::hash(GlobalSecondaryIndexes, _state);
                                            ::std::hash::Hash::hash(
                                                ReplicaInaccessibleDateTime,
                                                _state,
                                            );
                                            ::std::hash::Hash::hash(
                                                ReplicaTableClassSummary,
                                                _state,
                                            )
                                        }
                                    }
                                }
                            }

                            impl Default for ReplicaDescription {
                                fn default() -> ReplicaDescription {
                                    ReplicaDescription::ReplicaDescription {
                                        RegionName: ::std::default::Default::default(),
                                        ReplicaStatus: ::std::default::Default::default(),
                                        ReplicaStatusDescription: ::std::default::Default::default(
                                        ),
                                        ReplicaStatusPercentProgress:
                                            ::std::default::Default::default(),
                                        KMSMasterKeyId: ::std::default::Default::default(),
                                        ProvisionedThroughputOverride:
                                            ::std::default::Default::default(),
                                        GlobalSecondaryIndexes: ::std::default::Default::default(),
                                        ReplicaInaccessibleDateTime:
                                            ::std::default::Default::default(),
                                        ReplicaTableClassSummary: ::std::default::Default::default(
                                        ),
                                    }
                                }
                            }

                            impl AsRef<ReplicaDescription> for &ReplicaDescription {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            #[derive(PartialEq, Clone)]
                            pub enum ReplicaGlobalSecondaryIndexDescription {
                                ReplicaGlobalSecondaryIndexDescription {
                  IndexName: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>,
                  ProvisionedThroughputOverride: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ProvisionedThroughputOverride>>>
                }
              }

                            impl ReplicaGlobalSecondaryIndexDescription {
                                pub fn IndexName(
                                    &self,
                                ) -> &::std::rc::Rc<
                                    crate::r#_Wrappers_Compile::Option<
                                        ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                    >,
                                > {
                                    match self {
                    ReplicaGlobalSecondaryIndexDescription::ReplicaGlobalSecondaryIndexDescription{IndexName, ProvisionedThroughputOverride, } => IndexName,
                  }
                                }
                                pub fn ProvisionedThroughputOverride(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ProvisionedThroughputOverride>>>{
                                    match self {
                    ReplicaGlobalSecondaryIndexDescription::ReplicaGlobalSecondaryIndexDescription{IndexName, ProvisionedThroughputOverride, } => ProvisionedThroughputOverride,
                  }
                                }
                            }

                            impl Debug for ReplicaGlobalSecondaryIndexDescription {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for ReplicaGlobalSecondaryIndexDescription {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                    ReplicaGlobalSecondaryIndexDescription::ReplicaGlobalSecondaryIndexDescription{IndexName, ProvisionedThroughputOverride, } => {
                      write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.ReplicaGlobalSecondaryIndexDescription.ReplicaGlobalSecondaryIndexDescription(")?;
                      ::dafny_runtime::DafnyPrint::fmt_print(IndexName, _formatter, false)?;
                      write!(_formatter, ", ")?;
                      ::dafny_runtime::DafnyPrint::fmt_print(ProvisionedThroughputOverride, _formatter, false)?;
                      write!(_formatter, ")")?;
                      Ok(())
                    },
                  }
                                }
                            }

                            impl Eq for ReplicaGlobalSecondaryIndexDescription {}

                            impl Hash for ReplicaGlobalSecondaryIndexDescription {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                    ReplicaGlobalSecondaryIndexDescription::ReplicaGlobalSecondaryIndexDescription{IndexName, ProvisionedThroughputOverride, } => {
                      ::std::hash::Hash::hash(IndexName, _state);
                      ::std::hash::Hash::hash(ProvisionedThroughputOverride, _state)
                    },
                  }
                                }
                            }

                            impl Default for ReplicaGlobalSecondaryIndexDescription {
                                fn default() -> ReplicaGlobalSecondaryIndexDescription {
                                    ReplicaGlobalSecondaryIndexDescription::ReplicaGlobalSecondaryIndexDescription {
                    IndexName: ::std::default::Default::default(),
                    ProvisionedThroughputOverride: ::std::default::Default::default()
                  }
                                }
                            }

                            impl AsRef<ReplicaGlobalSecondaryIndexDescription> for &ReplicaGlobalSecondaryIndexDescription {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            #[derive(PartialEq, Clone)]
                            pub enum ReplicaStatus {
                                CREATING {},
                                CREATION_FAILED {},
                                UPDATING {},
                                DELETING {},
                                ACTIVE {},
                                REGION_DISABLED {},
                                INACCESSIBLE_ENCRYPTION_CREDENTIALS {},
                            }

                            impl ReplicaStatus {}

                            impl Debug for ReplicaStatus {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for ReplicaStatus {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        ReplicaStatus::CREATING {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.ReplicaStatus.CREATING")?;
                                            Ok(())
                                        }
                                        ReplicaStatus::CREATION_FAILED {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.ReplicaStatus.CREATION__FAILED")?;
                                            Ok(())
                                        }
                                        ReplicaStatus::UPDATING {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.ReplicaStatus.UPDATING")?;
                                            Ok(())
                                        }
                                        ReplicaStatus::DELETING {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.ReplicaStatus.DELETING")?;
                                            Ok(())
                                        }
                                        ReplicaStatus::ACTIVE {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.ReplicaStatus.ACTIVE")?;
                                            Ok(())
                                        }
                                        ReplicaStatus::REGION_DISABLED {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.ReplicaStatus.REGION__DISABLED")?;
                                            Ok(())
                                        }
                                        ReplicaStatus::INACCESSIBLE_ENCRYPTION_CREDENTIALS {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.ReplicaStatus.INACCESSIBLE__ENCRYPTION__CREDENTIALS")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for ReplicaStatus {}

                            impl Hash for ReplicaStatus {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        ReplicaStatus::CREATING {} => {}
                                        ReplicaStatus::CREATION_FAILED {} => {}
                                        ReplicaStatus::UPDATING {} => {}
                                        ReplicaStatus::DELETING {} => {}
                                        ReplicaStatus::ACTIVE {} => {}
                                        ReplicaStatus::REGION_DISABLED {} => {}
                                        ReplicaStatus::INACCESSIBLE_ENCRYPTION_CREDENTIALS {} => {}
                                    }
                                }
                            }

                            impl Default for ReplicaStatus {
                                fn default() -> ReplicaStatus {
                                    ReplicaStatus::CREATING {}
                                }
                            }

                            impl AsRef<ReplicaStatus> for &ReplicaStatus {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            #[derive(PartialEq, Clone)]
                            pub enum RestoreSummary {
                                RestoreSummary {
                                    SourceBackupArn: ::std::rc::Rc<
                                        crate::r#_Wrappers_Compile::Option<
                                            ::dafny_runtime::Sequence<
                                                ::dafny_runtime::DafnyCharUTF16,
                                            >,
                                        >,
                                    >,
                                    SourceTableArn: ::std::rc::Rc<
                                        crate::r#_Wrappers_Compile::Option<
                                            ::dafny_runtime::Sequence<
                                                ::dafny_runtime::DafnyCharUTF16,
                                            >,
                                        >,
                                    >,
                                    RestoreDateTime:
                                        ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                    RestoreInProgress: bool,
                                },
                            }

                            impl RestoreSummary {
                                pub fn SourceBackupArn(
                                    &self,
                                ) -> &::std::rc::Rc<
                                    crate::r#_Wrappers_Compile::Option<
                                        ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                    >,
                                > {
                                    match self {
                                        RestoreSummary::RestoreSummary {
                                            SourceBackupArn,
                                            SourceTableArn,
                                            RestoreDateTime,
                                            RestoreInProgress,
                                        } => SourceBackupArn,
                                    }
                                }
                                pub fn SourceTableArn(
                                    &self,
                                ) -> &::std::rc::Rc<
                                    crate::r#_Wrappers_Compile::Option<
                                        ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                    >,
                                > {
                                    match self {
                                        RestoreSummary::RestoreSummary {
                                            SourceBackupArn,
                                            SourceTableArn,
                                            RestoreDateTime,
                                            RestoreInProgress,
                                        } => SourceTableArn,
                                    }
                                }
                                pub fn RestoreDateTime(
                                    &self,
                                ) -> &::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>
                                {
                                    match self {
                                        RestoreSummary::RestoreSummary {
                                            SourceBackupArn,
                                            SourceTableArn,
                                            RestoreDateTime,
                                            RestoreInProgress,
                                        } => RestoreDateTime,
                                    }
                                }
                                pub fn RestoreInProgress(&self) -> &bool {
                                    match self {
                                        RestoreSummary::RestoreSummary {
                                            SourceBackupArn,
                                            SourceTableArn,
                                            RestoreDateTime,
                                            RestoreInProgress,
                                        } => RestoreInProgress,
                                    }
                                }
                            }

                            impl Debug for RestoreSummary {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for RestoreSummary {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        RestoreSummary::RestoreSummary {
                                            SourceBackupArn,
                                            SourceTableArn,
                                            RestoreDateTime,
                                            RestoreInProgress,
                                        } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.RestoreSummary.RestoreSummary(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                SourceBackupArn,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                SourceTableArn,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                RestoreDateTime,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                RestoreInProgress,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for RestoreSummary {}

                            impl Hash for RestoreSummary {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        RestoreSummary::RestoreSummary {
                                            SourceBackupArn,
                                            SourceTableArn,
                                            RestoreDateTime,
                                            RestoreInProgress,
                                        } => {
                                            ::std::hash::Hash::hash(SourceBackupArn, _state);
                                            ::std::hash::Hash::hash(SourceTableArn, _state);
                                            ::std::hash::Hash::hash(RestoreDateTime, _state);
                                            ::std::hash::Hash::hash(RestoreInProgress, _state)
                                        }
                                    }
                                }
                            }

                            impl Default for RestoreSummary {
                                fn default() -> RestoreSummary {
                                    RestoreSummary::RestoreSummary {
                                        SourceBackupArn: ::std::default::Default::default(),
                                        SourceTableArn: ::std::default::Default::default(),
                                        RestoreDateTime: ::std::default::Default::default(),
                                        RestoreInProgress: ::std::default::Default::default(),
                                    }
                                }
                            }

                            impl AsRef<RestoreSummary> for &RestoreSummary {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            #[derive(PartialEq, Clone)]
                            pub enum ReturnConsumedCapacity {
                                INDEXES {},
                                TOTAL {},
                                NONE {},
                            }

                            impl ReturnConsumedCapacity {}

                            impl Debug for ReturnConsumedCapacity {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for ReturnConsumedCapacity {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        ReturnConsumedCapacity::INDEXES {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.ReturnConsumedCapacity.INDEXES")?;
                                            Ok(())
                                        }
                                        ReturnConsumedCapacity::TOTAL {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.ReturnConsumedCapacity.TOTAL")?;
                                            Ok(())
                                        }
                                        ReturnConsumedCapacity::NONE {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.ReturnConsumedCapacity.NONE")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for ReturnConsumedCapacity {}

                            impl Hash for ReturnConsumedCapacity {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        ReturnConsumedCapacity::INDEXES {} => {}
                                        ReturnConsumedCapacity::TOTAL {} => {}
                                        ReturnConsumedCapacity::NONE {} => {}
                                    }
                                }
                            }

                            impl Default for ReturnConsumedCapacity {
                                fn default() -> ReturnConsumedCapacity {
                                    ReturnConsumedCapacity::INDEXES {}
                                }
                            }

                            impl AsRef<ReturnConsumedCapacity> for &ReturnConsumedCapacity {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            #[derive(PartialEq, Clone)]
                            pub enum ReturnItemCollectionMetrics {
                                SIZE {},
                                NONE {},
                            }

                            impl ReturnItemCollectionMetrics {}

                            impl Debug for ReturnItemCollectionMetrics {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for ReturnItemCollectionMetrics {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        ReturnItemCollectionMetrics::SIZE {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.ReturnItemCollectionMetrics.SIZE")?;
                                            Ok(())
                                        }
                                        ReturnItemCollectionMetrics::NONE {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.ReturnItemCollectionMetrics.NONE")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for ReturnItemCollectionMetrics {}

                            impl Hash for ReturnItemCollectionMetrics {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        ReturnItemCollectionMetrics::SIZE {} => {}
                                        ReturnItemCollectionMetrics::NONE {} => {}
                                    }
                                }
                            }

                            impl Default for ReturnItemCollectionMetrics {
                                fn default() -> ReturnItemCollectionMetrics {
                                    ReturnItemCollectionMetrics::SIZE {}
                                }
                            }

                            impl AsRef<ReturnItemCollectionMetrics> for &ReturnItemCollectionMetrics {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            #[derive(PartialEq, Clone)]
                            pub enum ReturnValue {
                                NONE {},
                                ALL_OLD {},
                                UPDATED_OLD {},
                                ALL_NEW {},
                                UPDATED_NEW {},
                            }

                            impl ReturnValue {}

                            impl Debug for ReturnValue {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for ReturnValue {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        ReturnValue::NONE {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.ReturnValue.NONE")?;
                                            Ok(())
                                        }
                                        ReturnValue::ALL_OLD {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.ReturnValue.ALL__OLD")?;
                                            Ok(())
                                        }
                                        ReturnValue::UPDATED_OLD {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.ReturnValue.UPDATED__OLD")?;
                                            Ok(())
                                        }
                                        ReturnValue::ALL_NEW {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.ReturnValue.ALL__NEW")?;
                                            Ok(())
                                        }
                                        ReturnValue::UPDATED_NEW {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.ReturnValue.UPDATED__NEW")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for ReturnValue {}

                            impl Hash for ReturnValue {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        ReturnValue::NONE {} => {}
                                        ReturnValue::ALL_OLD {} => {}
                                        ReturnValue::UPDATED_OLD {} => {}
                                        ReturnValue::ALL_NEW {} => {}
                                        ReturnValue::UPDATED_NEW {} => {}
                                    }
                                }
                            }

                            impl Default for ReturnValue {
                                fn default() -> ReturnValue {
                                    ReturnValue::NONE {}
                                }
                            }

                            impl AsRef<ReturnValue> for &ReturnValue {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            #[derive(PartialEq, Clone)]
                            pub enum ReturnValuesOnConditionCheckFailure {
                                ALL_OLD {},
                                NONE {},
                            }

                            impl ReturnValuesOnConditionCheckFailure {}

                            impl Debug for ReturnValuesOnConditionCheckFailure {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for ReturnValuesOnConditionCheckFailure {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        ReturnValuesOnConditionCheckFailure::ALL_OLD {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.ReturnValuesOnConditionCheckFailure.ALL__OLD")?;
                                            Ok(())
                                        }
                                        ReturnValuesOnConditionCheckFailure::NONE {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.ReturnValuesOnConditionCheckFailure.NONE")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for ReturnValuesOnConditionCheckFailure {}

                            impl Hash for ReturnValuesOnConditionCheckFailure {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        ReturnValuesOnConditionCheckFailure::ALL_OLD {} => {}
                                        ReturnValuesOnConditionCheckFailure::NONE {} => {}
                                    }
                                }
                            }

                            impl Default for ReturnValuesOnConditionCheckFailure {
                                fn default() -> ReturnValuesOnConditionCheckFailure {
                                    ReturnValuesOnConditionCheckFailure::ALL_OLD {}
                                }
                            }

                            impl AsRef<ReturnValuesOnConditionCheckFailure> for &ReturnValuesOnConditionCheckFailure {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            #[derive(PartialEq, Clone)]
                            pub enum ScalarAttributeType {
                                S {},
                                N {},
                                B {},
                            }

                            impl ScalarAttributeType {}

                            impl Debug for ScalarAttributeType {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for ScalarAttributeType {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        ScalarAttributeType::S {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.ScalarAttributeType.S")?;
                                            Ok(())
                                        }
                                        ScalarAttributeType::N {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.ScalarAttributeType.N")?;
                                            Ok(())
                                        }
                                        ScalarAttributeType::B {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.ScalarAttributeType.B")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for ScalarAttributeType {}

                            impl Hash for ScalarAttributeType {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        ScalarAttributeType::S {} => {}
                                        ScalarAttributeType::N {} => {}
                                        ScalarAttributeType::B {} => {}
                                    }
                                }
                            }

                            impl Default for ScalarAttributeType {
                                fn default() -> ScalarAttributeType {
                                    ScalarAttributeType::S {}
                                }
                            }

                            impl AsRef<ScalarAttributeType> for &ScalarAttributeType {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            #[derive(PartialEq, Clone)]
                            pub enum ScanInput {
                                ScanInput {
                  TableName: ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                  IndexName: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>,
                  AttributesToGet: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeNameList>>,
                  Limit: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::PositiveIntegerObject>>,
                  Select: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Select>>>,
                  ScanFilter: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Condition>>>>,
                  ConditionalOperator: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ConditionalOperator>>>,
                  ExclusiveStartKey: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>>>,
                  ReturnConsumedCapacity: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ReturnConsumedCapacity>>>,
                  TotalSegments: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ScanTotalSegments>>,
                  Segment: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ScanSegment>>,
                  ProjectionExpression: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>,
                  FilterExpression: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>,
                  ExpressionAttributeNames: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>>,
                  ExpressionAttributeValues: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>>>,
                  ConsistentRead: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<bool>>
                }
              }

                            impl ScanInput {
                                pub fn TableName(
                                    &self,
                                ) -> &::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>
                                {
                                    match self {
                                        ScanInput::ScanInput {
                                            TableName,
                                            IndexName,
                                            AttributesToGet,
                                            Limit,
                                            Select,
                                            ScanFilter,
                                            ConditionalOperator,
                                            ExclusiveStartKey,
                                            ReturnConsumedCapacity,
                                            TotalSegments,
                                            Segment,
                                            ProjectionExpression,
                                            FilterExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                            ConsistentRead,
                                        } => TableName,
                                    }
                                }
                                pub fn IndexName(
                                    &self,
                                ) -> &::std::rc::Rc<
                                    crate::r#_Wrappers_Compile::Option<
                                        ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                    >,
                                > {
                                    match self {
                                        ScanInput::ScanInput {
                                            TableName,
                                            IndexName,
                                            AttributesToGet,
                                            Limit,
                                            Select,
                                            ScanFilter,
                                            ConditionalOperator,
                                            ExclusiveStartKey,
                                            ReturnConsumedCapacity,
                                            TotalSegments,
                                            Segment,
                                            ProjectionExpression,
                                            FilterExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                            ConsistentRead,
                                        } => IndexName,
                                    }
                                }
                                pub fn AttributesToGet(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeNameList>>{
                                    match self {
                                        ScanInput::ScanInput {
                                            TableName,
                                            IndexName,
                                            AttributesToGet,
                                            Limit,
                                            Select,
                                            ScanFilter,
                                            ConditionalOperator,
                                            ExclusiveStartKey,
                                            ReturnConsumedCapacity,
                                            TotalSegments,
                                            Segment,
                                            ProjectionExpression,
                                            FilterExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                            ConsistentRead,
                                        } => AttributesToGet,
                                    }
                                }
                                pub fn Limit(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::PositiveIntegerObject>>{
                                    match self {
                                        ScanInput::ScanInput {
                                            TableName,
                                            IndexName,
                                            AttributesToGet,
                                            Limit,
                                            Select,
                                            ScanFilter,
                                            ConditionalOperator,
                                            ExclusiveStartKey,
                                            ReturnConsumedCapacity,
                                            TotalSegments,
                                            Segment,
                                            ProjectionExpression,
                                            FilterExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                            ConsistentRead,
                                        } => Limit,
                                    }
                                }
                                pub fn Select(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Select>>>{
                                    match self {
                                        ScanInput::ScanInput {
                                            TableName,
                                            IndexName,
                                            AttributesToGet,
                                            Limit,
                                            Select,
                                            ScanFilter,
                                            ConditionalOperator,
                                            ExclusiveStartKey,
                                            ReturnConsumedCapacity,
                                            TotalSegments,
                                            Segment,
                                            ProjectionExpression,
                                            FilterExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                            ConsistentRead,
                                        } => Select,
                                    }
                                }
                                pub fn ScanFilter(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Condition>>>>{
                                    match self {
                                        ScanInput::ScanInput {
                                            TableName,
                                            IndexName,
                                            AttributesToGet,
                                            Limit,
                                            Select,
                                            ScanFilter,
                                            ConditionalOperator,
                                            ExclusiveStartKey,
                                            ReturnConsumedCapacity,
                                            TotalSegments,
                                            Segment,
                                            ProjectionExpression,
                                            FilterExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                            ConsistentRead,
                                        } => ScanFilter,
                                    }
                                }
                                pub fn ConditionalOperator(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ConditionalOperator>>>{
                                    match self {
                                        ScanInput::ScanInput {
                                            TableName,
                                            IndexName,
                                            AttributesToGet,
                                            Limit,
                                            Select,
                                            ScanFilter,
                                            ConditionalOperator,
                                            ExclusiveStartKey,
                                            ReturnConsumedCapacity,
                                            TotalSegments,
                                            Segment,
                                            ProjectionExpression,
                                            FilterExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                            ConsistentRead,
                                        } => ConditionalOperator,
                                    }
                                }
                                pub fn ExclusiveStartKey(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>>>{
                                    match self {
                                        ScanInput::ScanInput {
                                            TableName,
                                            IndexName,
                                            AttributesToGet,
                                            Limit,
                                            Select,
                                            ScanFilter,
                                            ConditionalOperator,
                                            ExclusiveStartKey,
                                            ReturnConsumedCapacity,
                                            TotalSegments,
                                            Segment,
                                            ProjectionExpression,
                                            FilterExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                            ConsistentRead,
                                        } => ExclusiveStartKey,
                                    }
                                }
                                pub fn ReturnConsumedCapacity(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ReturnConsumedCapacity>>>{
                                    match self {
                                        ScanInput::ScanInput {
                                            TableName,
                                            IndexName,
                                            AttributesToGet,
                                            Limit,
                                            Select,
                                            ScanFilter,
                                            ConditionalOperator,
                                            ExclusiveStartKey,
                                            ReturnConsumedCapacity,
                                            TotalSegments,
                                            Segment,
                                            ProjectionExpression,
                                            FilterExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                            ConsistentRead,
                                        } => ReturnConsumedCapacity,
                                    }
                                }
                                pub fn TotalSegments(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ScanTotalSegments>>{
                                    match self {
                                        ScanInput::ScanInput {
                                            TableName,
                                            IndexName,
                                            AttributesToGet,
                                            Limit,
                                            Select,
                                            ScanFilter,
                                            ConditionalOperator,
                                            ExclusiveStartKey,
                                            ReturnConsumedCapacity,
                                            TotalSegments,
                                            Segment,
                                            ProjectionExpression,
                                            FilterExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                            ConsistentRead,
                                        } => TotalSegments,
                                    }
                                }
                                pub fn Segment(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ScanSegment>>{
                                    match self {
                                        ScanInput::ScanInput {
                                            TableName,
                                            IndexName,
                                            AttributesToGet,
                                            Limit,
                                            Select,
                                            ScanFilter,
                                            ConditionalOperator,
                                            ExclusiveStartKey,
                                            ReturnConsumedCapacity,
                                            TotalSegments,
                                            Segment,
                                            ProjectionExpression,
                                            FilterExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                            ConsistentRead,
                                        } => Segment,
                                    }
                                }
                                pub fn ProjectionExpression(
                                    &self,
                                ) -> &::std::rc::Rc<
                                    crate::r#_Wrappers_Compile::Option<
                                        ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                    >,
                                > {
                                    match self {
                                        ScanInput::ScanInput {
                                            TableName,
                                            IndexName,
                                            AttributesToGet,
                                            Limit,
                                            Select,
                                            ScanFilter,
                                            ConditionalOperator,
                                            ExclusiveStartKey,
                                            ReturnConsumedCapacity,
                                            TotalSegments,
                                            Segment,
                                            ProjectionExpression,
                                            FilterExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                            ConsistentRead,
                                        } => ProjectionExpression,
                                    }
                                }
                                pub fn FilterExpression(
                                    &self,
                                ) -> &::std::rc::Rc<
                                    crate::r#_Wrappers_Compile::Option<
                                        ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                    >,
                                > {
                                    match self {
                                        ScanInput::ScanInput {
                                            TableName,
                                            IndexName,
                                            AttributesToGet,
                                            Limit,
                                            Select,
                                            ScanFilter,
                                            ConditionalOperator,
                                            ExclusiveStartKey,
                                            ReturnConsumedCapacity,
                                            TotalSegments,
                                            Segment,
                                            ProjectionExpression,
                                            FilterExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                            ConsistentRead,
                                        } => FilterExpression,
                                    }
                                }
                                pub fn ExpressionAttributeNames(
                                    &self,
                                ) -> &::std::rc::Rc<
                                    crate::r#_Wrappers_Compile::Option<
                                        ::dafny_runtime::Map<
                                            ::dafny_runtime::Sequence<
                                                ::dafny_runtime::DafnyCharUTF16,
                                            >,
                                            ::dafny_runtime::Sequence<
                                                ::dafny_runtime::DafnyCharUTF16,
                                            >,
                                        >,
                                    >,
                                > {
                                    match self {
                                        ScanInput::ScanInput {
                                            TableName,
                                            IndexName,
                                            AttributesToGet,
                                            Limit,
                                            Select,
                                            ScanFilter,
                                            ConditionalOperator,
                                            ExclusiveStartKey,
                                            ReturnConsumedCapacity,
                                            TotalSegments,
                                            Segment,
                                            ProjectionExpression,
                                            FilterExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                            ConsistentRead,
                                        } => ExpressionAttributeNames,
                                    }
                                }
                                pub fn ExpressionAttributeValues(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>>>{
                                    match self {
                                        ScanInput::ScanInput {
                                            TableName,
                                            IndexName,
                                            AttributesToGet,
                                            Limit,
                                            Select,
                                            ScanFilter,
                                            ConditionalOperator,
                                            ExclusiveStartKey,
                                            ReturnConsumedCapacity,
                                            TotalSegments,
                                            Segment,
                                            ProjectionExpression,
                                            FilterExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                            ConsistentRead,
                                        } => ExpressionAttributeValues,
                                    }
                                }
                                pub fn ConsistentRead(
                                    &self,
                                ) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<bool>>
                                {
                                    match self {
                                        ScanInput::ScanInput {
                                            TableName,
                                            IndexName,
                                            AttributesToGet,
                                            Limit,
                                            Select,
                                            ScanFilter,
                                            ConditionalOperator,
                                            ExclusiveStartKey,
                                            ReturnConsumedCapacity,
                                            TotalSegments,
                                            Segment,
                                            ProjectionExpression,
                                            FilterExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                            ConsistentRead,
                                        } => ConsistentRead,
                                    }
                                }
                            }

                            impl Debug for ScanInput {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for ScanInput {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        ScanInput::ScanInput {
                                            TableName,
                                            IndexName,
                                            AttributesToGet,
                                            Limit,
                                            Select,
                                            ScanFilter,
                                            ConditionalOperator,
                                            ExclusiveStartKey,
                                            ReturnConsumedCapacity,
                                            TotalSegments,
                                            Segment,
                                            ProjectionExpression,
                                            FilterExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                            ConsistentRead,
                                        } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.ScanInput.ScanInput(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                TableName, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                IndexName, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                AttributesToGet,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                Limit, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                Select, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ScanFilter, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ConditionalOperator,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ExclusiveStartKey,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ReturnConsumedCapacity,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                TotalSegments,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                Segment, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ProjectionExpression,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                FilterExpression,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ExpressionAttributeNames,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ExpressionAttributeValues,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ConsistentRead,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for ScanInput {}

                            impl Hash for ScanInput {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        ScanInput::ScanInput {
                                            TableName,
                                            IndexName,
                                            AttributesToGet,
                                            Limit,
                                            Select,
                                            ScanFilter,
                                            ConditionalOperator,
                                            ExclusiveStartKey,
                                            ReturnConsumedCapacity,
                                            TotalSegments,
                                            Segment,
                                            ProjectionExpression,
                                            FilterExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                            ConsistentRead,
                                        } => {
                                            ::std::hash::Hash::hash(TableName, _state);
                                            ::std::hash::Hash::hash(IndexName, _state);
                                            ::std::hash::Hash::hash(AttributesToGet, _state);
                                            ::std::hash::Hash::hash(Limit, _state);
                                            ::std::hash::Hash::hash(Select, _state);
                                            ::std::hash::Hash::hash(ScanFilter, _state);
                                            ::std::hash::Hash::hash(ConditionalOperator, _state);
                                            ::std::hash::Hash::hash(ExclusiveStartKey, _state);
                                            ::std::hash::Hash::hash(ReturnConsumedCapacity, _state);
                                            ::std::hash::Hash::hash(TotalSegments, _state);
                                            ::std::hash::Hash::hash(Segment, _state);
                                            ::std::hash::Hash::hash(ProjectionExpression, _state);
                                            ::std::hash::Hash::hash(FilterExpression, _state);
                                            ::std::hash::Hash::hash(
                                                ExpressionAttributeNames,
                                                _state,
                                            );
                                            ::std::hash::Hash::hash(
                                                ExpressionAttributeValues,
                                                _state,
                                            );
                                            ::std::hash::Hash::hash(ConsistentRead, _state)
                                        }
                                    }
                                }
                            }

                            impl Default for ScanInput {
                                fn default() -> ScanInput {
                                    ScanInput::ScanInput {
                                        TableName: ::std::default::Default::default(),
                                        IndexName: ::std::default::Default::default(),
                                        AttributesToGet: ::std::default::Default::default(),
                                        Limit: ::std::default::Default::default(),
                                        Select: ::std::default::Default::default(),
                                        ScanFilter: ::std::default::Default::default(),
                                        ConditionalOperator: ::std::default::Default::default(),
                                        ExclusiveStartKey: ::std::default::Default::default(),
                                        ReturnConsumedCapacity: ::std::default::Default::default(),
                                        TotalSegments: ::std::default::Default::default(),
                                        Segment: ::std::default::Default::default(),
                                        ProjectionExpression: ::std::default::Default::default(),
                                        FilterExpression: ::std::default::Default::default(),
                                        ExpressionAttributeNames: ::std::default::Default::default(
                                        ),
                                        ExpressionAttributeValues: ::std::default::Default::default(
                                        ),
                                        ConsistentRead: ::std::default::Default::default(),
                                    }
                                }
                            }

                            impl AsRef<ScanInput> for &ScanInput {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            #[derive(PartialEq, Clone)]
                            pub enum ScanOutput {
                                ScanOutput {
                  Items: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>>>>,
                  Count: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<i32>>,
                  ScannedCount: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<i32>>,
                  LastEvaluatedKey: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>>>,
                  ConsumedCapacity: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ConsumedCapacity>>>
                }
              }

                            impl ScanOutput {
                                pub fn Items(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>>>>{
                                    match self {
                                        ScanOutput::ScanOutput {
                                            Items,
                                            Count,
                                            ScannedCount,
                                            LastEvaluatedKey,
                                            ConsumedCapacity,
                                        } => Items,
                                    }
                                }
                                pub fn Count(
                                    &self,
                                ) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<i32>>
                                {
                                    match self {
                                        ScanOutput::ScanOutput {
                                            Items,
                                            Count,
                                            ScannedCount,
                                            LastEvaluatedKey,
                                            ConsumedCapacity,
                                        } => Count,
                                    }
                                }
                                pub fn ScannedCount(
                                    &self,
                                ) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<i32>>
                                {
                                    match self {
                                        ScanOutput::ScanOutput {
                                            Items,
                                            Count,
                                            ScannedCount,
                                            LastEvaluatedKey,
                                            ConsumedCapacity,
                                        } => ScannedCount,
                                    }
                                }
                                pub fn LastEvaluatedKey(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>>>{
                                    match self {
                                        ScanOutput::ScanOutput {
                                            Items,
                                            Count,
                                            ScannedCount,
                                            LastEvaluatedKey,
                                            ConsumedCapacity,
                                        } => LastEvaluatedKey,
                                    }
                                }
                                pub fn ConsumedCapacity(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ConsumedCapacity>>>{
                                    match self {
                                        ScanOutput::ScanOutput {
                                            Items,
                                            Count,
                                            ScannedCount,
                                            LastEvaluatedKey,
                                            ConsumedCapacity,
                                        } => ConsumedCapacity,
                                    }
                                }
                            }

                            impl Debug for ScanOutput {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for ScanOutput {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        ScanOutput::ScanOutput {
                                            Items,
                                            Count,
                                            ScannedCount,
                                            LastEvaluatedKey,
                                            ConsumedCapacity,
                                        } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.ScanOutput.ScanOutput(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                Items, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                Count, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ScannedCount,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                LastEvaluatedKey,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ConsumedCapacity,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for ScanOutput {}

                            impl Hash for ScanOutput {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        ScanOutput::ScanOutput {
                                            Items,
                                            Count,
                                            ScannedCount,
                                            LastEvaluatedKey,
                                            ConsumedCapacity,
                                        } => {
                                            ::std::hash::Hash::hash(Items, _state);
                                            ::std::hash::Hash::hash(Count, _state);
                                            ::std::hash::Hash::hash(ScannedCount, _state);
                                            ::std::hash::Hash::hash(LastEvaluatedKey, _state);
                                            ::std::hash::Hash::hash(ConsumedCapacity, _state)
                                        }
                                    }
                                }
                            }

                            impl Default for ScanOutput {
                                fn default() -> ScanOutput {
                                    ScanOutput::ScanOutput {
                                        Items: ::std::default::Default::default(),
                                        Count: ::std::default::Default::default(),
                                        ScannedCount: ::std::default::Default::default(),
                                        LastEvaluatedKey: ::std::default::Default::default(),
                                        ConsumedCapacity: ::std::default::Default::default(),
                                    }
                                }
                            }

                            impl AsRef<ScanOutput> for &ScanOutput {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            pub type ScanSegment = i32;

                            pub type ScanTotalSegments = i32;

                            #[derive(PartialEq, Clone)]
                            pub enum Select {
                                ALL_ATTRIBUTES {},
                                ALL_PROJECTED_ATTRIBUTES {},
                                SPECIFIC_ATTRIBUTES {},
                                COUNT {},
                            }

                            impl Select {}

                            impl Debug for Select {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for Select {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        Select::ALL_ATTRIBUTES {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.Select.ALL__ATTRIBUTES")?;
                                            Ok(())
                                        }
                                        Select::ALL_PROJECTED_ATTRIBUTES {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.Select.ALL__PROJECTED__ATTRIBUTES")?;
                                            Ok(())
                                        }
                                        Select::SPECIFIC_ATTRIBUTES {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.Select.SPECIFIC__ATTRIBUTES")?;
                                            Ok(())
                                        }
                                        Select::COUNT {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.Select.COUNT")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for Select {}

                            impl Hash for Select {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        Select::ALL_ATTRIBUTES {} => {}
                                        Select::ALL_PROJECTED_ATTRIBUTES {} => {}
                                        Select::SPECIFIC_ATTRIBUTES {} => {}
                                        Select::COUNT {} => {}
                                    }
                                }
                            }

                            impl Default for Select {
                                fn default() -> Select {
                                    Select::ALL_ATTRIBUTES {}
                                }
                            }

                            impl AsRef<Select> for &Select {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            #[derive(PartialEq, Clone)]
                            pub enum SSEDescription {
                                SSEDescription {
                  Status: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::SSEStatus>>>,
                  SSEType: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::SSEType>>>,
                  KMSMasterKeyArn: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>,
                  InaccessibleEncryptionDateTime: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>
                }
              }

                            impl SSEDescription {
                                pub fn Status(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::SSEStatus>>>{
                                    match self {
                                        SSEDescription::SSEDescription {
                                            Status,
                                            SSEType,
                                            KMSMasterKeyArn,
                                            InaccessibleEncryptionDateTime,
                                        } => Status,
                                    }
                                }
                                pub fn SSEType(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::SSEType>>>{
                                    match self {
                                        SSEDescription::SSEDescription {
                                            Status,
                                            SSEType,
                                            KMSMasterKeyArn,
                                            InaccessibleEncryptionDateTime,
                                        } => SSEType,
                                    }
                                }
                                pub fn KMSMasterKeyArn(
                                    &self,
                                ) -> &::std::rc::Rc<
                                    crate::r#_Wrappers_Compile::Option<
                                        ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                    >,
                                > {
                                    match self {
                                        SSEDescription::SSEDescription {
                                            Status,
                                            SSEType,
                                            KMSMasterKeyArn,
                                            InaccessibleEncryptionDateTime,
                                        } => KMSMasterKeyArn,
                                    }
                                }
                                pub fn InaccessibleEncryptionDateTime(
                                    &self,
                                ) -> &::std::rc::Rc<
                                    crate::r#_Wrappers_Compile::Option<
                                        ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                    >,
                                > {
                                    match self {
                                        SSEDescription::SSEDescription {
                                            Status,
                                            SSEType,
                                            KMSMasterKeyArn,
                                            InaccessibleEncryptionDateTime,
                                        } => InaccessibleEncryptionDateTime,
                                    }
                                }
                            }

                            impl Debug for SSEDescription {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for SSEDescription {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        SSEDescription::SSEDescription {
                                            Status,
                                            SSEType,
                                            KMSMasterKeyArn,
                                            InaccessibleEncryptionDateTime,
                                        } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.SSEDescription.SSEDescription(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                Status, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                SSEType, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                KMSMasterKeyArn,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                InaccessibleEncryptionDateTime,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for SSEDescription {}

                            impl Hash for SSEDescription {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        SSEDescription::SSEDescription {
                                            Status,
                                            SSEType,
                                            KMSMasterKeyArn,
                                            InaccessibleEncryptionDateTime,
                                        } => {
                                            ::std::hash::Hash::hash(Status, _state);
                                            ::std::hash::Hash::hash(SSEType, _state);
                                            ::std::hash::Hash::hash(KMSMasterKeyArn, _state);
                                            ::std::hash::Hash::hash(
                                                InaccessibleEncryptionDateTime,
                                                _state,
                                            )
                                        }
                                    }
                                }
                            }

                            impl Default for SSEDescription {
                                fn default() -> SSEDescription {
                                    SSEDescription::SSEDescription {
                                        Status: ::std::default::Default::default(),
                                        SSEType: ::std::default::Default::default(),
                                        KMSMasterKeyArn: ::std::default::Default::default(),
                                        InaccessibleEncryptionDateTime:
                                            ::std::default::Default::default(),
                                    }
                                }
                            }

                            impl AsRef<SSEDescription> for &SSEDescription {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            #[derive(PartialEq, Clone)]
                            pub enum SSESpecification {
                                SSESpecification {
                  Enabled: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<bool>>,
                  SSEType: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::SSEType>>>,
                  KMSMasterKeyId: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>
                }
              }

                            impl SSESpecification {
                                pub fn Enabled(
                                    &self,
                                ) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<bool>>
                                {
                                    match self {
                                        SSESpecification::SSESpecification {
                                            Enabled,
                                            SSEType,
                                            KMSMasterKeyId,
                                        } => Enabled,
                                    }
                                }
                                pub fn SSEType(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::SSEType>>>{
                                    match self {
                                        SSESpecification::SSESpecification {
                                            Enabled,
                                            SSEType,
                                            KMSMasterKeyId,
                                        } => SSEType,
                                    }
                                }
                                pub fn KMSMasterKeyId(
                                    &self,
                                ) -> &::std::rc::Rc<
                                    crate::r#_Wrappers_Compile::Option<
                                        ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                    >,
                                > {
                                    match self {
                                        SSESpecification::SSESpecification {
                                            Enabled,
                                            SSEType,
                                            KMSMasterKeyId,
                                        } => KMSMasterKeyId,
                                    }
                                }
                            }

                            impl Debug for SSESpecification {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for SSESpecification {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        SSESpecification::SSESpecification {
                                            Enabled,
                                            SSEType,
                                            KMSMasterKeyId,
                                        } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.SSESpecification.SSESpecification(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                Enabled, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                SSEType, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                KMSMasterKeyId,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for SSESpecification {}

                            impl Hash for SSESpecification {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        SSESpecification::SSESpecification {
                                            Enabled,
                                            SSEType,
                                            KMSMasterKeyId,
                                        } => {
                                            ::std::hash::Hash::hash(Enabled, _state);
                                            ::std::hash::Hash::hash(SSEType, _state);
                                            ::std::hash::Hash::hash(KMSMasterKeyId, _state)
                                        }
                                    }
                                }
                            }

                            impl Default for SSESpecification {
                                fn default() -> SSESpecification {
                                    SSESpecification::SSESpecification {
                                        Enabled: ::std::default::Default::default(),
                                        SSEType: ::std::default::Default::default(),
                                        KMSMasterKeyId: ::std::default::Default::default(),
                                    }
                                }
                            }

                            impl AsRef<SSESpecification> for &SSESpecification {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            #[derive(PartialEq, Clone)]
                            pub enum SSEStatus {
                                ENABLING {},
                                ENABLED {},
                                DISABLING {},
                                DISABLED {},
                                UPDATING {},
                            }

                            impl SSEStatus {}

                            impl Debug for SSEStatus {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for SSEStatus {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        SSEStatus::ENABLING {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.SSEStatus.ENABLING")?;
                                            Ok(())
                                        }
                                        SSEStatus::ENABLED {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.SSEStatus.ENABLED")?;
                                            Ok(())
                                        }
                                        SSEStatus::DISABLING {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.SSEStatus.DISABLING")?;
                                            Ok(())
                                        }
                                        SSEStatus::DISABLED {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.SSEStatus.DISABLED")?;
                                            Ok(())
                                        }
                                        SSEStatus::UPDATING {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.SSEStatus.UPDATING")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for SSEStatus {}

                            impl Hash for SSEStatus {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        SSEStatus::ENABLING {} => {}
                                        SSEStatus::ENABLED {} => {}
                                        SSEStatus::DISABLING {} => {}
                                        SSEStatus::DISABLED {} => {}
                                        SSEStatus::UPDATING {} => {}
                                    }
                                }
                            }

                            impl Default for SSEStatus {
                                fn default() -> SSEStatus {
                                    SSEStatus::ENABLING {}
                                }
                            }

                            impl AsRef<SSEStatus> for &SSEStatus {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            #[derive(PartialEq, Clone)]
                            pub enum SSEType {
                                AES256 {},
                                KMS {},
                            }

                            impl SSEType {}

                            impl Debug for SSEType {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for SSEType {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        SSEType::AES256 {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.SSEType.AES256")?;
                                            Ok(())
                                        }
                                        SSEType::KMS {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.SSEType.KMS")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for SSEType {}

                            impl Hash for SSEType {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        SSEType::AES256 {} => {}
                                        SSEType::KMS {} => {}
                                    }
                                }
                            }

                            impl Default for SSEType {
                                fn default() -> SSEType {
                                    SSEType::AES256 {}
                                }
                            }

                            impl AsRef<SSEType> for &SSEType {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            pub type StreamArn =
                                ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>;

                            #[derive(PartialEq, Clone)]
                            pub enum StreamSpecification {
                                StreamSpecification {
                  StreamEnabled: bool,
                  StreamViewType: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::StreamViewType>>>
                }
              }

                            impl StreamSpecification {
                                pub fn StreamEnabled(&self) -> &bool {
                                    match self {
                                        StreamSpecification::StreamSpecification {
                                            StreamEnabled,
                                            StreamViewType,
                                        } => StreamEnabled,
                                    }
                                }
                                pub fn StreamViewType(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::StreamViewType>>>{
                                    match self {
                                        StreamSpecification::StreamSpecification {
                                            StreamEnabled,
                                            StreamViewType,
                                        } => StreamViewType,
                                    }
                                }
                            }

                            impl Debug for StreamSpecification {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for StreamSpecification {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        StreamSpecification::StreamSpecification {
                                            StreamEnabled,
                                            StreamViewType,
                                        } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.StreamSpecification.StreamSpecification(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                StreamEnabled,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                StreamViewType,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for StreamSpecification {}

                            impl Hash for StreamSpecification {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        StreamSpecification::StreamSpecification {
                                            StreamEnabled,
                                            StreamViewType,
                                        } => {
                                            ::std::hash::Hash::hash(StreamEnabled, _state);
                                            ::std::hash::Hash::hash(StreamViewType, _state)
                                        }
                                    }
                                }
                            }

                            impl Default for StreamSpecification {
                                fn default() -> StreamSpecification {
                                    StreamSpecification::StreamSpecification {
                                        StreamEnabled: ::std::default::Default::default(),
                                        StreamViewType: ::std::default::Default::default(),
                                    }
                                }
                            }

                            impl AsRef<StreamSpecification> for &StreamSpecification {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            #[derive(PartialEq, Clone)]
                            pub enum StreamViewType {
                                NEW_IMAGE {},
                                OLD_IMAGE {},
                                NEW_AND_OLD_IMAGES {},
                                KEYS_ONLY {},
                            }

                            impl StreamViewType {}

                            impl Debug for StreamViewType {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for StreamViewType {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        StreamViewType::NEW_IMAGE {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.StreamViewType.NEW__IMAGE")?;
                                            Ok(())
                                        }
                                        StreamViewType::OLD_IMAGE {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.StreamViewType.OLD__IMAGE")?;
                                            Ok(())
                                        }
                                        StreamViewType::NEW_AND_OLD_IMAGES {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.StreamViewType.NEW__AND__OLD__IMAGES")?;
                                            Ok(())
                                        }
                                        StreamViewType::KEYS_ONLY {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.StreamViewType.KEYS__ONLY")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for StreamViewType {}

                            impl Hash for StreamViewType {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        StreamViewType::NEW_IMAGE {} => {}
                                        StreamViewType::OLD_IMAGE {} => {}
                                        StreamViewType::NEW_AND_OLD_IMAGES {} => {}
                                        StreamViewType::KEYS_ONLY {} => {}
                                    }
                                }
                            }

                            impl Default for StreamViewType {
                                fn default() -> StreamViewType {
                                    StreamViewType::NEW_IMAGE {}
                                }
                            }

                            impl AsRef<StreamViewType> for &StreamViewType {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            #[derive(PartialEq, Clone)]
                            pub enum TableClass {
                                STANDARD {},
                                STANDARD_INFREQUENT_ACCESS {},
                            }

                            impl TableClass {}

                            impl Debug for TableClass {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for TableClass {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        TableClass::STANDARD {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.TableClass.STANDARD")?;
                                            Ok(())
                                        }
                                        TableClass::STANDARD_INFREQUENT_ACCESS {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.TableClass.STANDARD__INFREQUENT__ACCESS")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for TableClass {}

                            impl Hash for TableClass {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        TableClass::STANDARD {} => {}
                                        TableClass::STANDARD_INFREQUENT_ACCESS {} => {}
                                    }
                                }
                            }

                            impl Default for TableClass {
                                fn default() -> TableClass {
                                    TableClass::STANDARD {}
                                }
                            }

                            impl AsRef<TableClass> for &TableClass {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            #[derive(PartialEq, Clone)]
                            pub enum TableClassSummary {
                                TableClassSummary {
                  TableClass: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::TableClass>>>,
                  LastUpdateDateTime: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>
                }
              }

                            impl TableClassSummary {
                                pub fn TableClass(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::TableClass>>>{
                                    match self {
                                        TableClassSummary::TableClassSummary {
                                            TableClass,
                                            LastUpdateDateTime,
                                        } => TableClass,
                                    }
                                }
                                pub fn LastUpdateDateTime(
                                    &self,
                                ) -> &::std::rc::Rc<
                                    crate::r#_Wrappers_Compile::Option<
                                        ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                    >,
                                > {
                                    match self {
                                        TableClassSummary::TableClassSummary {
                                            TableClass,
                                            LastUpdateDateTime,
                                        } => LastUpdateDateTime,
                                    }
                                }
                            }

                            impl Debug for TableClassSummary {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for TableClassSummary {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        TableClassSummary::TableClassSummary {
                                            TableClass,
                                            LastUpdateDateTime,
                                        } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.TableClassSummary.TableClassSummary(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                TableClass, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                LastUpdateDateTime,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for TableClassSummary {}

                            impl Hash for TableClassSummary {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        TableClassSummary::TableClassSummary {
                                            TableClass,
                                            LastUpdateDateTime,
                                        } => {
                                            ::std::hash::Hash::hash(TableClass, _state);
                                            ::std::hash::Hash::hash(LastUpdateDateTime, _state)
                                        }
                                    }
                                }
                            }

                            impl Default for TableClassSummary {
                                fn default() -> TableClassSummary {
                                    TableClassSummary::TableClassSummary {
                                        TableClass: ::std::default::Default::default(),
                                        LastUpdateDateTime: ::std::default::Default::default(),
                                    }
                                }
                            }

                            impl AsRef<TableClassSummary> for &TableClassSummary {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            #[derive(PartialEq, Clone)]
                            pub enum TableDescription {
                                TableDescription {
                  AttributeDefinitions: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeDefinition>>>>,
                  TableName: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>,
                  KeySchema: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::KeySchema>>,
                  TableStatus: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::TableStatus>>>,
                  CreationDateTime: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>,
                  ProvisionedThroughput: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ProvisionedThroughputDescription>>>,
                  TableSizeBytes: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<i64>>,
                  ItemCount: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<i64>>,
                  TableArn: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>,
                  TableId: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>,
                  BillingModeSummary: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::BillingModeSummary>>>,
                  LocalSecondaryIndexes: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::LocalSecondaryIndexDescription>>>>,
                  GlobalSecondaryIndexes: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::GlobalSecondaryIndexDescription>>>>,
                  StreamSpecification: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::StreamSpecification>>>,
                  LatestStreamLabel: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>,
                  LatestStreamArn: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>,
                  GlobalTableVersion: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>,
                  Replicas: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ReplicaDescription>>>>,
                  RestoreSummary: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::RestoreSummary>>>,
                  SSEDescription: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::SSEDescription>>>,
                  ArchivalSummary: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ArchivalSummary>>>,
                  TableClassSummary: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::TableClassSummary>>>
                }
              }

                            impl TableDescription {
                                pub fn AttributeDefinitions(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeDefinition>>>>{
                                    match self {
                                        TableDescription::TableDescription {
                                            AttributeDefinitions,
                                            TableName,
                                            KeySchema,
                                            TableStatus,
                                            CreationDateTime,
                                            ProvisionedThroughput,
                                            TableSizeBytes,
                                            ItemCount,
                                            TableArn,
                                            TableId,
                                            BillingModeSummary,
                                            LocalSecondaryIndexes,
                                            GlobalSecondaryIndexes,
                                            StreamSpecification,
                                            LatestStreamLabel,
                                            LatestStreamArn,
                                            GlobalTableVersion,
                                            Replicas,
                                            RestoreSummary,
                                            SSEDescription,
                                            ArchivalSummary,
                                            TableClassSummary,
                                        } => AttributeDefinitions,
                                    }
                                }
                                pub fn TableName(
                                    &self,
                                ) -> &::std::rc::Rc<
                                    crate::r#_Wrappers_Compile::Option<
                                        ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                    >,
                                > {
                                    match self {
                                        TableDescription::TableDescription {
                                            AttributeDefinitions,
                                            TableName,
                                            KeySchema,
                                            TableStatus,
                                            CreationDateTime,
                                            ProvisionedThroughput,
                                            TableSizeBytes,
                                            ItemCount,
                                            TableArn,
                                            TableId,
                                            BillingModeSummary,
                                            LocalSecondaryIndexes,
                                            GlobalSecondaryIndexes,
                                            StreamSpecification,
                                            LatestStreamLabel,
                                            LatestStreamArn,
                                            GlobalTableVersion,
                                            Replicas,
                                            RestoreSummary,
                                            SSEDescription,
                                            ArchivalSummary,
                                            TableClassSummary,
                                        } => TableName,
                                    }
                                }
                                pub fn KeySchema(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::KeySchema>>{
                                    match self {
                                        TableDescription::TableDescription {
                                            AttributeDefinitions,
                                            TableName,
                                            KeySchema,
                                            TableStatus,
                                            CreationDateTime,
                                            ProvisionedThroughput,
                                            TableSizeBytes,
                                            ItemCount,
                                            TableArn,
                                            TableId,
                                            BillingModeSummary,
                                            LocalSecondaryIndexes,
                                            GlobalSecondaryIndexes,
                                            StreamSpecification,
                                            LatestStreamLabel,
                                            LatestStreamArn,
                                            GlobalTableVersion,
                                            Replicas,
                                            RestoreSummary,
                                            SSEDescription,
                                            ArchivalSummary,
                                            TableClassSummary,
                                        } => KeySchema,
                                    }
                                }
                                pub fn TableStatus(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::TableStatus>>>{
                                    match self {
                                        TableDescription::TableDescription {
                                            AttributeDefinitions,
                                            TableName,
                                            KeySchema,
                                            TableStatus,
                                            CreationDateTime,
                                            ProvisionedThroughput,
                                            TableSizeBytes,
                                            ItemCount,
                                            TableArn,
                                            TableId,
                                            BillingModeSummary,
                                            LocalSecondaryIndexes,
                                            GlobalSecondaryIndexes,
                                            StreamSpecification,
                                            LatestStreamLabel,
                                            LatestStreamArn,
                                            GlobalTableVersion,
                                            Replicas,
                                            RestoreSummary,
                                            SSEDescription,
                                            ArchivalSummary,
                                            TableClassSummary,
                                        } => TableStatus,
                                    }
                                }
                                pub fn CreationDateTime(
                                    &self,
                                ) -> &::std::rc::Rc<
                                    crate::r#_Wrappers_Compile::Option<
                                        ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                    >,
                                > {
                                    match self {
                                        TableDescription::TableDescription {
                                            AttributeDefinitions,
                                            TableName,
                                            KeySchema,
                                            TableStatus,
                                            CreationDateTime,
                                            ProvisionedThroughput,
                                            TableSizeBytes,
                                            ItemCount,
                                            TableArn,
                                            TableId,
                                            BillingModeSummary,
                                            LocalSecondaryIndexes,
                                            GlobalSecondaryIndexes,
                                            StreamSpecification,
                                            LatestStreamLabel,
                                            LatestStreamArn,
                                            GlobalTableVersion,
                                            Replicas,
                                            RestoreSummary,
                                            SSEDescription,
                                            ArchivalSummary,
                                            TableClassSummary,
                                        } => CreationDateTime,
                                    }
                                }
                                pub fn ProvisionedThroughput(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ProvisionedThroughputDescription>>>{
                                    match self {
                                        TableDescription::TableDescription {
                                            AttributeDefinitions,
                                            TableName,
                                            KeySchema,
                                            TableStatus,
                                            CreationDateTime,
                                            ProvisionedThroughput,
                                            TableSizeBytes,
                                            ItemCount,
                                            TableArn,
                                            TableId,
                                            BillingModeSummary,
                                            LocalSecondaryIndexes,
                                            GlobalSecondaryIndexes,
                                            StreamSpecification,
                                            LatestStreamLabel,
                                            LatestStreamArn,
                                            GlobalTableVersion,
                                            Replicas,
                                            RestoreSummary,
                                            SSEDescription,
                                            ArchivalSummary,
                                            TableClassSummary,
                                        } => ProvisionedThroughput,
                                    }
                                }
                                pub fn TableSizeBytes(
                                    &self,
                                ) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<i64>>
                                {
                                    match self {
                                        TableDescription::TableDescription {
                                            AttributeDefinitions,
                                            TableName,
                                            KeySchema,
                                            TableStatus,
                                            CreationDateTime,
                                            ProvisionedThroughput,
                                            TableSizeBytes,
                                            ItemCount,
                                            TableArn,
                                            TableId,
                                            BillingModeSummary,
                                            LocalSecondaryIndexes,
                                            GlobalSecondaryIndexes,
                                            StreamSpecification,
                                            LatestStreamLabel,
                                            LatestStreamArn,
                                            GlobalTableVersion,
                                            Replicas,
                                            RestoreSummary,
                                            SSEDescription,
                                            ArchivalSummary,
                                            TableClassSummary,
                                        } => TableSizeBytes,
                                    }
                                }
                                pub fn ItemCount(
                                    &self,
                                ) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<i64>>
                                {
                                    match self {
                                        TableDescription::TableDescription {
                                            AttributeDefinitions,
                                            TableName,
                                            KeySchema,
                                            TableStatus,
                                            CreationDateTime,
                                            ProvisionedThroughput,
                                            TableSizeBytes,
                                            ItemCount,
                                            TableArn,
                                            TableId,
                                            BillingModeSummary,
                                            LocalSecondaryIndexes,
                                            GlobalSecondaryIndexes,
                                            StreamSpecification,
                                            LatestStreamLabel,
                                            LatestStreamArn,
                                            GlobalTableVersion,
                                            Replicas,
                                            RestoreSummary,
                                            SSEDescription,
                                            ArchivalSummary,
                                            TableClassSummary,
                                        } => ItemCount,
                                    }
                                }
                                pub fn TableArn(
                                    &self,
                                ) -> &::std::rc::Rc<
                                    crate::r#_Wrappers_Compile::Option<
                                        ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                    >,
                                > {
                                    match self {
                                        TableDescription::TableDescription {
                                            AttributeDefinitions,
                                            TableName,
                                            KeySchema,
                                            TableStatus,
                                            CreationDateTime,
                                            ProvisionedThroughput,
                                            TableSizeBytes,
                                            ItemCount,
                                            TableArn,
                                            TableId,
                                            BillingModeSummary,
                                            LocalSecondaryIndexes,
                                            GlobalSecondaryIndexes,
                                            StreamSpecification,
                                            LatestStreamLabel,
                                            LatestStreamArn,
                                            GlobalTableVersion,
                                            Replicas,
                                            RestoreSummary,
                                            SSEDescription,
                                            ArchivalSummary,
                                            TableClassSummary,
                                        } => TableArn,
                                    }
                                }
                                pub fn TableId(
                                    &self,
                                ) -> &::std::rc::Rc<
                                    crate::r#_Wrappers_Compile::Option<
                                        ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                    >,
                                > {
                                    match self {
                                        TableDescription::TableDescription {
                                            AttributeDefinitions,
                                            TableName,
                                            KeySchema,
                                            TableStatus,
                                            CreationDateTime,
                                            ProvisionedThroughput,
                                            TableSizeBytes,
                                            ItemCount,
                                            TableArn,
                                            TableId,
                                            BillingModeSummary,
                                            LocalSecondaryIndexes,
                                            GlobalSecondaryIndexes,
                                            StreamSpecification,
                                            LatestStreamLabel,
                                            LatestStreamArn,
                                            GlobalTableVersion,
                                            Replicas,
                                            RestoreSummary,
                                            SSEDescription,
                                            ArchivalSummary,
                                            TableClassSummary,
                                        } => TableId,
                                    }
                                }
                                pub fn BillingModeSummary(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::BillingModeSummary>>>{
                                    match self {
                                        TableDescription::TableDescription {
                                            AttributeDefinitions,
                                            TableName,
                                            KeySchema,
                                            TableStatus,
                                            CreationDateTime,
                                            ProvisionedThroughput,
                                            TableSizeBytes,
                                            ItemCount,
                                            TableArn,
                                            TableId,
                                            BillingModeSummary,
                                            LocalSecondaryIndexes,
                                            GlobalSecondaryIndexes,
                                            StreamSpecification,
                                            LatestStreamLabel,
                                            LatestStreamArn,
                                            GlobalTableVersion,
                                            Replicas,
                                            RestoreSummary,
                                            SSEDescription,
                                            ArchivalSummary,
                                            TableClassSummary,
                                        } => BillingModeSummary,
                                    }
                                }
                                pub fn LocalSecondaryIndexes(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::LocalSecondaryIndexDescription>>>>{
                                    match self {
                                        TableDescription::TableDescription {
                                            AttributeDefinitions,
                                            TableName,
                                            KeySchema,
                                            TableStatus,
                                            CreationDateTime,
                                            ProvisionedThroughput,
                                            TableSizeBytes,
                                            ItemCount,
                                            TableArn,
                                            TableId,
                                            BillingModeSummary,
                                            LocalSecondaryIndexes,
                                            GlobalSecondaryIndexes,
                                            StreamSpecification,
                                            LatestStreamLabel,
                                            LatestStreamArn,
                                            GlobalTableVersion,
                                            Replicas,
                                            RestoreSummary,
                                            SSEDescription,
                                            ArchivalSummary,
                                            TableClassSummary,
                                        } => LocalSecondaryIndexes,
                                    }
                                }
                                pub fn GlobalSecondaryIndexes(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::GlobalSecondaryIndexDescription>>>>{
                                    match self {
                                        TableDescription::TableDescription {
                                            AttributeDefinitions,
                                            TableName,
                                            KeySchema,
                                            TableStatus,
                                            CreationDateTime,
                                            ProvisionedThroughput,
                                            TableSizeBytes,
                                            ItemCount,
                                            TableArn,
                                            TableId,
                                            BillingModeSummary,
                                            LocalSecondaryIndexes,
                                            GlobalSecondaryIndexes,
                                            StreamSpecification,
                                            LatestStreamLabel,
                                            LatestStreamArn,
                                            GlobalTableVersion,
                                            Replicas,
                                            RestoreSummary,
                                            SSEDescription,
                                            ArchivalSummary,
                                            TableClassSummary,
                                        } => GlobalSecondaryIndexes,
                                    }
                                }
                                pub fn StreamSpecification(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::StreamSpecification>>>{
                                    match self {
                                        TableDescription::TableDescription {
                                            AttributeDefinitions,
                                            TableName,
                                            KeySchema,
                                            TableStatus,
                                            CreationDateTime,
                                            ProvisionedThroughput,
                                            TableSizeBytes,
                                            ItemCount,
                                            TableArn,
                                            TableId,
                                            BillingModeSummary,
                                            LocalSecondaryIndexes,
                                            GlobalSecondaryIndexes,
                                            StreamSpecification,
                                            LatestStreamLabel,
                                            LatestStreamArn,
                                            GlobalTableVersion,
                                            Replicas,
                                            RestoreSummary,
                                            SSEDescription,
                                            ArchivalSummary,
                                            TableClassSummary,
                                        } => StreamSpecification,
                                    }
                                }
                                pub fn LatestStreamLabel(
                                    &self,
                                ) -> &::std::rc::Rc<
                                    crate::r#_Wrappers_Compile::Option<
                                        ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                    >,
                                > {
                                    match self {
                                        TableDescription::TableDescription {
                                            AttributeDefinitions,
                                            TableName,
                                            KeySchema,
                                            TableStatus,
                                            CreationDateTime,
                                            ProvisionedThroughput,
                                            TableSizeBytes,
                                            ItemCount,
                                            TableArn,
                                            TableId,
                                            BillingModeSummary,
                                            LocalSecondaryIndexes,
                                            GlobalSecondaryIndexes,
                                            StreamSpecification,
                                            LatestStreamLabel,
                                            LatestStreamArn,
                                            GlobalTableVersion,
                                            Replicas,
                                            RestoreSummary,
                                            SSEDescription,
                                            ArchivalSummary,
                                            TableClassSummary,
                                        } => LatestStreamLabel,
                                    }
                                }
                                pub fn LatestStreamArn(
                                    &self,
                                ) -> &::std::rc::Rc<
                                    crate::r#_Wrappers_Compile::Option<
                                        ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                    >,
                                > {
                                    match self {
                                        TableDescription::TableDescription {
                                            AttributeDefinitions,
                                            TableName,
                                            KeySchema,
                                            TableStatus,
                                            CreationDateTime,
                                            ProvisionedThroughput,
                                            TableSizeBytes,
                                            ItemCount,
                                            TableArn,
                                            TableId,
                                            BillingModeSummary,
                                            LocalSecondaryIndexes,
                                            GlobalSecondaryIndexes,
                                            StreamSpecification,
                                            LatestStreamLabel,
                                            LatestStreamArn,
                                            GlobalTableVersion,
                                            Replicas,
                                            RestoreSummary,
                                            SSEDescription,
                                            ArchivalSummary,
                                            TableClassSummary,
                                        } => LatestStreamArn,
                                    }
                                }
                                pub fn GlobalTableVersion(
                                    &self,
                                ) -> &::std::rc::Rc<
                                    crate::r#_Wrappers_Compile::Option<
                                        ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                    >,
                                > {
                                    match self {
                                        TableDescription::TableDescription {
                                            AttributeDefinitions,
                                            TableName,
                                            KeySchema,
                                            TableStatus,
                                            CreationDateTime,
                                            ProvisionedThroughput,
                                            TableSizeBytes,
                                            ItemCount,
                                            TableArn,
                                            TableId,
                                            BillingModeSummary,
                                            LocalSecondaryIndexes,
                                            GlobalSecondaryIndexes,
                                            StreamSpecification,
                                            LatestStreamLabel,
                                            LatestStreamArn,
                                            GlobalTableVersion,
                                            Replicas,
                                            RestoreSummary,
                                            SSEDescription,
                                            ArchivalSummary,
                                            TableClassSummary,
                                        } => GlobalTableVersion,
                                    }
                                }
                                pub fn Replicas(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ReplicaDescription>>>>{
                                    match self {
                                        TableDescription::TableDescription {
                                            AttributeDefinitions,
                                            TableName,
                                            KeySchema,
                                            TableStatus,
                                            CreationDateTime,
                                            ProvisionedThroughput,
                                            TableSizeBytes,
                                            ItemCount,
                                            TableArn,
                                            TableId,
                                            BillingModeSummary,
                                            LocalSecondaryIndexes,
                                            GlobalSecondaryIndexes,
                                            StreamSpecification,
                                            LatestStreamLabel,
                                            LatestStreamArn,
                                            GlobalTableVersion,
                                            Replicas,
                                            RestoreSummary,
                                            SSEDescription,
                                            ArchivalSummary,
                                            TableClassSummary,
                                        } => Replicas,
                                    }
                                }
                                pub fn RestoreSummary(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::RestoreSummary>>>{
                                    match self {
                                        TableDescription::TableDescription {
                                            AttributeDefinitions,
                                            TableName,
                                            KeySchema,
                                            TableStatus,
                                            CreationDateTime,
                                            ProvisionedThroughput,
                                            TableSizeBytes,
                                            ItemCount,
                                            TableArn,
                                            TableId,
                                            BillingModeSummary,
                                            LocalSecondaryIndexes,
                                            GlobalSecondaryIndexes,
                                            StreamSpecification,
                                            LatestStreamLabel,
                                            LatestStreamArn,
                                            GlobalTableVersion,
                                            Replicas,
                                            RestoreSummary,
                                            SSEDescription,
                                            ArchivalSummary,
                                            TableClassSummary,
                                        } => RestoreSummary,
                                    }
                                }
                                pub fn SSEDescription(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::SSEDescription>>>{
                                    match self {
                                        TableDescription::TableDescription {
                                            AttributeDefinitions,
                                            TableName,
                                            KeySchema,
                                            TableStatus,
                                            CreationDateTime,
                                            ProvisionedThroughput,
                                            TableSizeBytes,
                                            ItemCount,
                                            TableArn,
                                            TableId,
                                            BillingModeSummary,
                                            LocalSecondaryIndexes,
                                            GlobalSecondaryIndexes,
                                            StreamSpecification,
                                            LatestStreamLabel,
                                            LatestStreamArn,
                                            GlobalTableVersion,
                                            Replicas,
                                            RestoreSummary,
                                            SSEDescription,
                                            ArchivalSummary,
                                            TableClassSummary,
                                        } => SSEDescription,
                                    }
                                }
                                pub fn ArchivalSummary(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ArchivalSummary>>>{
                                    match self {
                                        TableDescription::TableDescription {
                                            AttributeDefinitions,
                                            TableName,
                                            KeySchema,
                                            TableStatus,
                                            CreationDateTime,
                                            ProvisionedThroughput,
                                            TableSizeBytes,
                                            ItemCount,
                                            TableArn,
                                            TableId,
                                            BillingModeSummary,
                                            LocalSecondaryIndexes,
                                            GlobalSecondaryIndexes,
                                            StreamSpecification,
                                            LatestStreamLabel,
                                            LatestStreamArn,
                                            GlobalTableVersion,
                                            Replicas,
                                            RestoreSummary,
                                            SSEDescription,
                                            ArchivalSummary,
                                            TableClassSummary,
                                        } => ArchivalSummary,
                                    }
                                }
                                pub fn TableClassSummary(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::TableClassSummary>>>{
                                    match self {
                                        TableDescription::TableDescription {
                                            AttributeDefinitions,
                                            TableName,
                                            KeySchema,
                                            TableStatus,
                                            CreationDateTime,
                                            ProvisionedThroughput,
                                            TableSizeBytes,
                                            ItemCount,
                                            TableArn,
                                            TableId,
                                            BillingModeSummary,
                                            LocalSecondaryIndexes,
                                            GlobalSecondaryIndexes,
                                            StreamSpecification,
                                            LatestStreamLabel,
                                            LatestStreamArn,
                                            GlobalTableVersion,
                                            Replicas,
                                            RestoreSummary,
                                            SSEDescription,
                                            ArchivalSummary,
                                            TableClassSummary,
                                        } => TableClassSummary,
                                    }
                                }
                            }

                            impl Debug for TableDescription {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for TableDescription {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        TableDescription::TableDescription {
                                            AttributeDefinitions,
                                            TableName,
                                            KeySchema,
                                            TableStatus,
                                            CreationDateTime,
                                            ProvisionedThroughput,
                                            TableSizeBytes,
                                            ItemCount,
                                            TableArn,
                                            TableId,
                                            BillingModeSummary,
                                            LocalSecondaryIndexes,
                                            GlobalSecondaryIndexes,
                                            StreamSpecification,
                                            LatestStreamLabel,
                                            LatestStreamArn,
                                            GlobalTableVersion,
                                            Replicas,
                                            RestoreSummary,
                                            SSEDescription,
                                            ArchivalSummary,
                                            TableClassSummary,
                                        } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.TableDescription.TableDescription(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                AttributeDefinitions,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                TableName, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                KeySchema, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                TableStatus,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                CreationDateTime,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ProvisionedThroughput,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                TableSizeBytes,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ItemCount, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                TableArn, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                TableId, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                BillingModeSummary,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                LocalSecondaryIndexes,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                GlobalSecondaryIndexes,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                StreamSpecification,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                LatestStreamLabel,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                LatestStreamArn,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                GlobalTableVersion,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                Replicas, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                RestoreSummary,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                SSEDescription,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ArchivalSummary,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                TableClassSummary,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for TableDescription {}

                            impl Hash for TableDescription {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        TableDescription::TableDescription {
                                            AttributeDefinitions,
                                            TableName,
                                            KeySchema,
                                            TableStatus,
                                            CreationDateTime,
                                            ProvisionedThroughput,
                                            TableSizeBytes,
                                            ItemCount,
                                            TableArn,
                                            TableId,
                                            BillingModeSummary,
                                            LocalSecondaryIndexes,
                                            GlobalSecondaryIndexes,
                                            StreamSpecification,
                                            LatestStreamLabel,
                                            LatestStreamArn,
                                            GlobalTableVersion,
                                            Replicas,
                                            RestoreSummary,
                                            SSEDescription,
                                            ArchivalSummary,
                                            TableClassSummary,
                                        } => {
                                            ::std::hash::Hash::hash(AttributeDefinitions, _state);
                                            ::std::hash::Hash::hash(TableName, _state);
                                            ::std::hash::Hash::hash(KeySchema, _state);
                                            ::std::hash::Hash::hash(TableStatus, _state);
                                            ::std::hash::Hash::hash(CreationDateTime, _state);
                                            ::std::hash::Hash::hash(ProvisionedThroughput, _state);
                                            ::std::hash::Hash::hash(TableSizeBytes, _state);
                                            ::std::hash::Hash::hash(ItemCount, _state);
                                            ::std::hash::Hash::hash(TableArn, _state);
                                            ::std::hash::Hash::hash(TableId, _state);
                                            ::std::hash::Hash::hash(BillingModeSummary, _state);
                                            ::std::hash::Hash::hash(LocalSecondaryIndexes, _state);
                                            ::std::hash::Hash::hash(GlobalSecondaryIndexes, _state);
                                            ::std::hash::Hash::hash(StreamSpecification, _state);
                                            ::std::hash::Hash::hash(LatestStreamLabel, _state);
                                            ::std::hash::Hash::hash(LatestStreamArn, _state);
                                            ::std::hash::Hash::hash(GlobalTableVersion, _state);
                                            ::std::hash::Hash::hash(Replicas, _state);
                                            ::std::hash::Hash::hash(RestoreSummary, _state);
                                            ::std::hash::Hash::hash(SSEDescription, _state);
                                            ::std::hash::Hash::hash(ArchivalSummary, _state);
                                            ::std::hash::Hash::hash(TableClassSummary, _state)
                                        }
                                    }
                                }
                            }

                            impl Default for TableDescription {
                                fn default() -> TableDescription {
                                    TableDescription::TableDescription {
                                        AttributeDefinitions: ::std::default::Default::default(),
                                        TableName: ::std::default::Default::default(),
                                        KeySchema: ::std::default::Default::default(),
                                        TableStatus: ::std::default::Default::default(),
                                        CreationDateTime: ::std::default::Default::default(),
                                        ProvisionedThroughput: ::std::default::Default::default(),
                                        TableSizeBytes: ::std::default::Default::default(),
                                        ItemCount: ::std::default::Default::default(),
                                        TableArn: ::std::default::Default::default(),
                                        TableId: ::std::default::Default::default(),
                                        BillingModeSummary: ::std::default::Default::default(),
                                        LocalSecondaryIndexes: ::std::default::Default::default(),
                                        GlobalSecondaryIndexes: ::std::default::Default::default(),
                                        StreamSpecification: ::std::default::Default::default(),
                                        LatestStreamLabel: ::std::default::Default::default(),
                                        LatestStreamArn: ::std::default::Default::default(),
                                        GlobalTableVersion: ::std::default::Default::default(),
                                        Replicas: ::std::default::Default::default(),
                                        RestoreSummary: ::std::default::Default::default(),
                                        SSEDescription: ::std::default::Default::default(),
                                        ArchivalSummary: ::std::default::Default::default(),
                                        TableClassSummary: ::std::default::Default::default(),
                                    }
                                }
                            }

                            impl AsRef<TableDescription> for &TableDescription {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            pub type TableName =
                                ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>;

                            #[derive(PartialEq, Clone)]
                            pub enum TableStatus {
                                CREATING {},
                                UPDATING {},
                                DELETING {},
                                ACTIVE {},
                                INACCESSIBLE_ENCRYPTION_CREDENTIALS {},
                                ARCHIVING {},
                                ARCHIVED {},
                            }

                            impl TableStatus {}

                            impl Debug for TableStatus {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for TableStatus {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        TableStatus::CREATING {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.TableStatus.CREATING")?;
                                            Ok(())
                                        }
                                        TableStatus::UPDATING {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.TableStatus.UPDATING")?;
                                            Ok(())
                                        }
                                        TableStatus::DELETING {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.TableStatus.DELETING")?;
                                            Ok(())
                                        }
                                        TableStatus::ACTIVE {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.TableStatus.ACTIVE")?;
                                            Ok(())
                                        }
                                        TableStatus::INACCESSIBLE_ENCRYPTION_CREDENTIALS {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.TableStatus.INACCESSIBLE__ENCRYPTION__CREDENTIALS")?;
                                            Ok(())
                                        }
                                        TableStatus::ARCHIVING {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.TableStatus.ARCHIVING")?;
                                            Ok(())
                                        }
                                        TableStatus::ARCHIVED {} => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.TableStatus.ARCHIVED")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for TableStatus {}

                            impl Hash for TableStatus {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        TableStatus::CREATING {} => {}
                                        TableStatus::UPDATING {} => {}
                                        TableStatus::DELETING {} => {}
                                        TableStatus::ACTIVE {} => {}
                                        TableStatus::INACCESSIBLE_ENCRYPTION_CREDENTIALS {} => {}
                                        TableStatus::ARCHIVING {} => {}
                                        TableStatus::ARCHIVED {} => {}
                                    }
                                }
                            }

                            impl Default for TableStatus {
                                fn default() -> TableStatus {
                                    TableStatus::CREATING {}
                                }
                            }

                            impl AsRef<TableStatus> for &TableStatus {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            #[derive(PartialEq, Clone)]
                            pub enum Tag {
                                Tag {
                                    Key: ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                    Value:
                                        ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                },
                            }

                            impl Tag {
                                pub fn Key(
                                    &self,
                                ) -> &::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>
                                {
                                    match self {
                                        Tag::Tag { Key, Value } => Key,
                                    }
                                }
                                pub fn Value(
                                    &self,
                                ) -> &::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>
                                {
                                    match self {
                                        Tag::Tag { Key, Value } => Value,
                                    }
                                }
                            }

                            impl Debug for Tag {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for Tag {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        Tag::Tag { Key, Value } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.Tag.Tag(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                Key, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                Value, _formatter, false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for Tag {}

                            impl Hash for Tag {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        Tag::Tag { Key, Value } => {
                                            ::std::hash::Hash::hash(Key, _state);
                                            ::std::hash::Hash::hash(Value, _state)
                                        }
                                    }
                                }
                            }

                            impl Default for Tag {
                                fn default() -> Tag {
                                    Tag::Tag {
                                        Key: ::std::default::Default::default(),
                                        Value: ::std::default::Default::default(),
                                    }
                                }
                            }

                            impl AsRef<Tag> for &Tag {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            pub type TagKeyString =
                                ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>;

                            pub type TagValueString =
                                ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>;

                            #[derive(PartialEq, Clone)]
                            pub enum TransactWriteItem {
                                TransactWriteItem {
                  ConditionCheck: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ConditionCheck>>>,
                  Put: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Put>>>,
                  Delete: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Delete>>>,
                  Update: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Update>>>
                }
              }

                            impl TransactWriteItem {
                                pub fn ConditionCheck(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ConditionCheck>>>{
                                    match self {
                                        TransactWriteItem::TransactWriteItem {
                                            ConditionCheck,
                                            Put,
                                            Delete,
                                            Update,
                                        } => ConditionCheck,
                                    }
                                }
                                pub fn Put(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Put>>>{
                                    match self {
                                        TransactWriteItem::TransactWriteItem {
                                            ConditionCheck,
                                            Put,
                                            Delete,
                                            Update,
                                        } => Put,
                                    }
                                }
                                pub fn Delete(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Delete>>>{
                                    match self {
                                        TransactWriteItem::TransactWriteItem {
                                            ConditionCheck,
                                            Put,
                                            Delete,
                                            Update,
                                        } => Delete,
                                    }
                                }
                                pub fn Update(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Update>>>{
                                    match self {
                                        TransactWriteItem::TransactWriteItem {
                                            ConditionCheck,
                                            Put,
                                            Delete,
                                            Update,
                                        } => Update,
                                    }
                                }
                            }

                            impl Debug for TransactWriteItem {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for TransactWriteItem {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        TransactWriteItem::TransactWriteItem {
                                            ConditionCheck,
                                            Put,
                                            Delete,
                                            Update,
                                        } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.TransactWriteItem.TransactWriteItem(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ConditionCheck,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                Put, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                Delete, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                Update, _formatter, false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for TransactWriteItem {}

                            impl Hash for TransactWriteItem {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        TransactWriteItem::TransactWriteItem {
                                            ConditionCheck,
                                            Put,
                                            Delete,
                                            Update,
                                        } => {
                                            ::std::hash::Hash::hash(ConditionCheck, _state);
                                            ::std::hash::Hash::hash(Put, _state);
                                            ::std::hash::Hash::hash(Delete, _state);
                                            ::std::hash::Hash::hash(Update, _state)
                                        }
                                    }
                                }
                            }

                            impl Default for TransactWriteItem {
                                fn default() -> TransactWriteItem {
                                    TransactWriteItem::TransactWriteItem {
                                        ConditionCheck: ::std::default::Default::default(),
                                        Put: ::std::default::Default::default(),
                                        Delete: ::std::default::Default::default(),
                                        Update: ::std::default::Default::default(),
                                    }
                                }
                            }

                            impl AsRef<TransactWriteItem> for &TransactWriteItem {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            pub type TransactWriteItemList = ::dafny_runtime::Sequence<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::TransactWriteItem>>;

                            #[derive(PartialEq, Clone)]
                            pub enum TransactWriteItemsInput {
                                TransactWriteItemsInput {
                  TransactItems: crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::TransactWriteItemList,
                  ReturnConsumedCapacity: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ReturnConsumedCapacity>>>,
                  ReturnItemCollectionMetrics: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ReturnItemCollectionMetrics>>>,
                  ClientRequestToken: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>
                }
              }

                            impl TransactWriteItemsInput {
                                pub fn TransactItems(&self) -> &crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::TransactWriteItemList{
                                    match self {
                                        TransactWriteItemsInput::TransactWriteItemsInput {
                                            TransactItems,
                                            ReturnConsumedCapacity,
                                            ReturnItemCollectionMetrics,
                                            ClientRequestToken,
                                        } => TransactItems,
                                    }
                                }
                                pub fn ReturnConsumedCapacity(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ReturnConsumedCapacity>>>{
                                    match self {
                                        TransactWriteItemsInput::TransactWriteItemsInput {
                                            TransactItems,
                                            ReturnConsumedCapacity,
                                            ReturnItemCollectionMetrics,
                                            ClientRequestToken,
                                        } => ReturnConsumedCapacity,
                                    }
                                }
                                pub fn ReturnItemCollectionMetrics(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ReturnItemCollectionMetrics>>>{
                                    match self {
                                        TransactWriteItemsInput::TransactWriteItemsInput {
                                            TransactItems,
                                            ReturnConsumedCapacity,
                                            ReturnItemCollectionMetrics,
                                            ClientRequestToken,
                                        } => ReturnItemCollectionMetrics,
                                    }
                                }
                                pub fn ClientRequestToken(
                                    &self,
                                ) -> &::std::rc::Rc<
                                    crate::r#_Wrappers_Compile::Option<
                                        ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                    >,
                                > {
                                    match self {
                                        TransactWriteItemsInput::TransactWriteItemsInput {
                                            TransactItems,
                                            ReturnConsumedCapacity,
                                            ReturnItemCollectionMetrics,
                                            ClientRequestToken,
                                        } => ClientRequestToken,
                                    }
                                }
                            }

                            impl Debug for TransactWriteItemsInput {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for TransactWriteItemsInput {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        TransactWriteItemsInput::TransactWriteItemsInput {
                                            TransactItems,
                                            ReturnConsumedCapacity,
                                            ReturnItemCollectionMetrics,
                                            ClientRequestToken,
                                        } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.TransactWriteItemsInput.TransactWriteItemsInput(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                TransactItems,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ReturnConsumedCapacity,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ReturnItemCollectionMetrics,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ClientRequestToken,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for TransactWriteItemsInput {}

                            impl Hash for TransactWriteItemsInput {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        TransactWriteItemsInput::TransactWriteItemsInput {
                                            TransactItems,
                                            ReturnConsumedCapacity,
                                            ReturnItemCollectionMetrics,
                                            ClientRequestToken,
                                        } => {
                                            ::std::hash::Hash::hash(TransactItems, _state);
                                            ::std::hash::Hash::hash(ReturnConsumedCapacity, _state);
                                            ::std::hash::Hash::hash(
                                                ReturnItemCollectionMetrics,
                                                _state,
                                            );
                                            ::std::hash::Hash::hash(ClientRequestToken, _state)
                                        }
                                    }
                                }
                            }

                            impl Default for TransactWriteItemsInput {
                                fn default() -> TransactWriteItemsInput {
                                    TransactWriteItemsInput::TransactWriteItemsInput {
                                        TransactItems: ::std::default::Default::default(),
                                        ReturnConsumedCapacity: ::std::default::Default::default(),
                                        ReturnItemCollectionMetrics:
                                            ::std::default::Default::default(),
                                        ClientRequestToken: ::std::default::Default::default(),
                                    }
                                }
                            }

                            impl AsRef<TransactWriteItemsInput> for &TransactWriteItemsInput {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            #[derive(PartialEq, Clone)]
                            pub enum TransactWriteItemsOutput {
                                TransactWriteItemsOutput {
                  ConsumedCapacity: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ConsumedCapacity>>>>,
                  ItemCollectionMetrics: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::dafny_runtime::Sequence<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ItemCollectionMetrics>>>>>
                }
              }

                            impl TransactWriteItemsOutput {
                                pub fn ConsumedCapacity(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ConsumedCapacity>>>>{
                                    match self {
                                        TransactWriteItemsOutput::TransactWriteItemsOutput {
                                            ConsumedCapacity,
                                            ItemCollectionMetrics,
                                        } => ConsumedCapacity,
                                    }
                                }
                                pub fn ItemCollectionMetrics(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::dafny_runtime::Sequence<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ItemCollectionMetrics>>>>>{
                                    match self {
                                        TransactWriteItemsOutput::TransactWriteItemsOutput {
                                            ConsumedCapacity,
                                            ItemCollectionMetrics,
                                        } => ItemCollectionMetrics,
                                    }
                                }
                            }

                            impl Debug for TransactWriteItemsOutput {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for TransactWriteItemsOutput {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        TransactWriteItemsOutput::TransactWriteItemsOutput {
                                            ConsumedCapacity,
                                            ItemCollectionMetrics,
                                        } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.TransactWriteItemsOutput.TransactWriteItemsOutput(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ConsumedCapacity,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ItemCollectionMetrics,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for TransactWriteItemsOutput {}

                            impl Hash for TransactWriteItemsOutput {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        TransactWriteItemsOutput::TransactWriteItemsOutput {
                                            ConsumedCapacity,
                                            ItemCollectionMetrics,
                                        } => {
                                            ::std::hash::Hash::hash(ConsumedCapacity, _state);
                                            ::std::hash::Hash::hash(ItemCollectionMetrics, _state)
                                        }
                                    }
                                }
                            }

                            impl Default for TransactWriteItemsOutput {
                                fn default() -> TransactWriteItemsOutput {
                                    TransactWriteItemsOutput::TransactWriteItemsOutput {
                                        ConsumedCapacity: ::std::default::Default::default(),
                                        ItemCollectionMetrics: ::std::default::Default::default(),
                                    }
                                }
                            }

                            impl AsRef<TransactWriteItemsOutput> for &TransactWriteItemsOutput {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            #[derive(PartialEq, Clone)]
                            pub enum Update {
                                Update {
                  Key: ::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>,
                  UpdateExpression: ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                  TableName: ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                  ConditionExpression: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>,
                  ExpressionAttributeNames: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>>,
                  ExpressionAttributeValues: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>>>,
                  ReturnValuesOnConditionCheckFailure: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ReturnValuesOnConditionCheckFailure>>>
                }
              }

                            impl Update {
                                pub fn Key(&self) -> &::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>{
                                    match self {
                                        Update::Update {
                                            Key,
                                            UpdateExpression,
                                            TableName,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                            ReturnValuesOnConditionCheckFailure,
                                        } => Key,
                                    }
                                }
                                pub fn UpdateExpression(
                                    &self,
                                ) -> &::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>
                                {
                                    match self {
                                        Update::Update {
                                            Key,
                                            UpdateExpression,
                                            TableName,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                            ReturnValuesOnConditionCheckFailure,
                                        } => UpdateExpression,
                                    }
                                }
                                pub fn TableName(
                                    &self,
                                ) -> &::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>
                                {
                                    match self {
                                        Update::Update {
                                            Key,
                                            UpdateExpression,
                                            TableName,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                            ReturnValuesOnConditionCheckFailure,
                                        } => TableName,
                                    }
                                }
                                pub fn ConditionExpression(
                                    &self,
                                ) -> &::std::rc::Rc<
                                    crate::r#_Wrappers_Compile::Option<
                                        ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                    >,
                                > {
                                    match self {
                                        Update::Update {
                                            Key,
                                            UpdateExpression,
                                            TableName,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                            ReturnValuesOnConditionCheckFailure,
                                        } => ConditionExpression,
                                    }
                                }
                                pub fn ExpressionAttributeNames(
                                    &self,
                                ) -> &::std::rc::Rc<
                                    crate::r#_Wrappers_Compile::Option<
                                        ::dafny_runtime::Map<
                                            ::dafny_runtime::Sequence<
                                                ::dafny_runtime::DafnyCharUTF16,
                                            >,
                                            ::dafny_runtime::Sequence<
                                                ::dafny_runtime::DafnyCharUTF16,
                                            >,
                                        >,
                                    >,
                                > {
                                    match self {
                                        Update::Update {
                                            Key,
                                            UpdateExpression,
                                            TableName,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                            ReturnValuesOnConditionCheckFailure,
                                        } => ExpressionAttributeNames,
                                    }
                                }
                                pub fn ExpressionAttributeValues(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>>>{
                                    match self {
                                        Update::Update {
                                            Key,
                                            UpdateExpression,
                                            TableName,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                            ReturnValuesOnConditionCheckFailure,
                                        } => ExpressionAttributeValues,
                                    }
                                }
                                pub fn ReturnValuesOnConditionCheckFailure(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ReturnValuesOnConditionCheckFailure>>>{
                                    match self {
                                        Update::Update {
                                            Key,
                                            UpdateExpression,
                                            TableName,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                            ReturnValuesOnConditionCheckFailure,
                                        } => ReturnValuesOnConditionCheckFailure,
                                    }
                                }
                            }

                            impl Debug for Update {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for Update {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        Update::Update {
                                            Key,
                                            UpdateExpression,
                                            TableName,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                            ReturnValuesOnConditionCheckFailure,
                                        } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.Update.Update(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                Key, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                UpdateExpression,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                TableName, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ConditionExpression,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ExpressionAttributeNames,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ExpressionAttributeValues,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ReturnValuesOnConditionCheckFailure,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for Update {}

                            impl Hash for Update {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        Update::Update {
                                            Key,
                                            UpdateExpression,
                                            TableName,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                            ReturnValuesOnConditionCheckFailure,
                                        } => {
                                            ::std::hash::Hash::hash(Key, _state);
                                            ::std::hash::Hash::hash(UpdateExpression, _state);
                                            ::std::hash::Hash::hash(TableName, _state);
                                            ::std::hash::Hash::hash(ConditionExpression, _state);
                                            ::std::hash::Hash::hash(
                                                ExpressionAttributeNames,
                                                _state,
                                            );
                                            ::std::hash::Hash::hash(
                                                ExpressionAttributeValues,
                                                _state,
                                            );
                                            ::std::hash::Hash::hash(
                                                ReturnValuesOnConditionCheckFailure,
                                                _state,
                                            )
                                        }
                                    }
                                }
                            }

                            impl Default for Update {
                                fn default() -> Update {
                                    Update::Update {
                                        Key: ::std::default::Default::default(),
                                        UpdateExpression: ::std::default::Default::default(),
                                        TableName: ::std::default::Default::default(),
                                        ConditionExpression: ::std::default::Default::default(),
                                        ExpressionAttributeNames: ::std::default::Default::default(
                                        ),
                                        ExpressionAttributeValues: ::std::default::Default::default(
                                        ),
                                        ReturnValuesOnConditionCheckFailure:
                                            ::std::default::Default::default(),
                                    }
                                }
                            }

                            impl AsRef<Update> for &Update {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            #[derive(PartialEq, Clone)]
                            pub enum UpdateItemInput {
                                UpdateItemInput {
                  TableName: ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                  Key: ::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>,
                  AttributeUpdates: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValueUpdate>>>>,
                  Expected: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ExpectedAttributeValue>>>>,
                  ConditionalOperator: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ConditionalOperator>>>,
                  ReturnValues: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ReturnValue>>>,
                  ReturnConsumedCapacity: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ReturnConsumedCapacity>>>,
                  ReturnItemCollectionMetrics: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ReturnItemCollectionMetrics>>>,
                  UpdateExpression: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>,
                  ConditionExpression: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>,
                  ExpressionAttributeNames: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>>,
                  ExpressionAttributeValues: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>>>
                }
              }

                            impl UpdateItemInput {
                                pub fn TableName(
                                    &self,
                                ) -> &::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>
                                {
                                    match self {
                                        UpdateItemInput::UpdateItemInput {
                                            TableName,
                                            Key,
                                            AttributeUpdates,
                                            Expected,
                                            ConditionalOperator,
                                            ReturnValues,
                                            ReturnConsumedCapacity,
                                            ReturnItemCollectionMetrics,
                                            UpdateExpression,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                        } => TableName,
                                    }
                                }
                                pub fn Key(&self) -> &::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>{
                                    match self {
                                        UpdateItemInput::UpdateItemInput {
                                            TableName,
                                            Key,
                                            AttributeUpdates,
                                            Expected,
                                            ConditionalOperator,
                                            ReturnValues,
                                            ReturnConsumedCapacity,
                                            ReturnItemCollectionMetrics,
                                            UpdateExpression,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                        } => Key,
                                    }
                                }
                                pub fn AttributeUpdates(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValueUpdate>>>>{
                                    match self {
                                        UpdateItemInput::UpdateItemInput {
                                            TableName,
                                            Key,
                                            AttributeUpdates,
                                            Expected,
                                            ConditionalOperator,
                                            ReturnValues,
                                            ReturnConsumedCapacity,
                                            ReturnItemCollectionMetrics,
                                            UpdateExpression,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                        } => AttributeUpdates,
                                    }
                                }
                                pub fn Expected(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ExpectedAttributeValue>>>>{
                                    match self {
                                        UpdateItemInput::UpdateItemInput {
                                            TableName,
                                            Key,
                                            AttributeUpdates,
                                            Expected,
                                            ConditionalOperator,
                                            ReturnValues,
                                            ReturnConsumedCapacity,
                                            ReturnItemCollectionMetrics,
                                            UpdateExpression,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                        } => Expected,
                                    }
                                }
                                pub fn ConditionalOperator(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ConditionalOperator>>>{
                                    match self {
                                        UpdateItemInput::UpdateItemInput {
                                            TableName,
                                            Key,
                                            AttributeUpdates,
                                            Expected,
                                            ConditionalOperator,
                                            ReturnValues,
                                            ReturnConsumedCapacity,
                                            ReturnItemCollectionMetrics,
                                            UpdateExpression,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                        } => ConditionalOperator,
                                    }
                                }
                                pub fn ReturnValues(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ReturnValue>>>{
                                    match self {
                                        UpdateItemInput::UpdateItemInput {
                                            TableName,
                                            Key,
                                            AttributeUpdates,
                                            Expected,
                                            ConditionalOperator,
                                            ReturnValues,
                                            ReturnConsumedCapacity,
                                            ReturnItemCollectionMetrics,
                                            UpdateExpression,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                        } => ReturnValues,
                                    }
                                }
                                pub fn ReturnConsumedCapacity(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ReturnConsumedCapacity>>>{
                                    match self {
                                        UpdateItemInput::UpdateItemInput {
                                            TableName,
                                            Key,
                                            AttributeUpdates,
                                            Expected,
                                            ConditionalOperator,
                                            ReturnValues,
                                            ReturnConsumedCapacity,
                                            ReturnItemCollectionMetrics,
                                            UpdateExpression,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                        } => ReturnConsumedCapacity,
                                    }
                                }
                                pub fn ReturnItemCollectionMetrics(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ReturnItemCollectionMetrics>>>{
                                    match self {
                                        UpdateItemInput::UpdateItemInput {
                                            TableName,
                                            Key,
                                            AttributeUpdates,
                                            Expected,
                                            ConditionalOperator,
                                            ReturnValues,
                                            ReturnConsumedCapacity,
                                            ReturnItemCollectionMetrics,
                                            UpdateExpression,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                        } => ReturnItemCollectionMetrics,
                                    }
                                }
                                pub fn UpdateExpression(
                                    &self,
                                ) -> &::std::rc::Rc<
                                    crate::r#_Wrappers_Compile::Option<
                                        ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                    >,
                                > {
                                    match self {
                                        UpdateItemInput::UpdateItemInput {
                                            TableName,
                                            Key,
                                            AttributeUpdates,
                                            Expected,
                                            ConditionalOperator,
                                            ReturnValues,
                                            ReturnConsumedCapacity,
                                            ReturnItemCollectionMetrics,
                                            UpdateExpression,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                        } => UpdateExpression,
                                    }
                                }
                                pub fn ConditionExpression(
                                    &self,
                                ) -> &::std::rc::Rc<
                                    crate::r#_Wrappers_Compile::Option<
                                        ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                    >,
                                > {
                                    match self {
                                        UpdateItemInput::UpdateItemInput {
                                            TableName,
                                            Key,
                                            AttributeUpdates,
                                            Expected,
                                            ConditionalOperator,
                                            ReturnValues,
                                            ReturnConsumedCapacity,
                                            ReturnItemCollectionMetrics,
                                            UpdateExpression,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                        } => ConditionExpression,
                                    }
                                }
                                pub fn ExpressionAttributeNames(
                                    &self,
                                ) -> &::std::rc::Rc<
                                    crate::r#_Wrappers_Compile::Option<
                                        ::dafny_runtime::Map<
                                            ::dafny_runtime::Sequence<
                                                ::dafny_runtime::DafnyCharUTF16,
                                            >,
                                            ::dafny_runtime::Sequence<
                                                ::dafny_runtime::DafnyCharUTF16,
                                            >,
                                        >,
                                    >,
                                > {
                                    match self {
                                        UpdateItemInput::UpdateItemInput {
                                            TableName,
                                            Key,
                                            AttributeUpdates,
                                            Expected,
                                            ConditionalOperator,
                                            ReturnValues,
                                            ReturnConsumedCapacity,
                                            ReturnItemCollectionMetrics,
                                            UpdateExpression,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                        } => ExpressionAttributeNames,
                                    }
                                }
                                pub fn ExpressionAttributeValues(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>>>{
                                    match self {
                                        UpdateItemInput::UpdateItemInput {
                                            TableName,
                                            Key,
                                            AttributeUpdates,
                                            Expected,
                                            ConditionalOperator,
                                            ReturnValues,
                                            ReturnConsumedCapacity,
                                            ReturnItemCollectionMetrics,
                                            UpdateExpression,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                        } => ExpressionAttributeValues,
                                    }
                                }
                            }

                            impl Debug for UpdateItemInput {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for UpdateItemInput {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        UpdateItemInput::UpdateItemInput {
                                            TableName,
                                            Key,
                                            AttributeUpdates,
                                            Expected,
                                            ConditionalOperator,
                                            ReturnValues,
                                            ReturnConsumedCapacity,
                                            ReturnItemCollectionMetrics,
                                            UpdateExpression,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                        } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.UpdateItemInput.UpdateItemInput(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                TableName, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                Key, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                AttributeUpdates,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                Expected, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ConditionalOperator,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ReturnValues,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ReturnConsumedCapacity,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ReturnItemCollectionMetrics,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                UpdateExpression,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ConditionExpression,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ExpressionAttributeNames,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ExpressionAttributeValues,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for UpdateItemInput {}

                            impl Hash for UpdateItemInput {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        UpdateItemInput::UpdateItemInput {
                                            TableName,
                                            Key,
                                            AttributeUpdates,
                                            Expected,
                                            ConditionalOperator,
                                            ReturnValues,
                                            ReturnConsumedCapacity,
                                            ReturnItemCollectionMetrics,
                                            UpdateExpression,
                                            ConditionExpression,
                                            ExpressionAttributeNames,
                                            ExpressionAttributeValues,
                                        } => {
                                            ::std::hash::Hash::hash(TableName, _state);
                                            ::std::hash::Hash::hash(Key, _state);
                                            ::std::hash::Hash::hash(AttributeUpdates, _state);
                                            ::std::hash::Hash::hash(Expected, _state);
                                            ::std::hash::Hash::hash(ConditionalOperator, _state);
                                            ::std::hash::Hash::hash(ReturnValues, _state);
                                            ::std::hash::Hash::hash(ReturnConsumedCapacity, _state);
                                            ::std::hash::Hash::hash(
                                                ReturnItemCollectionMetrics,
                                                _state,
                                            );
                                            ::std::hash::Hash::hash(UpdateExpression, _state);
                                            ::std::hash::Hash::hash(ConditionExpression, _state);
                                            ::std::hash::Hash::hash(
                                                ExpressionAttributeNames,
                                                _state,
                                            );
                                            ::std::hash::Hash::hash(
                                                ExpressionAttributeValues,
                                                _state,
                                            )
                                        }
                                    }
                                }
                            }

                            impl Default for UpdateItemInput {
                                fn default() -> UpdateItemInput {
                                    UpdateItemInput::UpdateItemInput {
                                        TableName: ::std::default::Default::default(),
                                        Key: ::std::default::Default::default(),
                                        AttributeUpdates: ::std::default::Default::default(),
                                        Expected: ::std::default::Default::default(),
                                        ConditionalOperator: ::std::default::Default::default(),
                                        ReturnValues: ::std::default::Default::default(),
                                        ReturnConsumedCapacity: ::std::default::Default::default(),
                                        ReturnItemCollectionMetrics:
                                            ::std::default::Default::default(),
                                        UpdateExpression: ::std::default::Default::default(),
                                        ConditionExpression: ::std::default::Default::default(),
                                        ExpressionAttributeNames: ::std::default::Default::default(
                                        ),
                                        ExpressionAttributeValues: ::std::default::Default::default(
                                        ),
                                    }
                                }
                            }

                            impl AsRef<UpdateItemInput> for &UpdateItemInput {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            #[derive(PartialEq, Clone)]
                            pub enum UpdateItemOutput {
                                UpdateItemOutput {
                  Attributes: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>>>,
                  ConsumedCapacity: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ConsumedCapacity>>>,
                  ItemCollectionMetrics: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ItemCollectionMetrics>>>
                }
              }

                            impl UpdateItemOutput {
                                pub fn Attributes(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>>>{
                                    match self {
                                        UpdateItemOutput::UpdateItemOutput {
                                            Attributes,
                                            ConsumedCapacity,
                                            ItemCollectionMetrics,
                                        } => Attributes,
                                    }
                                }
                                pub fn ConsumedCapacity(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ConsumedCapacity>>>{
                                    match self {
                                        UpdateItemOutput::UpdateItemOutput {
                                            Attributes,
                                            ConsumedCapacity,
                                            ItemCollectionMetrics,
                                        } => ConsumedCapacity,
                                    }
                                }
                                pub fn ItemCollectionMetrics(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ItemCollectionMetrics>>>{
                                    match self {
                                        UpdateItemOutput::UpdateItemOutput {
                                            Attributes,
                                            ConsumedCapacity,
                                            ItemCollectionMetrics,
                                        } => ItemCollectionMetrics,
                                    }
                                }
                            }

                            impl Debug for UpdateItemOutput {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for UpdateItemOutput {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        UpdateItemOutput::UpdateItemOutput {
                                            Attributes,
                                            ConsumedCapacity,
                                            ItemCollectionMetrics,
                                        } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.UpdateItemOutput.UpdateItemOutput(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                Attributes, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ConsumedCapacity,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                ItemCollectionMetrics,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for UpdateItemOutput {}

                            impl Hash for UpdateItemOutput {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        UpdateItemOutput::UpdateItemOutput {
                                            Attributes,
                                            ConsumedCapacity,
                                            ItemCollectionMetrics,
                                        } => {
                                            ::std::hash::Hash::hash(Attributes, _state);
                                            ::std::hash::Hash::hash(ConsumedCapacity, _state);
                                            ::std::hash::Hash::hash(ItemCollectionMetrics, _state)
                                        }
                                    }
                                }
                            }

                            impl Default for UpdateItemOutput {
                                fn default() -> UpdateItemOutput {
                                    UpdateItemOutput::UpdateItemOutput {
                                        Attributes: ::std::default::Default::default(),
                                        ConsumedCapacity: ::std::default::Default::default(),
                                        ItemCollectionMetrics: ::std::default::Default::default(),
                                    }
                                }
                            }

                            impl AsRef<UpdateItemOutput> for &UpdateItemOutput {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            #[derive(PartialEq, Clone)]
                            pub enum Error {
                                ConditionalCheckFailedException {
                  message: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>
                },
                IdempotentParameterMismatchException {
                  Message: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>
                },
                InternalServerError {
                  message: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>
                },
                InvalidEndpointException {
                  Message: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>
                },
                ItemCollectionSizeLimitExceededException {
                  message: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>
                },
                LimitExceededException {
                  message: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>
                },
                ProvisionedThroughputExceededException {
                  message: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>
                },
                RequestLimitExceeded {
                  message: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>
                },
                ResourceInUseException {
                  message: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>
                },
                ResourceNotFoundException {
                  message: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>
                },
                TransactionCanceledException {
                  Message: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>,
                  CancellationReasons: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::CancellationReasonList>>
                },
                TransactionConflictException {
                  message: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>
                },
                TransactionInProgressException {
                  Message: ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>
                },
                Opaque {
                  obj: ::dafny_runtime::Object<dyn ::std::any::Any>
                }
              }

                            impl Error {
                                pub fn message(
                                    &self,
                                ) -> &::std::rc::Rc<
                                    crate::r#_Wrappers_Compile::Option<
                                        ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                    >,
                                > {
                                    match self {
                                        Error::ConditionalCheckFailedException { message } => {
                                            message
                                        }
                                        Error::IdempotentParameterMismatchException { Message } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        Error::InternalServerError { message } => message,
                                        Error::InvalidEndpointException { Message } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        Error::ItemCollectionSizeLimitExceededException {
                                            message,
                                        } => message,
                                        Error::LimitExceededException { message } => message,
                                        Error::ProvisionedThroughputExceededException {
                                            message,
                                        } => message,
                                        Error::RequestLimitExceeded { message } => message,
                                        Error::ResourceInUseException { message } => message,
                                        Error::ResourceNotFoundException { message } => message,
                                        Error::TransactionCanceledException {
                                            Message,
                                            CancellationReasons,
                                        } => panic!("field does not exist on this variant"),
                                        Error::TransactionConflictException { message } => message,
                                        Error::TransactionInProgressException { Message } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        Error::Opaque { obj } => {
                                            panic!("field does not exist on this variant")
                                        }
                                    }
                                }
                                pub fn Message(
                                    &self,
                                ) -> &::std::rc::Rc<
                                    crate::r#_Wrappers_Compile::Option<
                                        ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                                    >,
                                > {
                                    match self {
                                        Error::ConditionalCheckFailedException { message } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        Error::IdempotentParameterMismatchException { Message } => {
                                            Message
                                        }
                                        Error::InternalServerError { message } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        Error::InvalidEndpointException { Message } => Message,
                                        Error::ItemCollectionSizeLimitExceededException {
                                            message,
                                        } => panic!("field does not exist on this variant"),
                                        Error::LimitExceededException { message } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        Error::ProvisionedThroughputExceededException {
                                            message,
                                        } => panic!("field does not exist on this variant"),
                                        Error::RequestLimitExceeded { message } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        Error::ResourceInUseException { message } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        Error::ResourceNotFoundException { message } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        Error::TransactionCanceledException {
                                            Message,
                                            CancellationReasons,
                                        } => Message,
                                        Error::TransactionConflictException { message } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        Error::TransactionInProgressException { Message } => {
                                            Message
                                        }
                                        Error::Opaque { obj } => {
                                            panic!("field does not exist on this variant")
                                        }
                                    }
                                }
                                pub fn CancellationReasons(&self) -> &::std::rc::Rc<crate::r#_Wrappers_Compile::Option<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::CancellationReasonList>>{
                                    match self {
                                        Error::ConditionalCheckFailedException { message } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        Error::IdempotentParameterMismatchException { Message } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        Error::InternalServerError { message } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        Error::InvalidEndpointException { Message } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        Error::ItemCollectionSizeLimitExceededException {
                                            message,
                                        } => panic!("field does not exist on this variant"),
                                        Error::LimitExceededException { message } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        Error::ProvisionedThroughputExceededException {
                                            message,
                                        } => panic!("field does not exist on this variant"),
                                        Error::RequestLimitExceeded { message } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        Error::ResourceInUseException { message } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        Error::ResourceNotFoundException { message } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        Error::TransactionCanceledException {
                                            Message,
                                            CancellationReasons,
                                        } => CancellationReasons,
                                        Error::TransactionConflictException { message } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        Error::TransactionInProgressException { Message } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        Error::Opaque { obj } => {
                                            panic!("field does not exist on this variant")
                                        }
                                    }
                                }
                                pub fn obj(&self) -> &::dafny_runtime::Object<dyn::std::any::Any> {
                                    match self {
                                        Error::ConditionalCheckFailedException { message } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        Error::IdempotentParameterMismatchException { Message } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        Error::InternalServerError { message } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        Error::InvalidEndpointException { Message } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        Error::ItemCollectionSizeLimitExceededException {
                                            message,
                                        } => panic!("field does not exist on this variant"),
                                        Error::LimitExceededException { message } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        Error::ProvisionedThroughputExceededException {
                                            message,
                                        } => panic!("field does not exist on this variant"),
                                        Error::RequestLimitExceeded { message } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        Error::ResourceInUseException { message } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        Error::ResourceNotFoundException { message } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        Error::TransactionCanceledException {
                                            Message,
                                            CancellationReasons,
                                        } => panic!("field does not exist on this variant"),
                                        Error::TransactionConflictException { message } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        Error::TransactionInProgressException { Message } => {
                                            panic!("field does not exist on this variant")
                                        }
                                        Error::Opaque { obj } => obj,
                                    }
                                }
                            }

                            impl Debug for Error {
                                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                                    ::dafny_runtime::DafnyPrint::fmt_print(self, f, true)
                                }
                            }

                            impl DafnyPrint for Error {
                                fn fmt_print(
                                    &self,
                                    _formatter: &mut ::std::fmt::Formatter,
                                    _in_seq: bool,
                                ) -> std::fmt::Result {
                                    match self {
                                        Error::ConditionalCheckFailedException { message } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.Error.ConditionalCheckFailedException(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                message, _formatter, false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                        Error::IdempotentParameterMismatchException { Message } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.Error.IdempotentParameterMismatchException(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                Message, _formatter, false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                        Error::InternalServerError { message } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.Error.InternalServerError(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                message, _formatter, false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                        Error::InvalidEndpointException { Message } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.Error.InvalidEndpointException(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                Message, _formatter, false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                        Error::ItemCollectionSizeLimitExceededException {
                                            message,
                                        } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.Error.ItemCollectionSizeLimitExceededException(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                message, _formatter, false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                        Error::LimitExceededException { message } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.Error.LimitExceededException(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                message, _formatter, false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                        Error::ProvisionedThroughputExceededException {
                                            message,
                                        } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.Error.ProvisionedThroughputExceededException(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                message, _formatter, false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                        Error::RequestLimitExceeded { message } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.Error.RequestLimitExceeded(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                message, _formatter, false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                        Error::ResourceInUseException { message } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.Error.ResourceInUseException(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                message, _formatter, false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                        Error::ResourceNotFoundException { message } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.Error.ResourceNotFoundException(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                message, _formatter, false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                        Error::TransactionCanceledException {
                                            Message,
                                            CancellationReasons,
                                        } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.Error.TransactionCanceledException(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                Message, _formatter, false,
                                            )?;
                                            write!(_formatter, ", ")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                CancellationReasons,
                                                _formatter,
                                                false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                        Error::TransactionConflictException { message } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.Error.TransactionConflictException(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                message, _formatter, false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                        Error::TransactionInProgressException { Message } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.Error.TransactionInProgressException(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                Message, _formatter, false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                        Error::Opaque { obj } => {
                                            write!(_formatter, "software.amazon.cryptography.services.dynamodb.internaldafny.types.Error.Opaque(")?;
                                            ::dafny_runtime::DafnyPrint::fmt_print(
                                                obj, _formatter, false,
                                            )?;
                                            write!(_formatter, ")")?;
                                            Ok(())
                                        }
                                    }
                                }
                            }

                            impl Eq for Error {}

                            impl Hash for Error {
                                fn hash<_H: ::std::hash::Hasher>(&self, _state: &mut _H) {
                                    match self {
                                        Error::ConditionalCheckFailedException { message } => {
                                            ::std::hash::Hash::hash(message, _state)
                                        }
                                        Error::IdempotentParameterMismatchException { Message } => {
                                            ::std::hash::Hash::hash(Message, _state)
                                        }
                                        Error::InternalServerError { message } => {
                                            ::std::hash::Hash::hash(message, _state)
                                        }
                                        Error::InvalidEndpointException { Message } => {
                                            ::std::hash::Hash::hash(Message, _state)
                                        }
                                        Error::ItemCollectionSizeLimitExceededException {
                                            message,
                                        } => ::std::hash::Hash::hash(message, _state),
                                        Error::LimitExceededException { message } => {
                                            ::std::hash::Hash::hash(message, _state)
                                        }
                                        Error::ProvisionedThroughputExceededException {
                                            message,
                                        } => ::std::hash::Hash::hash(message, _state),
                                        Error::RequestLimitExceeded { message } => {
                                            ::std::hash::Hash::hash(message, _state)
                                        }
                                        Error::ResourceInUseException { message } => {
                                            ::std::hash::Hash::hash(message, _state)
                                        }
                                        Error::ResourceNotFoundException { message } => {
                                            ::std::hash::Hash::hash(message, _state)
                                        }
                                        Error::TransactionCanceledException {
                                            Message,
                                            CancellationReasons,
                                        } => {
                                            ::std::hash::Hash::hash(Message, _state);
                                            ::std::hash::Hash::hash(CancellationReasons, _state)
                                        }
                                        Error::TransactionConflictException { message } => {
                                            ::std::hash::Hash::hash(message, _state)
                                        }
                                        Error::TransactionInProgressException { Message } => {
                                            ::std::hash::Hash::hash(Message, _state)
                                        }
                                        Error::Opaque { obj } => {
                                            ::std::hash::Hash::hash(obj, _state)
                                        }
                                    }
                                }
                            }

                            impl Default for Error {
                                fn default() -> Error {
                                    Error::ConditionalCheckFailedException {
                                        message: ::std::default::Default::default(),
                                    }
                                }
                            }

                            impl AsRef<Error> for &Error {
                                fn as_ref(&self) -> Self {
                                    self
                                }
                            }

                            pub type OpaqueError = ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>;
                        }
                    }
                }
            }
        }
    }
}
pub mod r#_Com_Compile {
    pub mod r#_Amazonaws_Compile {}
}
pub mod r#_TestDDBv2_Compile {
    pub struct _default {}

    impl _default {
        pub fn BasicQueryTests() -> () {
            let mut attributeNameMap: ::dafny_runtime::Map<
                ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
                ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
            > = ::dafny_runtime::map![(::dafny_runtime::string_utf16_of("#bkid")) => (::dafny_runtime::string_utf16_of("branch-key-id")), (::dafny_runtime::string_utf16_of("#status")) => (::dafny_runtime::string_utf16_of("status"))];
            let mut attributeValueMap: ::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>> = ::dafny_runtime::map![(::dafny_runtime::string_utf16_of(":bkid")) => (::std::rc::Rc::new(crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue::S {
                S: ::dafny_runtime::string_utf16_of("aws-kms-h")
              })), (::dafny_runtime::string_utf16_of(":status")) => (::std::rc::Rc::new(crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue::S {
                S: ::dafny_runtime::string_utf16_of("ACTIVE")
              }))];
            let mut queryInput: ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::QueryInput> = ::std::rc::Rc::new(crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::QueryInput::QueryInput {
            TableName: crate::r#_TestDDBv2_Compile::_default::tableNameTest(),
            IndexName: ::std::rc::Rc::new(crate::r#_Wrappers_Compile::Option::<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>::Some {
                  value: crate::r#_TestDDBv2_Compile::_default::secIndex()
                }),
            Select: ::std::rc::Rc::new(crate::r#_Wrappers_Compile::Option::<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Select>>::None {}),
            AttributesToGet: ::std::rc::Rc::new(crate::r#_Wrappers_Compile::Option::<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeNameList>::None {}),
            Limit: ::std::rc::Rc::new(crate::r#_Wrappers_Compile::Option::<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::PositiveIntegerObject>::None {}),
            ConsistentRead: ::std::rc::Rc::new(crate::r#_Wrappers_Compile::Option::<bool>::None {}),
            KeyConditions: ::std::rc::Rc::new(crate::r#_Wrappers_Compile::Option::<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Condition>>>::None {}),
            QueryFilter: ::std::rc::Rc::new(crate::r#_Wrappers_Compile::Option::<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Condition>>>::None {}),
            ConditionalOperator: ::std::rc::Rc::new(crate::r#_Wrappers_Compile::Option::<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ConditionalOperator>>::None {}),
            ScanIndexForward: ::std::rc::Rc::new(crate::r#_Wrappers_Compile::Option::<bool>::None {}),
            ExclusiveStartKey: ::std::rc::Rc::new(crate::r#_Wrappers_Compile::Option::<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>>::None {}),
            ReturnConsumedCapacity: ::std::rc::Rc::new(crate::r#_Wrappers_Compile::Option::<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ReturnConsumedCapacity>>::None {}),
            ProjectionExpression: ::std::rc::Rc::new(crate::r#_Wrappers_Compile::Option::<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>::None {}),
            FilterExpression: ::std::rc::Rc::new(crate::r#_Wrappers_Compile::Option::<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>::None {}),
            KeyConditionExpression: ::std::rc::Rc::new(crate::r#_Wrappers_Compile::Option::<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>::Some {
                  value: ::dafny_runtime::string_utf16_of("#status = :status and #bkid = :bkid")
                }),
            ExpressionAttributeNames: ::std::rc::Rc::new(crate::r#_Wrappers_Compile::Option::<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>::Some {
                  value: attributeNameMap.clone()
                }),
            ExpressionAttributeValues: ::std::rc::Rc::new(crate::r#_Wrappers_Compile::Option::<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>>::Some {
                  value: attributeValueMap.clone()
                })
          });
            crate::r#_TestDDBv2_Compile::_default::BasicQueryTest(&queryInput);
            return ();
        }
        pub fn BasicGetTests() -> () {
            let mut Key2Get: ::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>> = ::dafny_runtime::map![(::dafny_runtime::string_utf16_of("branch-key-id")) => (::std::rc::Rc::new(crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue::S {
                S: ::dafny_runtime::string_utf16_of("aws-kms-h")
              })), (::dafny_runtime::string_utf16_of("version")) => (::std::rc::Rc::new(crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue::S {
                S: ::dafny_runtime::string_utf16_of("1")
              }))];
            let mut getInput: ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::GetItemInput> = ::std::rc::Rc::new(crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::GetItemInput::GetItemInput {
            TableName: crate::r#_TestDDBv2_Compile::_default::tableNameTest(),
            Key: Key2Get.clone(),
            AttributesToGet: ::std::rc::Rc::new(crate::r#_Wrappers_Compile::Option::<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeNameList>::None {}),
            ConsistentRead: ::std::rc::Rc::new(crate::r#_Wrappers_Compile::Option::<bool>::None {}),
            ReturnConsumedCapacity: ::std::rc::Rc::new(crate::r#_Wrappers_Compile::Option::<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ReturnConsumedCapacity>>::None {}),
            ProjectionExpression: ::std::rc::Rc::new(crate::r#_Wrappers_Compile::Option::<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>::None {}),
            ExpressionAttributeNames: ::std::rc::Rc::new(crate::r#_Wrappers_Compile::Option::<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>::None {})
          });
            crate::r#_TestDDBv2_Compile::_default::BasicGetTest(&getInput);
            return ();
        }
        pub fn BasicPutTests() -> () {
            let mut attributeValueMap: ::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>> = ::dafny_runtime::map![(::dafny_runtime::string_utf16_of("branch-key-id")) => (::std::rc::Rc::new(crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue::S {
                S: ::dafny_runtime::string_utf16_of("aws-kms-put-item")
              })), (::dafny_runtime::string_utf16_of("status")) => (::std::rc::Rc::new(crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue::S {
                S: ::dafny_runtime::string_utf16_of("ACTIVE")
              })), (::dafny_runtime::string_utf16_of("version")) => (::std::rc::Rc::new(crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue::S {
                S: ::dafny_runtime::string_utf16_of("version-1")
              }))];
            let mut putInput: ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::PutItemInput> = ::std::rc::Rc::new(crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::PutItemInput::PutItemInput {
            TableName: crate::r#_TestDDBv2_Compile::_default::tableNameTest(),
            Item: attributeValueMap.clone(),
            Expected: ::std::rc::Rc::new(crate::r#_Wrappers_Compile::Option::<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ExpectedAttributeValue>>>::None {}),
            ReturnValues: ::std::rc::Rc::new(crate::r#_Wrappers_Compile::Option::<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ReturnValue>>::None {}),
            ReturnConsumedCapacity: ::std::rc::Rc::new(crate::r#_Wrappers_Compile::Option::<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ReturnConsumedCapacity>>::None {}),
            ReturnItemCollectionMetrics: ::std::rc::Rc::new(crate::r#_Wrappers_Compile::Option::<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ReturnItemCollectionMetrics>>::None {}),
            ConditionalOperator: ::std::rc::Rc::new(crate::r#_Wrappers_Compile::Option::<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ConditionalOperator>>::None {}),
            ConditionExpression: ::std::rc::Rc::new(crate::r#_Wrappers_Compile::Option::<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>::None {}),
            ExpressionAttributeNames: ::std::rc::Rc::new(crate::r#_Wrappers_Compile::Option::<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>::None {}),
            ExpressionAttributeValues: ::std::rc::Rc::new(crate::r#_Wrappers_Compile::Option::<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>>::None {})
          });
            crate::r#_TestDDBv2_Compile::_default::BasicPutTest(&putInput);
            return ();
        }
        pub fn BatGetItemTests() -> () {
            let mut attributeNameBranchKey: ::dafny_runtime::Sequence<
                ::dafny_runtime::DafnyCharUTF16,
            > = ::dafny_runtime::string_utf16_of("branch-key-id");
            let mut attributeValueBranchKey: ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue> = ::std::rc::Rc::new(crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue::S {
            S: ::dafny_runtime::string_utf16_of("aws-kms-put-item")
          });
            let mut attributeNameVersion: ::dafny_runtime::Sequence<
                ::dafny_runtime::DafnyCharUTF16,
            > = ::dafny_runtime::string_utf16_of("version");
            let mut attributeValueVersion: ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue> = ::std::rc::Rc::new(crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue::S {
            S: ::dafny_runtime::string_utf16_of("version-1")
          });
            let mut key: ::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>> = ::dafny_runtime::map![(attributeNameBranchKey.clone()) => (attributeValueBranchKey.clone()), (attributeNameVersion.clone()) => (attributeValueVersion.clone())];
            let mut keys: crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::KeyList = ::dafny_runtime::seq![key.clone()];
            let mut keyAndAttributes: ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::KeysAndAttributes> = ::std::rc::Rc::new(crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::KeysAndAttributes::KeysAndAttributes {
            Keys: keys.clone(),
            AttributesToGet: ::std::rc::Rc::new(crate::r#_Wrappers_Compile::Option::<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeNameList>::None {}),
            ConsistentRead: ::std::rc::Rc::new(crate::r#_Wrappers_Compile::Option::<bool>::None {}),
            ProjectionExpression: ::std::rc::Rc::new(crate::r#_Wrappers_Compile::Option::<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>::None {}),
            ExpressionAttributeNames: ::std::rc::Rc::new(crate::r#_Wrappers_Compile::Option::<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>>::None {})
          });
            let mut batchGetRequestMap: crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::BatchGetRequestMap = ::dafny_runtime::map![(crate::r#_TestDDBv2_Compile::_default::tableNameTest()) => (keyAndAttributes.clone())];
            let mut batchGetInput: ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::BatchGetItemInput> = ::std::rc::Rc::new(crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::BatchGetItemInput::BatchGetItemInput {
            RequestItems: batchGetRequestMap.clone(),
            ReturnConsumedCapacity: ::std::rc::Rc::new(crate::r#_Wrappers_Compile::Option::<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::ReturnConsumedCapacity>>::None {})
          });
            crate::r#_TestDDBv2_Compile::_default::BatchGetItemTest(&batchGetInput);
            return ();
        }
        pub fn BasicQueryTest(
            input: &::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::QueryInput>,
        ) -> () {
            let mut valueOrError0 = ::dafny_runtime::MaybePlacebo::<::std::rc::Rc<crate::r#_Wrappers_Compile::Result<::dafny_runtime::Object<dyn crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::IDynamoDBClient>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>>>>::new();
            let mut _out0 = ::dafny_runtime::MaybePlacebo::<::std::rc::Rc<crate::r#_Wrappers_Compile::Result<::dafny_runtime::Object<dyn crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::IDynamoDBClient>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>>>>::new();
            _out0 = ::dafny_runtime::MaybePlacebo::from(crate::software::amazon::cryptography::services::dynamodb::internaldafny::_default::DynamoDBClient());
            valueOrError0 = ::dafny_runtime::MaybePlacebo::from(_out0.read());
            if !(!valueOrError0.read().IsFailure()) {
                panic!("Halt")
            };
            let mut client: ::dafny_runtime::Object<dyn crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::IDynamoDBClient> = valueOrError0.read().Extract();
            let mut ret = ::dafny_runtime::MaybePlacebo::<::std::rc::Rc<crate::r#_Wrappers_Compile::Result<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::QueryOutput>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>>>>::new();
            let mut _out1 = ::dafny_runtime::MaybePlacebo::<::std::rc::Rc<crate::r#_Wrappers_Compile::Result<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::QueryOutput>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>>>>::new();
            _out1 = ::dafny_runtime::MaybePlacebo::from(crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::IDynamoDBClient::Query(::dafny_runtime::md!(client.clone()), input));
            ret = ::dafny_runtime::MaybePlacebo::from(_out1.read());
            if !matches!(
                (&ret.read()).as_ref(),
                crate::r#_Wrappers_Compile::Result::Success { .. }
            ) {
                panic!("Halt")
            };
            let mut queryOutput: ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::QueryOutput> = ret.read().value().clone();
            if !matches!(
                queryOutput.Items().as_ref(),
                crate::r#_Wrappers_Compile::Option::Some { .. }
            ) {
                panic!("Halt")
            };
            let mut queryItem: ::dafny_runtime::Sequence<::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>> = queryOutput.Items().value().clone();
            if !(::dafny_runtime::int!(0) < queryItem.cardinality()) {
                panic!("Halt")
            };
            let mut item: ::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>> = queryItem.get(&::dafny_runtime::int!(0));
            let mut _e00: ::dafny_runtime::Set<
                ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
            > = item.keys();
            let mut _e10: ::dafny_runtime::Set<
                ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
            > = ::dafny_runtime::set! {::dafny_runtime::string_utf16_of("branch-key-id"), ::dafny_runtime::string_utf16_of("version"), ::dafny_runtime::string_utf16_of("create-time"), ::dafny_runtime::string_utf16_of("enc"), ::dafny_runtime::string_utf16_of("hierarchy-version"), ::dafny_runtime::string_utf16_of("status")};
            if !(_e00.clone() == _e10.clone()) {
                print!(
                    "{}",
                    ::dafny_runtime::DafnyPrintWrapper(&::dafny_runtime::string_utf16_of(
                        "Left:\n"
                    ))
                );
                print!("{}", ::dafny_runtime::DafnyPrintWrapper(&_e00));
                print!(
                    "{}",
                    ::dafny_runtime::DafnyPrintWrapper(&::dafny_runtime::string_utf16_of(
                        "Right:\n"
                    ))
                );
                print!("{}", ::dafny_runtime::DafnyPrintWrapper(&_e10));
                panic!("Halt")
            };
            let mut _e01: ::dafny_runtime::DafnyInt = item.keys().cardinality();
            let mut _e11: ::dafny_runtime::DafnyInt = item.values().cardinality();
            if !(_e01.clone() == _e11.clone()) {
                print!(
                    "{}",
                    ::dafny_runtime::DafnyPrintWrapper(&::dafny_runtime::string_utf16_of(
                        "Left:\n"
                    ))
                );
                print!("{}", ::dafny_runtime::DafnyPrintWrapper(&_e01));
                print!(
                    "{}",
                    ::dafny_runtime::DafnyPrintWrapper(&::dafny_runtime::string_utf16_of(
                        "Right:\n"
                    ))
                );
                print!("{}", ::dafny_runtime::DafnyPrintWrapper(&_e11));
                panic!("Halt")
            };
            return ();
        }
        pub fn BasicGetTest(
            input: &::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::GetItemInput>,
        ) -> () {
            let mut valueOrError0 = ::dafny_runtime::MaybePlacebo::<::std::rc::Rc<crate::r#_Wrappers_Compile::Result<::dafny_runtime::Object<dyn crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::IDynamoDBClient>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>>>>::new();
            let mut _out2 = ::dafny_runtime::MaybePlacebo::<::std::rc::Rc<crate::r#_Wrappers_Compile::Result<::dafny_runtime::Object<dyn crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::IDynamoDBClient>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>>>>::new();
            _out2 = ::dafny_runtime::MaybePlacebo::from(crate::software::amazon::cryptography::services::dynamodb::internaldafny::_default::DynamoDBClient());
            valueOrError0 = ::dafny_runtime::MaybePlacebo::from(_out2.read());
            if !(!valueOrError0.read().IsFailure()) {
                panic!("Halt")
            };
            let mut client: ::dafny_runtime::Object<dyn crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::IDynamoDBClient> = valueOrError0.read().Extract();
            let mut ret = ::dafny_runtime::MaybePlacebo::<::std::rc::Rc<crate::r#_Wrappers_Compile::Result<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::GetItemOutput>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>>>>::new();
            let mut _out3 = ::dafny_runtime::MaybePlacebo::<::std::rc::Rc<crate::r#_Wrappers_Compile::Result<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::GetItemOutput>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>>>>::new();
            _out3 = ::dafny_runtime::MaybePlacebo::from(crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::IDynamoDBClient::GetItem(::dafny_runtime::md!(client.clone()), input));
            ret = ::dafny_runtime::MaybePlacebo::from(_out3.read());
            if !matches!(
                (&ret.read()).as_ref(),
                crate::r#_Wrappers_Compile::Result::Success { .. }
            ) {
                panic!("Halt")
            };
            let mut itemOutput: ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::GetItemOutput> = ret.read().value().clone();
            if !matches!(
                itemOutput.Item().as_ref(),
                crate::r#_Wrappers_Compile::Option::Some { .. }
            ) {
                panic!("Halt")
            };
            let mut item: ::dafny_runtime::Map<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>> = itemOutput.Item().value().clone();
            let mut _e02: ::dafny_runtime::Set<
                ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
            > = item.keys();
            let mut _e12: ::dafny_runtime::Set<
                ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
            > = ::dafny_runtime::set! {::dafny_runtime::string_utf16_of("branch-key-id"), ::dafny_runtime::string_utf16_of("version"), ::dafny_runtime::string_utf16_of("create-time"), ::dafny_runtime::string_utf16_of("enc"), ::dafny_runtime::string_utf16_of("hierarchy-version"), ::dafny_runtime::string_utf16_of("status")};
            if !(_e02.clone() == _e12.clone()) {
                print!(
                    "{}",
                    ::dafny_runtime::DafnyPrintWrapper(&::dafny_runtime::string_utf16_of(
                        "Left:\n"
                    ))
                );
                print!("{}", ::dafny_runtime::DafnyPrintWrapper(&_e02));
                print!(
                    "{}",
                    ::dafny_runtime::DafnyPrintWrapper(&::dafny_runtime::string_utf16_of(
                        "Right:\n"
                    ))
                );
                print!("{}", ::dafny_runtime::DafnyPrintWrapper(&_e12));
                panic!("Halt")
            };
            let mut _e03: ::dafny_runtime::DafnyInt = item.keys().cardinality();
            let mut _e13: ::dafny_runtime::DafnyInt = item.values().cardinality();
            if !(_e03.clone() == _e13.clone()) {
                print!(
                    "{}",
                    ::dafny_runtime::DafnyPrintWrapper(&::dafny_runtime::string_utf16_of(
                        "Left:\n"
                    ))
                );
                print!("{}", ::dafny_runtime::DafnyPrintWrapper(&_e03));
                print!(
                    "{}",
                    ::dafny_runtime::DafnyPrintWrapper(&::dafny_runtime::string_utf16_of(
                        "Right:\n"
                    ))
                );
                print!("{}", ::dafny_runtime::DafnyPrintWrapper(&_e13));
                panic!("Halt")
            };
            return ();
        }
        pub fn BasicPutTest(
            input: &::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::PutItemInput>,
        ) -> () {
            let mut valueOrError0 = ::dafny_runtime::MaybePlacebo::<::std::rc::Rc<crate::r#_Wrappers_Compile::Result<::dafny_runtime::Object<dyn crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::IDynamoDBClient>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>>>>::new();
            let mut _out4 = ::dafny_runtime::MaybePlacebo::<::std::rc::Rc<crate::r#_Wrappers_Compile::Result<::dafny_runtime::Object<dyn crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::IDynamoDBClient>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>>>>::new();
            _out4 = ::dafny_runtime::MaybePlacebo::from(crate::software::amazon::cryptography::services::dynamodb::internaldafny::_default::DynamoDBClient());
            valueOrError0 = ::dafny_runtime::MaybePlacebo::from(_out4.read());
            if !(!valueOrError0.read().IsFailure()) {
                panic!("Halt")
            };
            let mut client: ::dafny_runtime::Object<dyn crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::IDynamoDBClient> = valueOrError0.read().Extract();
            let mut ret = ::dafny_runtime::MaybePlacebo::<::std::rc::Rc<crate::r#_Wrappers_Compile::Result<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::PutItemOutput>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>>>>::new();
            let mut _out5 = ::dafny_runtime::MaybePlacebo::<::std::rc::Rc<crate::r#_Wrappers_Compile::Result<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::PutItemOutput>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>>>>::new();
            _out5 = ::dafny_runtime::MaybePlacebo::from(crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::IDynamoDBClient::PutItem(::dafny_runtime::md!(client.clone()), input));
            ret = ::dafny_runtime::MaybePlacebo::from(_out5.read());
            if !matches!(
                (&ret.read()).as_ref(),
                crate::r#_Wrappers_Compile::Result::Success { .. }
            ) {
                panic!("Halt")
            };
            return ();
        }
        pub fn BatchGetItemTest(
            input: &::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::BatchGetItemInput>,
        ) -> () {
            let mut valueOrError0 = ::dafny_runtime::MaybePlacebo::<::std::rc::Rc<crate::r#_Wrappers_Compile::Result<::dafny_runtime::Object<dyn crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::IDynamoDBClient>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>>>>::new();
            let mut _out6 = ::dafny_runtime::MaybePlacebo::<::std::rc::Rc<crate::r#_Wrappers_Compile::Result<::dafny_runtime::Object<dyn crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::IDynamoDBClient>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>>>>::new();
            _out6 = ::dafny_runtime::MaybePlacebo::from(crate::software::amazon::cryptography::services::dynamodb::internaldafny::_default::DynamoDBClient());
            valueOrError0 = ::dafny_runtime::MaybePlacebo::from(_out6.read());
            if !(!valueOrError0.read().IsFailure()) {
                panic!("Halt")
            };
            let mut client: ::dafny_runtime::Object<dyn crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::IDynamoDBClient> = valueOrError0.read().Extract();
            let mut ret = ::dafny_runtime::MaybePlacebo::<::std::rc::Rc<crate::r#_Wrappers_Compile::Result<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::BatchGetItemOutput>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>>>>::new();
            let mut _out7 = ::dafny_runtime::MaybePlacebo::<::std::rc::Rc<crate::r#_Wrappers_Compile::Result<::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::BatchGetItemOutput>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>>>>::new();
            _out7 = ::dafny_runtime::MaybePlacebo::from(crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::IDynamoDBClient::BatchGetItem(::dafny_runtime::md!(client.clone()), input));
            ret = ::dafny_runtime::MaybePlacebo::from(_out7.read());
            if matches!(
                (&ret.read()).as_ref(),
                crate::r#_Wrappers_Compile::Result::Failure { .. }
            ) {
                print!(
                    "{}",
                    ::dafny_runtime::DafnyPrintWrapper(&::dafny_runtime::string_utf16_of(
                        "\n\t BatchGetItemTest Failed"
                    ))
                );
                print!(
                    "{}",
                    ::dafny_runtime::DafnyPrintWrapper(&::dafny_runtime::string_utf16_of("\n\t"))
                );
                print!("{}", ::dafny_runtime::DafnyPrintWrapper(&ret.read()));
                print!(
                    "{}",
                    ::dafny_runtime::DafnyPrintWrapper(&::dafny_runtime::string_utf16_of("\n"))
                )
            };
            if !matches!(
                (&ret.read()).as_ref(),
                crate::r#_Wrappers_Compile::Result::Success { .. }
            ) {
                panic!("Halt")
            };
            return ();
        }
        pub fn tableNameTest() -> ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16> {
            ::dafny_runtime::string_utf16_of("TestTable")
        }
        pub fn secIndex() -> ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16> {
            ::dafny_runtime::string_utf16_of("Active-Keys")
        }
    }

    #[test]
    pub fn BasicQueryTests() {
        _default::BasicQueryTests()
    }

    #[test]
    pub fn BasicGetTests() {
        _default::BasicGetTests()
    }

    #[test]
    pub fn BasicPutTests() {
        _default::BasicPutTests()
    }

    #[test]
    pub fn BatGetItemTests() {
        _default::BatGetItemTests()
    }
}
pub mod r#_StandardLibraryInterop_Compile {
    pub use dafny_runtime::UpcastObject;
    pub use std::any::Any;

    pub struct WrappersInterop {}

    impl WrappersInterop {
        pub fn _allocate_object() -> ::dafny_runtime::Object<Self> {
            ::dafny_runtime::allocate_object::<Self>()
        }
        pub fn CreateStringSome(
            s: &::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
        ) -> ::std::rc::Rc<
            crate::r#_Wrappers_Compile::Option<
                ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
            >,
        > {
            ::std::rc::Rc::new(crate::r#_Wrappers_Compile::Option::<
                ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
            >::Some {
                value: s.clone(),
            })
        }
        pub fn CreateStringNone() -> ::std::rc::Rc<
            crate::r#_Wrappers_Compile::Option<
                ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
            >,
        > {
            ::std::rc::Rc::new(crate::r#_Wrappers_Compile::Option::<
                ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
            >::None {})
        }
        pub fn CreateBooleanSome(
            b: bool,
        ) -> ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<bool>> {
            ::std::rc::Rc::new(crate::r#_Wrappers_Compile::Option::<bool>::Some { value: b })
        }
        pub fn CreateBooleanNone() -> ::std::rc::Rc<crate::r#_Wrappers_Compile::Option<bool>> {
            ::std::rc::Rc::new(crate::r#_Wrappers_Compile::Option::<bool>::None {})
        }
    }

    impl UpcastObject<dyn Any> for WrappersInterop {
        ::dafny_runtime::UpcastObjectFn!(dyn::std::any::Any);
    }
}
pub mod _module {
    pub struct _default {}

    impl _default {
        pub fn _Test__Main_() -> () {
            let mut success: bool = true;
            print!(
                "{}",
                ::dafny_runtime::DafnyPrintWrapper(&::dafny_runtime::string_utf16_of(
                    r#"TestDDBv2.BasicQueryTests: "#
                ))
            );
            crate::r#_TestDDBv2_Compile::_default::BasicQueryTests();
            print!(
                "{}",
                ::dafny_runtime::DafnyPrintWrapper(&::dafny_runtime::string_utf16_of(
                    r#"PASSED
"#
                ))
            );
            print!(
                "{}",
                ::dafny_runtime::DafnyPrintWrapper(&::dafny_runtime::string_utf16_of(
                    r#"TestDDBv2.BasicGetTests: "#
                ))
            );
            crate::r#_TestDDBv2_Compile::_default::BasicGetTests();
            print!(
                "{}",
                ::dafny_runtime::DafnyPrintWrapper(&::dafny_runtime::string_utf16_of(
                    r#"PASSED
"#
                ))
            );
            print!(
                "{}",
                ::dafny_runtime::DafnyPrintWrapper(&::dafny_runtime::string_utf16_of(
                    r#"TestDDBv2.BasicPutTests: "#
                ))
            );
            crate::r#_TestDDBv2_Compile::_default::BasicPutTests();
            print!(
                "{}",
                ::dafny_runtime::DafnyPrintWrapper(&::dafny_runtime::string_utf16_of(
                    r#"PASSED
"#
                ))
            );
            print!(
                "{}",
                ::dafny_runtime::DafnyPrintWrapper(&::dafny_runtime::string_utf16_of(
                    r#"TestDDBv2.BatGetItemTests: "#
                ))
            );
            crate::r#_TestDDBv2_Compile::_default::BatGetItemTests();
            print!(
                "{}",
                ::dafny_runtime::DafnyPrintWrapper(&::dafny_runtime::string_utf16_of(
                    r#"PASSED
"#
                ))
            );
            if !success {
                panic!("Halt")
            };
            return ();
        }
    }
}
fn main() {
    _module::_default::_Test__Main_();
}
