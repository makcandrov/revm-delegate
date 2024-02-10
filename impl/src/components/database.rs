use delegate_trait::TraitConfig;
use proc_macro2::TokenStream;
use quote::quote;

use crate::expand::Context;

pub fn impl_database(context: &Context, config: &TraitConfig) -> syn::Result<TokenStream> {
    let to = &config.to;
    let trait_ident = &config.ident;
    let wi = config.wi.clone().unwrap_or_default();

    let methods = quote! {
        #wi

        ::revm_delegate::__private::delegate! { to #to {
            #[through(::revm_delegate::__private::revm::Database)]
            fn basic(
                &self,
                address: ::revm_delegate::__private::revm::primitives::Address
            ) -> Result<Option<::revm_delegate::__private::revm::AccountInfo>, Self::Error>;

            #[through(::revm_delegate::__private::revm::Database)]
            fn block_hash(
                &self,
                number: ::revm_delegate::__private::revm::primitives::U256
            ) -> Result<::revm_delegate::__private::revm::primitives::B256, Self::Error>;

            #[through(::revm_delegate::__private::revm::Database)]
            fn code_by_hash(
                &self,
                code_hash: ::revm_delegate::__private::revm::primitives::B256
            ) -> Result<::revm_delegate::__private::revm::primitives::Bytecode, Self::Error>;

            #[through(::revm_delegate::__private::revm::Database)]
            fn storage(
                &self,
                address: ::revm_delegate::__private::revm::primitives::Address,
                index: ::revm_delegate::__private::revm::primitives::U256
            ) -> Result<::revm_delegate::__private::revm::primitives::U256, Self::Error>
        }}
    };

    let trait_for = quote! {
        ::revm_delegate::__private::revm::#trait_ident for
    };

    Ok(context.in_impl(&trait_for, &methods))
}
