use delegate_trait::{Context, TraitConfig};
use proc_macro2::TokenStream;

use crate::
    idents::{DATABASE, DATABASE_REF, INSPECTOR}
;

mod database;
mod database_ref;
mod inspector;

pub fn implem_trait_from_attribute(
    context: &Context<'_>,
    attr: &syn::Attribute,
) -> syn::Result<TokenStream> {
    let list = attr.meta.require_list()?;
    let config = list.parse_args::<TraitConfig>()?;

    return match config.ident.to_string().as_str() {
        DATABASE => database::impl_database(context, &config),
        DATABASE_REF => database_ref::impl_database_ref(context, &config),
        INSPECTOR => inspector::impl_inspector(context, &config),
        _ => Err(syn::Error::new_spanned(&config.ident, "Unknown trait.")),
    };
}
