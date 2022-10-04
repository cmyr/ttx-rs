
use proc_macro::TokenStream;
use syn::{ItemStatic};

#[proc_macro_attribute]
pub fn ttx_me_baby(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{}\"", attr.to_string());
    println!("item: \"{}\"", item.to_string());

    // Parse the string representation into a syntax tree
    let input = item.clone();
    let ast = syn::parse_macro_input!(input as ItemStatic);

    println!("ast \"{:#?}\"", ast);

    item
}