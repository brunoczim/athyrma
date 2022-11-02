use crate::render_format::{Html, Markdown, Text};

use super::{Component, ComponentKind, Context, Render, Renderer};
use std::fmt::{self, Write};

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

impl Render<Html> for str {
    fn render(
        &self,
        renderer: &mut Renderer<Html>,
        _ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        let mut start = 0;
        let iter = self
            .char_indices()
            .filter_map(|(i, ch)| html_escape(ch).map(|s| (i, s)));

        for (end, escape) in iter {
            renderer.write_str(&self[start .. end])?;
            renderer.write_str(escape)?;
            start = end + 1;
        }

        renderer.write_str(&self[start ..])?;
        Ok(())
    }
}

impl<'sess> Render<Markdown<'sess>> for str {
    fn render(
        &self,
        renderer: &mut Renderer<Markdown<'sess>>,
        _ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        let mut start = 0;
        let iter = self
            .char_indices()
            .filter_map(|(i, ch)| md_escape(ch).map(|s| (i, s)));

        for (end, escape) in iter {
            renderer.write_str(&self[start .. end])?;
            renderer.write_str(escape)?;
            start = end + 1;
        }

        renderer.write_str(&self[start ..])?;
        Ok(())
    }
}

impl<'sess> Render<Text<'sess>> for str {
    fn render(
        &self,
        renderer: &mut Renderer<Text<'sess>>,
        _ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.write_str(self)
    }
}

impl Component for String {
    type Kind = InlineComponent;
}

impl Render<Html> for String {
    fn render(
        &self,
        renderer: &mut Renderer<Html>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        (**self).render(renderer, ctx)
    }
}

impl<'sess> Render<Markdown<'sess>> for String {
    fn render(
        &self,
        renderer: &mut Renderer<Markdown<'sess>>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        (**self).render(renderer, ctx)
    }
}

impl<'sess> Render<Text<'sess>> for String {
    fn render(
        &self,
        renderer: &mut Renderer<Text<'sess>>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        (**self).render(renderer, ctx)
    }
}
