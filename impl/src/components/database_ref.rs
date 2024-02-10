use delegate_trait::{Context, TraitConfig};
use proc_macro2::TokenStream;
use quote::quote;

pub fn impl_database_ref(context: &Context, config: &TraitConfig) -> syn::Result<TokenStream> {
    let to = &config.to;
    let trait_ident = &config.ident;
    let root = quote! { ::revm_delegate::__private };
    let trait_path = quote! { #root::revm:: #trait_ident };
    let wi = config.wi.clone().unwrap_or_default();

    let methods = quote! {
        #wi

        #root::delegate! { to #to {
            #[through(#trait_path)]
            fn basic_ref(
                &self,
                address: #root::revm::primitives::Address
            ) -> Result<Option<#root::revm::primitives::AccountInfo>, Self::Error>;

            #[through(#trait_path)]
            fn block_hash_ref(
                &self,
                number: #root::revm::primitives::U256
            ) -> Result<#root::revm::primitives::B256, Self::Error>;

            #[through(#trait_path)]
            fn code_by_hash_ref(
                &self,
                code_hash: #root::revm::primitives::B256
            ) -> Result<#root::revm::primitives::Bytecode, Self::Error>;

            #[through(#trait_path)]
            fn storage_ref(
                &self,
                address: #root::revm::primitives::Address,
                index: #root::revm::primitives::U256
            ) -> Result<#root::revm::primitives::U256, Self::Error>;
        }}
    };

    let res = config.wrap_methods(context, &trait_path, &config.generics, &methods);

    Ok(res)
}
