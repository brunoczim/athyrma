pub mod list;
pub mod colist;
pub mod function;
pub mod combinator;
pub mod by_ref;

#[macro_export]
macro_rules! hlist {
    [] => { $crate::list::Nil::new() };
    [$elem:expr $(, $elems:expr)*] => {
        $crate::list::Cons {
            head: $elem,
            tail: $crate::hlist![$($elems),*],
        }
    };
    [$($elems:expr,)*] => {
        $crate::hlist![$($elems),*]
    };
}

#[macro_export]
macro_rules! HList {
    [(): $m:ty] => { $crate::list::Nil<$m> };
    [($elem:ty $(, $elems:ty)*): $m:ty] => {
        $crate::list::Cons<$elem, $crate::HList![($($elems),*): $m]>
    };
    [($($elems:ty,)*): $m:ty] => {
        $crate::HList![($($elems),*): $m]
    };

    [] => { $crate::list::Nil };
    [$elem:ty $(, $elems:ty)*] => {
        $crate::list::Cons<$elem, $crate::HList![$($elems),*]>
    };
    [$($elems:ty,)*] => {
        $crate::HList![$($elems),*]
    };
}

#[macro_export]
macro_rules! hcolist {
    [h($elem:expr)] => {
        $crate::colist::Cocons::Head($elem)
    };
    [t::$($tok:tt)+] => {
        $crate::colist::Cocons::Tail($crate::hcolist![$($tok)*])
    };
}

#[macro_export]
macro_rules! HColist {
    [(): $m:ty] => { $crate::colist::Conil<$m> };
    [($elem:ty $(, $elems:ty)*): $m:ty] => {
        $crate::colist::Cocons<$elem, $crate::HColist![($($elems),*): $m]>
    };
    [($($elems:ty,)*): $m:ty] => {
        $crate::HColist![($($elems),*): $m]
    };

    [] => { $crate::colist::Conil };
    [$elem:ty $(, $elems:ty)*] => {
        $crate::colist::Cocons<$elem, $crate::HColist![$($elems),*]>
    };
    [$($elems:ty,)*] => {
        $crate::HColist![$($elems),*]
    };
}

#[cfg(test)]
mod test {
    #[allow(dead_code)]
    const BOOL_META_LIST: HList![(&str, i32): bool] = hlist!["a", 2];

    #[allow(dead_code)]
    const BOOL_UNIT_META_LIST: HList![(): bool] = hlist![];

    #[allow(dead_code)]
    const UNIT_META_LIST: HList![&str, i32] = hlist!["a", 2];

    #[allow(dead_code)]
    const EMPTY_UNIT_META_LIST: HList![] = hlist![];

    #[allow(dead_code)]
    const BOOL_META_COLIST: HColist![
        (&str, i32, [f64; 3], u8, i16, u128, (i64, u64)): bool
    ] = hcolist![t::t::h([0.0; 3])];

    #[allow(dead_code)]
    type EmptyBoolMetaCoList = HColist![(): bool];

    #[allow(dead_code)]
    const UNIT_META_COLIST: HColist![
        &str,
        i32,
        [f64; 3],
        u8,
        i16,
        u128,
        (i64, u64)
    ] = hcolist![t::t::h([0.0; 3])];

    #[allow(dead_code)]
    type EmptyUnitMetaCoList = HColist![];
}
