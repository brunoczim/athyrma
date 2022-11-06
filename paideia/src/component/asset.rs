use std::fmt::{self, Write};

use crate::{
    location::Location,
    render::{Context, Html, Render, Renderer},
};

use super::{Component, ComponentKind, InlineComponent};

#[derive(Debug)]
pub struct AssetComponent {
    _priv: (),
}

impl AssetComponent {
    pub(crate) fn new() -> Self {
        Self { _priv: () }
    }
}

impl ComponentKind for AssetComponent {}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Stylesheet {
    pub location: Location,
}

impl Component for Stylesheet {
    type Kind = AssetComponent;
}

impl Render<Html> for Stylesheet {
    fn render(
        &self,
        renderer: &mut Renderer<Html>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.write_str("<link rel=\"stylesheet\" href=\"")?;
        self.location
            .render(renderer, ctx.with_kind(&InlineComponent::new()))?;
        renderer.write_str("\">")?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Script {
    pub location: Location,
}

impl Component for Script {
    type Kind = AssetComponent;
}

impl Render<Html> for Script {
    fn render(
        &self,
        renderer: &mut Renderer<Html>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.write_str("<script type=\"application/javascript\" src=\"")?;
        self.location
            .render(renderer, ctx.with_kind(&InlineComponent::new()))?;
        renderer.write_str("\"></script>")?;
        Ok(())
    }
}
