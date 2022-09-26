use proc_macro::TokenStream;
use quote::quote;
use syn::{self, TypePath};

/* ---------------------------------- Input --------------------------------- */

#[proc_macro_derive(QuerioRGInput, attributes(querio))]
pub fn querio_input_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_querio_input_macro(&ast)
}

fn impl_querio_input_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();
    if let syn::Data::Struct(strct) = &ast.data {
        let mut cscd = vec![];
        if let syn::Fields::Named(fields) = &strct.fields {
            let mut text = "".to_string();
            let mut idents = vec![];
            for field in &fields.named {

                let f_ident = field.ident.as_ref().unwrap().clone();

                let mut ignore = false;
                for aaa in &field.attrs {
                    let ewr = aaa.path.get_ident();
                    let nme = ewr.as_ref().unwrap().to_string();
                    if &nme == "querio" {
                        if let Ok(bbb) = aaa.parse_meta() {
                            if let syn::Meta::List(list) = bbb {
                                for abc in list.nested {
                                    if let syn::NestedMeta::Meta(meta) = abc {
                                        if let syn::Meta::Path(pp) = meta {
                                            let nident = pp.get_ident().as_ref().unwrap().to_string();
                                            if &nident == "cascade" {
                                                cscd.push(f_ident.clone());
                                            } else if &nident == "ignore" {
                                                ignore = true;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                if ignore {continue;}

                match &field.ty {
                    syn::Type::Path(ty) => {
                        text += " ";
                        idents.push(field.ident.as_ref().unwrap().clone());
                        text += &field.ident.as_ref().unwrap().to_string();
                        text += "=";
                        let ident = ty.path.segments.last().unwrap().ident.to_string();
                        text += if ident=="String" {
                            "\"{}\""
                        } else {"{}"};
                    }
                    _ => todo!(),
                }
            }
            let gen = quote! {
                impl #impl_generics QuerioInput for #name #ty_generics #where_clause {
                    fn querio_input(&self) -> String {
                        let mut a = format!(#text #(,self.#idents)* );
                        #(a += &self.#cscd.querio_input() )*;
                        a
                    }
                }
            };
            gen.into()
        } else {
            let gen = quote! {
                impl #impl_generics QuerioInput for #name #ty_generics #where_clause {
                    fn querio_input(&self) -> String {
                        let mut  a = "".to_string();
                        #(a += &self.#cscd.querio_input() )*;
                        a
                    }
                }
            };
            gen.into()
        }
    } else {panic!("Not a Struct!")}
}

/* --------------------------------- Output --------------------------------- */

#[proc_macro_derive(QuerioRGOutput)]
pub fn querio_output_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_querio_output_macro(&ast)
}

fn impl_querio_output_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    if let syn::Data::Struct(strct) = &ast.data {
        let mut text: String = "".into();
        match &strct.fields {
            syn::Fields::Named(fields) => {
                let mut i = 0;
                for field in &fields.named {
                    match &field.ty {
                        syn::Type::Path(_ty) => {
                            let fname = &field.ident.as_ref().unwrap().to_string();
                            text += fname;
                            if i != &fields.named.len()-1 {
                                text += ",";
                            } 
                        },_ => todo!(),
                    }
                    i += 1;
                }
            },
            syn::Fields::Unnamed(fields) => {
                let mut i = 0;
                for field in &fields.unnamed {
                    match &field.ty {
                        syn::Type::Path(_ty) => {
                            text += &i.to_string();
                            if i != &fields.unnamed.len()-1 {
                                text += ",";
                            } 
                        },_ => todo!(),
                    }
                    i += 1;
                }
            }, _ => {}
        }
        let gen = quote! {
            impl QuerioOutput for #name {
                const QUERIO_OUTPUT: &'static str = #text;
            }
        };
        gen.into()
    } else {panic!("Not a Struct!")}
}

/* ------------------------------- Output JSON ------------------------------ */

#[proc_macro_derive(QuerioRGOutputJson)]
pub fn querio_output_json_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_querio_output_json_macro(&ast)
}

fn querio_json_varstr(fname: &String, ty: &TypePath) -> String {
    let ident = ty.path.segments.last().unwrap().ident.to_string();
    if ident=="String" {
        "'\"'+".to_string()+fname+"+'\"'"
    } else {
        fname.to_string()
    }
}

fn querio_json_single(strct: &syn::DataStruct) -> String {
    match &strct.fields {
        syn::Fields::Named(fields) => {
            let mut text = "'{'".to_string();
            let mut i = 0;
            for field in &fields.named {
                match &field.ty {
                    syn::Type::Path(ty) => {
                        let fname = &field.ident.as_ref().unwrap().to_string();
                        text += &format!("+'\"{}\":'+",fname);
                        text += &querio_json_varstr(&fname,ty);
                        if i != &fields.named.len()-1 {
                            text += "+','";
                        } 
                    },_ => todo!(),
                }
                i += 1;
            }
            text += "+'}'";
            text
        },
        syn::Fields::Unnamed(fields) => {
            let mut text = "'['".to_string();
            //
            let mut i = 0;
            for field in &fields.unnamed {
                match &field.ty {
                    syn::Type::Path(ty) => {
                        text += "+";
                        text += &querio_json_varstr(&i.to_string(),ty);
                        if i != &fields.unnamed.len()-1 {
                            text += "+','";
                        } 
                    },_ => todo!(),
                }
                i += 1;
            }
            //
            text += "+']'";
            text
        },
        syn::Fields::Unit => {
            "".to_string()
        },
    }
}

fn impl_querio_output_json_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    if let syn::Data::Struct(strct) = &ast.data {
        let text = format!("{}",querio_json_single(strct));
        let gen = quote! {
            impl QuerioOutput for #name {
                const QUERIO_OUTPUT: &'static str = #text;
            }
        };
        gen.into()
    } else {panic!("Not a Struct!")}
}