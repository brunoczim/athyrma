use crate::component::Component;

pub struct List<L>(pub L)
where
    for<'a> &'a L: IntoIterator,
    for<'a> <&'a L as IntoIterator>::Item: Component;
