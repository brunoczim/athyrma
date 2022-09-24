use super::ComponentKind;

pub mod text;

#[derive(Debug)]
pub struct BlockComponent {
    _priv: (),
}

impl ComponentKind for BlockComponent {}
