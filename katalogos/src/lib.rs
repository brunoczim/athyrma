pub mod list;
pub mod colist;
pub mod function;
pub mod combinator;
pub mod by_ref;

#[macro_export]
macro_rules! hlist {
    [] => { $crate::list::Nil };
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
    [] => { $crate::list::Nil };
    [$elem:expr $(, $elems:expr)*] => {
        $crate::list::Cons<$elem, $crate::HList![$($elems),*]>,
    };
    [$($elems:expr,)*] => {
        $crate::HList![$($elems),*]
    };
}

#[macro_export]
macro_rules! hcolist {
    [$($t:ident ::)* h($elem:expr)] => {{
        let expr = $crate::colist::Cocons::Head($elem);
        $(let expr = $crate::colist::Cocons::Tail(expr);)*
        expr
    }};
}

#[macro_export]
macro_rules! HColist {
    [] => { $crate::colist::Conil };
    [$elem:expr $(, $elems:expr)*] => {
        $crate::colist::Cocons<$elem, $crate::HColist![$($elems),*]>,
    };
    [$($elems:expr,)*] => {
        $crate::HColist![$($elems),*]
    };
}
