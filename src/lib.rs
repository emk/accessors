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

#[proc_macro_derive(getters)]
pub fn derive_getters(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input(&input.to_string()).unwrap();
    let expanded = expand_getters(ast);
    expanded.to_string().parse().unwrap()
}

fn expand_getters(ast: syn::MacroInput) -> quote::Tokens {
    //println!("Defining getters for: {:#?}", ast);

    let fields: Vec<_> = match ast.body {
        syn::Body::Struct(syn::VariantData::Struct(ref fields)) => {
            fields.iter().map(|f| (f.ident.as_ref().unwrap(), &f.ty)).collect()
        }
        _ => panic!("#[derive(getters)] can only be used with braced structs"),
    };

    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let getter: Vec<_> = fields.iter().map(|f| f.0).collect();
    let field: Vec<_> = fields.iter().map(|f| f.0).collect();
    let ty: Vec<_> = fields.iter().map(|f| f.1).collect();

    quote! {
        #ast

        impl #impl_generics #name #ty_generics #where_clause {
            #(
                pub fn #getter(&self) -> &#ty {
                    &self.#field
                }
            )*
        }
    }
}

#[proc_macro_derive(setters)]
pub fn derive_setters(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input(&input.to_string()).unwrap();
    let expanded = expand_setters(ast);
    expanded.to_string().parse().unwrap()
}

fn expand_setters(ast: syn::MacroInput) -> quote::Tokens {
    //println!("Defining setters for: {:#?}", ast);

    let fields: Vec<_> = match ast.body {
        syn::Body::Struct(syn::VariantData::Struct(ref fields)) => {
            fields.iter().map(|f| (f.ident.as_ref().unwrap(), &f.ty)).collect()
        }
        _ => panic!("#[derive(setters)] can only be used with braced structs"),
    };

    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let setter: Vec<syn::Ident> = fields
        .iter()
        .map(|f| {
            let s: &str = f.0.as_ref();
            format!("set_{}", s).into()
        })
        .collect();
    let field: Vec<_> = fields.iter().map(|f| f.0).collect();
    let ty: Vec<_> = fields.iter().map(|f| f.1).collect();

    quote! {
        #ast

        impl #impl_generics #name #ty_generics #where_clause {
            #(
                pub fn #setter(&mut self, value: #ty) {
                    self.#field = value;
                }
            )*
        }
    }
}
