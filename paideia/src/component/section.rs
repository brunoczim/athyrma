#[derive(Debug)]
pub struct SectionComponent {
    _priv: (),
}

impl SectionComponent {
    pub(crate) fn new() -> Self {
        Self { _priv: () }
    }
}
