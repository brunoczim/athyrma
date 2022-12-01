use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    parse2,
    parse_macro_input,
    punctuated::{Pair, Punctuated},
    token::Comma,
    DeriveInput,
    GenericParam,
    Token,
    Type,
    WhereClause,
};

#[derive(Debug, Clone)]
struct RenderAttr {
    generic_params: Punctuated<GenericParam, Comma>,
    where_clause: Option<WhereClause>,
    format: Type,
}

impl Parse for RenderAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        parenthesized!( content in input );
        let mut generic_params = Punctuated::<GenericParam, Comma>::new();
        if content.peek(Token![<]) {
            content.parse::<Token![<]>()?;
            generic_params = content.parse_terminated(GenericParam::parse)?;
            content.parse::<Token![>]>()?;
        }

        let format = content.parse()?;

        let mut where_clause = None;
        if content.peek(Token![where]) {
            where_clause = Some(content.parse()?);
        }

        Ok(Self { generic_params, format, where_clause })
    }
}

#[proc_macro_derive(Component, attributes(render))]
pub fn derive_component(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    let generic_params = &derive_input.generics.params;
    let ident = &derive_input.ident;
    let where_clause = derive_input.generics.where_clause.as_ref();
    let mut output = quote! {
        impl<#generic_params> ::paideia::component::Component
            for #ident<#generic_params>
            #where_clause
        {
            type Kind = <
                <#ident as ::paideia::component::BuildComponent>::Target
                as
                ::paideia::component::Component
            >::Kind;
        }
    };

    for attr in derive_input.attrs {
        let parsed_attr: RenderAttr = match parse2(attr.tokens) {
            Ok(attr) => attr,
            Err(error) => return error.into_compile_error().into(),
        };

        let format = parsed_attr.format;
        let mut generic_params = derive_input
            .generics
            .params
            .pairs()
            .map(|pair| match pair {
                Pair::Punctuated(value, comma) => {
                    Pair::Punctuated(value.clone(), comma.clone())
                },
                Pair::End(value) => Pair::End(value.clone()),
            })
            .chain(parsed_attr.generic_params.into_pairs())
            .map(|pair| match pair {
                Pair::Punctuated(value, comma) => {
                    Pair::Punctuated(value, comma)
                },
                Pair::End(value) => Pair::Punctuated(value, Comma {
                    spans: [Span::mixed_site()],
                }),
            })
            .collect::<Vec<_>>();

        generic_params.sort_by_key(|param| match param.value() {
            GenericParam::Lifetime(_) => 0,
            GenericParam::Type(_) => 1,
            GenericParam::Const(_) => 2,
        });

        let generic_params: Punctuated<GenericParam, Comma> =
            generic_params.into_iter().collect();

        let where_clause = match (
            derive_input.generics.where_clause.as_ref(),
            parsed_attr.where_clause,
        ) {
            (Some(clause), None) => Some(clause.clone()),
            (None, Some(clause)) => Some(clause),
            (Some(left), Some(right)) => {
                let mut clause = left.clone();
                clause.predicates.extend(right.predicates);
                Some(clause)
            },
            (None, None) => None,
        };

        output = quote! {
            #output

            impl<#generic_params> ::paideia::render::Render<#format> for
                #ident<#generic_params>
                #where_clause
            {
                fn render(&self,
                    renderer: &mut ::paideia::render::Renderer<#format>,
                    ctx: &mut ::paideia::render::Context<
                        <Self as :::paideia::component::Component>::Kind
                    >,
                ) -: ::std::fmt::Result {
                    ::paideia::component::BuildComponent::build(self)
                        .render(renderer, ctx)
                }
            }
        };
    }

    output.into()
}
