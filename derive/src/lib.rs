#![allow(unused_variables,unused_assignments)]
use proc_macro::{TokenStream,};
use proc_macro2::{Span};
use quote::quote;

#[proc_macro_derive(Querio, attributes(querio))]
pub fn querio_input_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_querio_input_macro(&ast)
}

#[cfg(feature = "flatten")]
fn flatten (str: &str) -> String{
    let re = regex::Regex::new(r"\s+").unwrap();
    re.replace_all(&str, " ").to_string()
}

#[cfg(not(feature = "flatten"))]
fn flatten (str: &str) -> String{
    str.to_string()
}

fn impl_querio_input_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();
    if let syn::Data::Struct(strct) = &ast.data {

        let mut out = quote!();
        for aaa in &ast.attrs {
            let ewr = aaa.path.get_ident();
            if &ewr.as_ref().unwrap().to_string() == "querio" {
                if let Ok(bbb) = aaa.parse_meta() {
                    if let syn::Meta::List(list) = bbb {

                        let mut input_a     = syn::Ident::new("QuerioInputUnit",    Span::call_site());
                        let mut input_b     = syn::Ident::new("QuerioInputUnit",    Span::call_site());
                        let mut output      = syn::Ident::new("QuerioOutputUnit",   Span::call_site());
                        let mut variables   = syn::Ident::new("QuerioVariableUnit", Span::call_site());

                        let a = list.nested.last().unwrap();
                        let mut query = if let syn::NestedMeta::Lit(lit) = a { if let syn::Lit::Str(str) = lit {
                            str.value()
                        } else {panic!("Nope")}} else {panic!("Nope")};

                        
                        for abc in list.nested {
                            //
                            if let syn::NestedMeta::Meta(meta) = abc {
                                //
                                if let syn::Meta::List(list) = meta {
                                    let s = &list.path.segments.last().unwrap().ident.to_string();

                                    if s=="input" || s=="I" { if let Some(syn::NestedMeta::Meta(meta)) = list.nested.first(){ if let syn::Meta::Path(path) = meta {
                                        input_a = path.get_ident().unwrap().clone();
                                        if list.nested.len() > 1 {if let Some(syn::NestedMeta::Meta(meta)) = list.nested.last(){ if let syn::Meta::Path(path) = meta {
                                            input_b = path.get_ident().unwrap().clone();
                                        } else {panic!("Nope")}}}
                                    } else {panic!("Nope")}} else {panic!("Nope")}}
                                    else if s=="output" || s=="O" {if let Some(syn::NestedMeta::Meta(meta)) = list.nested.first(){ if let syn::Meta::Path(path) = meta {
                                        output = path.get_ident().unwrap().clone();
                                    } else {panic!("Nope")}}}
                                    else if s=="variables" || s=="V" {if let Some(syn::NestedMeta::Meta(meta)) = list.nested.first(){ if let syn::Meta::Path(path) = meta {
                                        variables = path.get_ident().unwrap().clone();
                                    } else {panic!("Nope")}} else {panic!("Nope")}}
                                    else if s=="sections" || s=="S" { 
                                        let mut i = 0;
                                        for l in list.nested { if let syn::NestedMeta::Lit(lit) = l { if let syn::Lit::Str(str) = lit {
                                            query = query.replace(&format!("<{}>",i),&str.value());
                                        } else {panic!("Nope")}} else {panic!("Nope")}
                                        i += 1;
                                    }}

                                }
                                //
                            }
                            //
                        }
                        query = flatten(&query);

                        let mut ni = quote!{};
                        #[cfg(feature = "native_input")]
                        {ni = quote!{
                            type QuerioInputA = #input_a;
                            type QuerioInputB = #input_b;
                        };}

                        let mut no = quote!{};
                        #[cfg(feature = "native_output")]
                        {no = quote!{
                            type QuerioOutput = #output;
                        };}

                        let mut va = quote!{};
                        #[cfg(feature = "variables")]
                        {va = quote!{
                            type QuerioVariable = #variables;
                        };}

                        out = quote!{
                            impl #impl_generics Querio for #name #ty_generics #where_clause {
                                #ni
                                #no
                                #va
                                const QUERY: &'static str = #query;
                            }
                        }
                    }
                }
            }
        }
        out.into()
    } else {panic!("Not a Struct!")}
}