macro_rules! impl_text_as_display {
    (<$($gen:ident),*> $ty:ty) => {
        impl<$($gen),*> $crate::component::Render<$crate::component::TextRendering> for $ty {
            fn render(
                &self,
                fmtr: &mut std::fmt::Formatter,
                _ctx: &$crate::component::Context< $crate::component::TextRendering, Self::Kind>,
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
