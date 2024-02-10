use delegate_trait::{Context, TraitConfig};
use proc_macro2::TokenStream;
use quote::quote;

pub fn impl_inspector(context: &Context, config: &TraitConfig) -> syn::Result<TokenStream> {
    let to = &config.to;
    let trait_ident = &config.ident;
    let trait_generics = &config.generics;
    let root = quote! { ::revm_delegate::__private };
    let trait_path = quote! { #root ::revm:: #trait_ident };
    let wi = config.wi.clone().unwrap_or_default();

    let methods = quote! {
        #wi

        #root::delegate! { to #to {
            #[through(#trait_path)]
            fn initialize_interp(
                &mut self,
                interp: &mut #root::revm::interpreter::Interpreter,
                context: &mut #root::revm::EvmContext #trait_generics
            );

            
        }}
    };

    let res = config.wrap_methods(context, &trait_path, trait_generics, &methods);

    Ok(res)
}
