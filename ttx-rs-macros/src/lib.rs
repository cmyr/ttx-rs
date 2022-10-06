
use proc_macro::TokenStream;
use syn::{LitStr, Lit, ExprLit};
use syn::{ItemStatic, punctuated::Punctuated, Expr, Token};
use syn::parse::Parser;

#[proc_macro]
pub fn ttx_xml(body: TokenStream) -> TokenStream {
        
    // Did we try parsing the TokenStream directly with TokenTree first? Of course we did...
    // https://docs.rs/syn/1.0.101/syn/parse/index.html
    let tokens = body.clone();
    let parser = Punctuated::<Expr, Token![,]>::parse_terminated;    
    let mut args =  parser.parse(tokens).expect("Illegible args");

    if args.is_empty() {
        panic!("ttx_xml is meaningless without args");
    }

    let mut xml_name = String::from("");
    for (pos, arg) in args.iter().enumerate() {
        println!("{} {:#?}", pos, arg);
        if pos == 0 {
            if let Expr::Lit(literal) = arg {
                if let Lit::Str(lit_str) = &literal.lit {
                    xml_name = lit_str.value()
                }
            }
            if xml_name.is_empty() {
                panic!("First arg must be a literal string, name of the xml element");
            }
        }
    }

    println!("xml_name {}", xml_name); 

    return TokenStream::new();
}

#[proc_macro_attribute]
pub fn ttx_me_baby(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{}\"", attr.to_string());
    println!("item: \"{}\"", item.to_string());

    // Parse the string representation into a syntax tree
    let input = item.clone();
    let ast = syn::parse_macro_input!(input as ItemStatic);

    //println!("attr ast\n\"{:#?}\"", syn::parse_macro_input!(attr as ItemStatic));
    println!("item ast\n\"{:#?}\"", ast);
    if ast.ident.to_string().ne(&String::from("TTX_FIELDS")) {
        panic!("ttx_me_baby must apply to TTX_FIELDS")
    }

    return TokenStream::new();
}