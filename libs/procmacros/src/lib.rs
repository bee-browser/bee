use proc_macro::TokenStream;
use quote::quote;
use syn::LitStr;
use syn::parse_macro_input;

#[proc_macro]
pub fn utf16_size(input: TokenStream) -> TokenStream {
    let utf8 = parse_macro_input!(input as LitStr).value();
    let size = utf8.encode_utf16().count();
    quote! {
        #size
    }
    .into()
}

#[proc_macro]
pub fn utf16_array(input: TokenStream) -> TokenStream {
    let utf8 = parse_macro_input!(input as LitStr).value();
    let utf16 = utf8.encode_utf16().collect::<Vec<_>>();
    quote! {
        [#(#utf16),*]
    }
    .into()
}

#[proc_macro]
pub fn utf16_slice(input: TokenStream) -> TokenStream {
    let utf8 = parse_macro_input!(input as LitStr).value();
    let utf16 = utf8.encode_utf16().collect::<Vec<_>>();
    quote! {
        [#(#utf16),*].as_slice()
    }
    .into()
}
