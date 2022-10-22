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
    [(): $m:ty] => { $crate::list::Nil<M> };
    [($elem:ty $(, $elems:ty)*): $m:ty] => {
        $crate::list::Cons<$elem, $crate::HList![($($elems),*): $m]>,
    };
    [($($elems:ty,)*): $m:ty] => {
        $crate::HList![($($elems),*): $m]
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
    [(): $m:ty] => { $crate::colist::Conil<M> };
    [($elem:ty $(, $elems:ty)*): $m:ty] => {
        $crate::colist::Cocons<$elem, $crate::HColist![($($elems),*): $m]>,
    };
    [($($elems:ty,)*): $m:ty] => {
        $crate::HColist![($($elems),*): $m]
    };
}
