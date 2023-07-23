use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::Result;

#[proc_macro]
pub fn gpt(tokens: TokenStream) -> TokenStream {
    match gpt_internal(tokens) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.into_compile_error().into(),
    }
}

fn gpt_internal(tokens: impl Into<TokenStream2>) -> Result<TokenStream2> {
    Ok(quote!())
}
