use crate::component::{self, Component, HtmlRendering, Render};
use katalogos::{List, Uncons};
use std::fmt;

pub struct UnorderedList<L>(pub L)
where
    L: List;

impl<L> Component for UnorderedList<L> {}

struct UnorderedListElements<L>(L);

impl<L> Component for UnorderedListElements<L> {}

impl<L> Render<HtmlRendering> for UnorderedListElements<L>
where
    L: List + Uncons,
    L::Head: Component,
{
    fn render(
        &self,
        fmtr: &mut fmt::Formatter,
        ctx: &component::Context<HtmlRendering, Self::Kind>,
    ) -> fmt::Result {
        match self.0.uncons() {
            Some((head, tail)) => {
                write!(fmtr, "<li>{}</li>",)?;
                UnorderedListElements(tail).render(fmtr, ctx)
            },
            None => Ok(()),
        }
    }
}
