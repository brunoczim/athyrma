macro_rules! impl_text_as_display {
    (<$($gen:ident),*> $ty:ty) => {
        impl<$($gen),*> $crate::Render<$crate::TextRendering> for $ty {
            fn render(
                &self,
                fmtr: &mut std::fmt::Formatter,
                _ctx: &Self::Context,
                _render_format: &$crate::TextRendering
            ) -> std::fmt::Result {
                std::fmt::Display::fmt(self, fmtr)
            }
        }
    };

    (<$($gen:ident,)*> $ty:ty) => {
        impl_text_as_display! { <$($gen),*> $ty }
    };

    ($ty:ty) => {
        impl_text_as_display! { <> $ty }
    };

}
