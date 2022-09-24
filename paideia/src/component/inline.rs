use super::ComponentKind;

pub mod text;

#[derive(Debug)]
pub struct InlineComponent {
    _priv: (),
}

impl ComponentKind for InlineComponent {}
