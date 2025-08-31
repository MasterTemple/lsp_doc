use core::panic;
use proc_macro2::TokenTree;
use quote::quote;
use std::fs::read_to_string;
use syn::{Attribute, Item};

#[proc_macro_attribute]
pub fn lsp_doc(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attr: proc_macro2::TokenStream = attr.into();
    let item: proc_macro2::TokenStream = item.into();

    let path = attr
        .clone()
        .into_iter()
        .find_map(|tree| {
            if let TokenTree::Literal(lit) = tree {
                let path = lit.to_string();
                Some(path[1..path.len() - 1].to_string())
            } else {
                None
            }
        })
        .expect("The attribute macro should have a path to the file of type `Literal`.");

    let md = read_to_string(&path).unwrap_or_else(|_| {
        panic!("Could not find {path:?}");
    });
    let doc_comment = format!("\n\n{}\n\n\n", md.trim());
    let doc_attr: Attribute = syn::parse_quote! { #[doc = #doc_comment] };
    let mut item_ast = syn::parse2::<Item>(item.clone())
        .expect("The attribute macro should be applied to a valid Rust item.");

    let attrs = match &mut item_ast {
        Item::Const(c) => &mut c.attrs,
        Item::Enum(e) => &mut e.attrs,
        Item::ExternCrate(e) => &mut e.attrs,
        Item::Fn(f) => &mut f.attrs,
        Item::ForeignMod(f) => &mut f.attrs,
        Item::Impl(i) => &mut i.attrs,
        Item::Macro(m) => &mut m.attrs,
        Item::Mod(m) => &mut m.attrs,
        Item::Static(s) => &mut s.attrs,
        Item::Struct(s) => &mut s.attrs,
        Item::Trait(t) => &mut t.attrs,
        Item::TraitAlias(t) => &mut t.attrs,
        Item::Type(t) => &mut t.attrs,
        Item::Union(u) => &mut u.attrs,
        Item::Use(u) => &mut u.attrs,
        _ => {
            let error_msg = "lsp_doc can only be used on items that support attributes.";
            return quote! { compile_error!(#error_msg); #item_ast }.into();
        }
    };

    attrs.push(doc_attr);
    quote! { #item_ast }.into()
}
