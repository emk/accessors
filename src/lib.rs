//! Support for `#[derive(accesors)]`.  Based on the [example code][] for
//! syn.
//!
//! [example code]: https://github.com/dtolnay/syn

#![feature(proc_macro, proc_macro_lib)]

#[macro_use]
extern crate log;
extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;

#[proc_macro_derive(accessors)]
pub fn derive(input: TokenStream) -> TokenStream {
    let source = input.to_string();

    // Parse the string representation to an AST.
    let ast = syn::parse_macro_input(&source).unwrap();

    // Build the output.
    let expanded = expand_accessors(ast);

    // Parse back to a token stream and return it
    expanded.to_string().parse().unwrap()
}

fn expand_accessors(ast: syn::MacroInput) -> quote::Tokens {
    println!("Defining accessors for: {:#?}", ast);

    let fields: Vec<_> = match ast.body {
        syn::Body::Struct(ref data) => {
            data.fields().iter().map(|f| (f.ident.as_ref().unwrap(), &f.ty)).collect()
        }
        _ => panic!("#[derive(accessors)] can only be used with structs"),
    };

    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let getter: Vec<_> = fields.iter().map(|f| f.0).collect();
    let setter: Vec<syn::Ident> = fields
        .iter()
        .map(|f| {
            let s: &str = f.0.as_ref();
            format!("set_{}", s).into()
        })
        .collect();
    let field1: Vec<_> = fields.iter().map(|f| f.0).collect();
    let ty1: Vec<_> = fields.iter().map(|f| f.1).collect();
    let field2: Vec<_> = fields.iter().map(|f| f.0).collect();
    let ty2: Vec<_> = fields.iter().map(|f| f.1).collect();

    quote! {
        #ast

        impl #impl_generics #name #ty_generics #where_clause {
            #(
                pub fn #getter(&self) -> &#ty1 {
                    &self.#field1
                }

                pub fn #setter(&mut self, value: #ty2) {
                    self.#field2 = value;
                }
            )*
        }
    }
}
