#![feature(proc_macro_span)]

use std::fs::read_to_string;

use proc_macro2::TokenTree;
use quote::{quote, ToTokens};

fn get_start_index(tree: &TokenTree) -> usize {
    match tree {
        TokenTree::Group(group) => group.span(),
        TokenTree::Ident(ident) => ident.span(),
        TokenTree::Punct(punct) => punct.span(),
        TokenTree::Literal(literal) => literal.span(),
    }
    .unwrap()
    .byte_range()
    .start
}

#[proc_macro_attribute]
pub fn lsp_doc(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attr: proc_macro2::TokenStream = attr.into();
    let item: proc_macro2::TokenStream = item.into();

    let start_pos = attr
        .clone()
        .into_iter()
        .next()
        .and_then(|tree| Some(get_start_index(&tree)))
        .expect("The attribute macro should have a starting position.");

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

    let md = read_to_string(&path).expect(format!("Could not find {path:?}").as_str());

    let mut new_items = vec![];
    let mut inserted = false;
    for tree in item.into_iter() {
        let start = get_start_index(&tree);
        if start > start_pos && inserted == false {
            new_items.push(quote! {
                #[doc = #md]
            });
            inserted = true;
        }
        new_items.push(tree.to_token_stream());
    }

    quote! {
        #(#new_items)*
    }
    .into()
}
