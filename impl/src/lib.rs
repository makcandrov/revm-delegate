use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod components;
mod expand;
mod idents;

#[proc_macro_derive(RevmDelegate, attributes(revm_delegate))]
pub fn derive_revm_delegate(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand::derive(&input).into()
}
