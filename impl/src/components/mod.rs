use delegate_trait::TraitConfig;
use proc_macro2::TokenStream;

use crate::{
    expand::Context,
    idents::{REVM_HELPER, DATABASE, DATABASE_REF},
};

mod database;
mod database_ref;
mod inspector;

pub fn implem_trait_from_attribute(
    context: &Context<'_>,
    attr: &syn::Attribute,
) -> syn::Result<TokenStream> {
    if attr.path().is_ident(REVM_HELPER) {
        let list = attr.meta.require_list()?;
        let config = list.parse_args::<TraitConfig>()?;

        return match config.ident.to_string().as_str() {
            DATABASE => database::impl_database(context, &config),
            DATABASE_REF => database_ref::impl_database_ref(context, &config),
            _ => Err(syn::Error::new_spanned(&config.ident, "Unknown trait.")),
        };
    }

    Ok(Default::default())
}
