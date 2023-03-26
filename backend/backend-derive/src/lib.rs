use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};
use quote::quote;

#[proc_macro_derive(Creatable)]
pub fn derive_creatable(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl Creatable for #ident {}
    };
    output.into()
}

#[proc_macro_derive(Patchable)]
pub fn derive_patchable(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl Patchable for #ident {}
    };
    output.into()
}

#[proc_macro_derive(UntisResult)]
pub fn derive_untis_result(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl UntisResult for #ident {}
    };
    output.into()
}

#[proc_macro_derive(UntisArrayResult)]
pub fn derive_untis_array_result(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl UntisArrayResult for #ident {}
    };
    output.into()
}
