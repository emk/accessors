//! Support for `#[derive(accesors)]`.  Based on the [example code][] for
//! syn.
//!
//! [example code]: https://github.com/dtolnay/syn

#![feature(proc_macro, proc_macro_lib)]

// I threw this code together in just a few minutes, and it could use a
// good refactoring once I figure out the basic ideas.  Do not use use this
// as an example of good style.

#[macro_use]
extern crate log;
extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use std::collections::BTreeMap;

#[proc_macro_derive(getters)]
pub fn derive_getters(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input(&input.to_string()).unwrap();
    let expanded = expand_getters(ast);
    expanded.to_string().parse().unwrap()
}

fn expand_getters(mut ast: syn::MacroInput) -> quote::Tokens {
    // println!("Defining getters for: {:#?}", ast);

    extract_attrs(&mut ast.attrs, "getters");

    let fields: Vec<_> = match ast.body {
        syn::Body::Struct(syn::VariantData::Struct(ref fields)) => {
            fields.iter().map(|f| (f.ident.as_ref().unwrap(), &f.ty)).collect()
        }
        _ => panic!("#[derive(getters)] can only be used with braced structs"),
    };

    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics
        .split_for_impl();
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
    // println!("Expanded: {}", expanded.to_string());
    expanded.to_string().parse().unwrap()
}

fn expand_setters(mut ast: syn::MacroInput) -> quote::Tokens {
    // println!("Defining setters for: {:#?}", ast);

    let setters_attrs = extract_attrs(&mut ast.attrs, "setters");
    let config = config_from(&setters_attrs, &["into"]);
    // println!("Config: {:#?}", &config);
    let into_default = syn::Lit::Bool(false);
    let into = match *config.get("into").unwrap_or(&into_default) {
        syn::Lit::Bool(b) => b,
        ref val => panic!("'into' must be a boolean value, not {:?}", val),
    };

    let fields: Vec<_> = match ast.body {
        syn::Body::Struct(syn::VariantData::Struct(ref fields)) => {
            fields.iter().map(|f| (f.ident.as_ref().unwrap(), &f.ty)).collect()
        }
        _ => panic!("#[derive(setters)] can only be used with braced structs"),
    };

    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics
        .split_for_impl();
    let setters: Vec<_> = fields.iter()
        .map(|&(ref field_name, ref ty)| {
            let set_fn_name: syn::Ident = format!("set_{}", field_name).into();
            if into {
                quote! {
                    pub fn #set_fn_name<T>(&mut self, value: T)
                        where T: Into<#ty>
                    {
                        self.#field_name = value.into();
                    }
                }
            } else {
                quote! {
                    pub fn #set_fn_name(&mut self, value: #ty) {
                        self.#field_name = value;
                    }
                }
            }
        })
        .collect();

    quote! {
        #ast

        impl #impl_generics #name #ty_generics #where_clause {
            #(#setters)*
        }
    }
}

fn extract_attrs(attrs: &mut Vec<syn::Attribute>,
                 name: &str)
                 -> Vec<syn::Attribute> {
    let extracted =
        attrs.iter().filter(|a| a.name() == name).cloned().collect();
    attrs.retain(|a| a.name() != name);
    extracted
}

fn config_from(attrs: &[syn::Attribute],
               keys: &[&str])
               -> BTreeMap<String, syn::Lit> {
    let mut result = BTreeMap::new();
    for attr in attrs {
        if let syn::MetaItem::List(_, ref args) = attr.value {
            for arg in args {
                let name = arg.name();
                if !keys.contains(&name) {
                    panic!("'{}' in {:?} is not a known attribute", name, attr);
                }
                match *arg {
                    syn::MetaItem::Word(_) => {
                        result.insert(name.to_owned(), syn::Lit::Bool(true));
                    }
                    syn::MetaItem::NameValue(_, ref value) => {
                        result.insert(name.to_owned(), value.to_owned());
                    }
                    _ => panic!("can't parse '{:?}'", &arg),
                }
            }
        } else {
            panic!("{:?} must be a key-value attribute", attr);
        }
    }
    result
}
