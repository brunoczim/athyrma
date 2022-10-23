use super::{
    Component,
    ComponentKind,
    Context,
    HtmlRendering,
    MdRendering,
    Render,
};
use std::fmt;

pub mod text;

#[derive(Debug)]
pub struct InlineComponent {
    _priv: (),
}

impl InlineComponent {
    pub(crate) fn new() -> Self {
        Self { _priv: () }
    }
}

impl ComponentKind for InlineComponent {}

fn html_escape(ch: char) -> Option<&'static str> {
    match ch {
        '&' => Some("&amp;"),
        '<' => Some("&lt;"),
        '>' => Some("&gt;"),
        '"' => Some("&quot;"),
        '\'' => Some("&#39;"),
        '\\' => Some("&#92;"),
        _ => None,
    }
}

fn md_escape(ch: char) -> Option<&'static str> {
    match ch {
        '*' => Some("\\*"),
        '-' => Some("\\-"),
        '`' => Some("\\`"),
        '_' => Some("\\_"),
        '(' => Some("\\("),
        ')' => Some("\\)"),
        '[' => Some("\\["),
        ']' => Some("\\]"),
        _ => html_escape(ch),
    }
}

impl Component for str {
    type Kind = InlineComponent;
}

impl Render<HtmlRendering> for str {
    fn render(
        &self,
        fmtr: &mut fmt::Formatter,
        _ctx: &Context<HtmlRendering, Self::Kind>,
    ) -> fmt::Result {
        let mut start = 0;
        let iter = self
            .char_indices()
            .filter_map(|(i, ch)| html_escape(ch).map(|s| (i, s)));

        for (end, escape) in iter {
            fmtr.write_str(&self[start .. end])?;
            fmtr.write_str(escape)?;
            start = end + 1;
        }

        fmtr.write_str(&self[start ..])?;
        Ok(())
    }
}

impl Render<MdRendering> for str {
    fn render(
        &self,
        fmtr: &mut fmt::Formatter,
        _ctx: &Context<MdRendering, Self::Kind>,
    ) -> fmt::Result {
        let mut start = 0;
        let iter = self
            .char_indices()
            .filter_map(|(i, ch)| md_escape(ch).map(|s| (i, s)));

        for (end, escape) in iter {
            fmtr.write_str(&self[start .. end])?;
            fmtr.write_str(escape)?;
            start = end + 1;
        }

        fmtr.write_str(&self[start ..])?;
        Ok(())
    }
}

impl_text_as_display! { str }

impl Component for String {
    type Kind = InlineComponent;
}

impl Render<HtmlRendering> for String {
    fn render(
        &self,
        fmtr: &mut fmt::Formatter,
        ctx: &Context<HtmlRendering, Self::Kind>,
    ) -> fmt::Result {
        (**self).render(fmtr, ctx)
    }
}

impl Render<MdRendering> for String {
    fn render(
        &self,
        fmtr: &mut fmt::Formatter,
        ctx: &Context<MdRendering, Self::Kind>,
    ) -> fmt::Result {
        (**self).render(fmtr, ctx)
    }
}

impl_text_as_display! { String }
