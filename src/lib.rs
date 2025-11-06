#![cfg_attr(feature = "inline", feature(proc_macro_span))]

mod nightly;
mod stable;

#[proc_macro_attribute]
pub fn lsp_doc(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attr: proc_macro2::TokenStream = attr.into();
    let item: proc_macro2::TokenStream = item.into();

    #[cfg(not(feature = "inline"))]
    {
        stable::lsp_doc(attr, item)
    }

    #[cfg(feature = "inline")]
    {
        nightly::lsp_doc(attr, item)
    }
}
