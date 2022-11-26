pub mod coproduct;
pub mod function;
pub mod combinator;
pub mod by_ref;

#[macro_export]
macro_rules! hiter {
    [] => {
        ::std::iter::empty<$crate::coproduct::Conil>()
    };
    [$elem:expr $(, $elems:expr)*] => {
        ::std::iter::once($crate::coproduct::Cocons::Head($elem))
            .chain(
                $crate::hiter![$($elems),*]
                .map($ccrate::coproduct::Cocons::Tail)
            )
    };
    [$($elems:expr,)*] => {
        $crate::hiter![$($elems),*]
    };
}

#[macro_export]
macro_rules! hvec {
    [$($elems:expr),*] => {
        $crate::hiter![$($elems),*].collect::<Vec<_>>()
    };
    [$($elems:expr,)*] => {
        $crate::hvec![$($elems),*]
    };
}

#[macro_export]
macro_rules! harray {
    [$($elems:expr),*] => {
        $crate::harray![
            @done_in = []
            @done_out = []
            @buf = []
            @todo = [$($elems),*]
        ]
    };

    [$($elems:expr,)*] => {
        $crate::harray![$($elems),*]
    };

    [
        @done_in = []
        @done_out = [$($done:expr),*]
        @buf = []
        @todo = [$elem:expr $(,$elems:expr)*]
    ] => {
        $crate::harray![
            @done_in = [$($done),*]
            @done_out = []
            @buf = [$crate::coproduct::Cocons::Head($elem)]
            @todo = [$($elems),*]
        ]
    };


    [
        @done_in = []
        @done_out = [$($done:expr),*]
        @buf = []
        @todo = []
    ] => {
        [$($done),*]
    };

    [
        @done_in = []
        @done_out = [$($done_out:expr),*]
        @buf = [$buf:expr]
        @todo = [$($elems:expr),*]
    ] => {
        $crate::harray![
            @done_in = []
            @done_out = [$($done_out,)* $buf]
            @buf = []
            @todo = [$($elems),*]
        ]
    };

    [
        @done_in = [$done:expr $(, $done_in:expr)*]
        @done_out = [$($done_out:expr),*]
        @buf = [$buf:expr]
        @todo = [$($elems:expr),*]
    ] => {
        $crate::harray![
            @done_in = [$($done_in),*]
            @done_out = [$($done_out,)* $done]
            @buf = [$crate::coproduct::Cocons::Tail($buf)]
            @todo = [$($elems),*]
        ]
    };
}

#[macro_export]
macro_rules! HArray {
    [($($tys:ty),*): $m:ty] => {
        $crate::HArray![
            @revert = [$($tys),*]
            @done = []
            @meta = [$m]
        ]
    };

    [$($tys:ty),*] => {
        $crate::HArray![
            @revert = [$($tys),*]
            @done = []
            @meta = []
        ]
    };

    [
        @revert = [$input:ty $(,$inputs:ty)*]
        @done = [$($tys:ty),*]
        @meta = [$($tt:tt)*]
    ] => {
        $crate::HArray![
            @revert = [$($inputs),*]
            @done = [$input $(,$tys)*]
            @meta = [$($tt)*]
        ]
    };

    [
        @revert = []
        @done = [$($tys:ty),*]
        @meta = []
    ] => {
        $crate::HArray![
            @count = [0]
            @buf = [$($tys),*]
            @done = [$crate::coproduct::Conil]
        ]
    };

    [
        @revert = []
        @done = [$($tys:ty),*]
        @meta = [$m:ty]
    ] => {
        $crate::HArray![
            @count = [0]
            @buf = [$($tys),*]
            @done = [$crate::coproduct::Conil<$m>]
        ]
    };

    [
        @count = [$n:expr] @buf = [$ty:ty $(,$tys:ty)*]
        @done = [$done:ty]
    ] => {
        $crate::HArray![
            @count = [$n + 1]
            @buf = [$($tys),*]
            @done = [$crate::coproduct::Cocons<$ty, $done>]
        ]
    };

    [@count = [$n:expr] @buf = []  @done = [$ty:ty]] => {
        [$ty; $n]
    };
}

#[macro_export]
macro_rules! Coproduct {
    [(): $m:ty] => { $crate::coproduct::Conil<$m> };
    [($elem:ty $(, $elems:ty)*): $m:ty] => {
        $crate::coproduct::Cocons<$elem, $crate::Coproduct![($($elems),*): $m]>
    };
    [($($elems:ty,)*): $m:ty] => {
        $crate::Coproduct![($($elems),*): $m]
    };

    [] => { $crate::coproduct::Conil };
    [$elem:ty $(, $elems:ty)*] => {
        $crate::coproduct::Cocons<$elem, $crate::Coproduct![$($elems),*]>
    };
    [$($elems:ty,)*] => {
        $crate::Coproduct![$($elems),*]
    };
}

#[cfg(test)]
mod test {
    #[allow(dead_code)]
    const BOOL_META_LIST: HArray![&str, i32] = harray!["a", 2];
}
