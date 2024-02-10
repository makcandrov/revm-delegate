use delegate_trait::Context;
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::{components::implem_trait_from_attribute, idents::REVM_HELPER};

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

fn try_expand(input: &DeriveInput) -> syn::Result<TokenStream> {
    let context = Context::new(input);
    let mut res = TokenStream::default();

    for attr in input.attrs.iter().filter(|attr| attr.path().is_ident(REVM_HELPER)) {
        let trait_implem = implem_trait_from_attribute(&context, attr);
        res.extend(trait_implem);
    }

    Ok(res)
}