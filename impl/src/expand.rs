use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::components::implem_trait_from_attribute;

pub fn derive(input: &DeriveInput) -> TokenStream {
    match try_expand(input) {
        Ok(expanded) => expanded,
        Err(err) => {
            let error = err.to_compile_error();
            quote! {
                #error
            }
        }
    }
}

pub struct Context<'a> {
    pub ident: &'a syn::Ident,
    pub generics: &'a syn::Generics,
}

impl<'a> Context<'a> {
    pub fn new(input: &'a DeriveInput) -> Self {
        Self { ident: &input.ident, generics: &input.generics }
    }

    pub fn in_impl(&self, trait_for: &TokenStream, tokens: &TokenStream) -> TokenStream {
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();
        let ident = self.ident;
        quote! {
            impl #impl_generics #trait_for #ident #ty_generics #where_clause {
                #tokens
            }
        }
    }
}

fn try_expand(input: &DeriveInput) -> syn::Result<TokenStream> {
    let context = Context::new(input);
    let mut res = TokenStream::default();

    for attr in input.attrs.iter() {
        let trait_implem = implem_trait_from_attribute(&context, attr);
        res.extend(trait_implem);
    }

    Ok(res)
}