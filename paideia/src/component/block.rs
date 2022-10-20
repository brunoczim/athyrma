use super::ComponentKind;

pub mod text;
pub mod list;

#[derive(Debug)]
pub struct BlockComponent {
    _priv: (),
}

impl ComponentKind for BlockComponent {}
