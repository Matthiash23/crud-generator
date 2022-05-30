use proc_macro::TokenStream;

use proc_macro2::Span;
use quote::{format_ident, quote, ToTokens};
use syn::{Attribute, Data, DeriveInput, Field, Fields, Lifetime, LitStr, MetaList, MetaNameValue, parse2, parse_macro_input, Path};
use syn::punctuated::Punctuated;
use syn::token::Comma;

use commons::ParsedField;
use helpers::camel_to_snake;

use crate::create::generate_create;
use crate::delete::generate_delete;
use crate::faker::implement_faker;
use crate::patch::generate_patch;
use crate::read::generate_read;

mod helpers;
mod commons;
mod read;
mod create;
mod patch;
mod delete;
mod faker;

// #[proc_macro_error]
#[proc_macro_derive(GenerateCrud, attributes(operations, table_name, creatable, patchable))]
pub fn generate_crud(item: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(item);
    let mut r = quote! {
        pub trait Generate {
            fn generate() -> Self;

            fn generate_multiple(amount: u32) -> Vec<Self> where Self: Sized;
        }
        pub trait FromMultiple<T>: Sized {
            fn from_multiple(_: T) -> Vec<Self>;
        }
    };

    let s_id = ast.ident.to_string();
    let s_ident = format_ident!("{}", &s_id);
    let (derived_traits,
        operations,
        parsed_table_name) = parse_attributes(&ast.attrs);
    let (table_name, table_name_quote) = format_table_name(parsed_table_name, &s_id);
    let endpoint = format!(r#"/{}"#, table_name);
    let endpoint_w_param = format!(r#"/{}/{{id}}"#, table_name);
    let mut endpoint_config = quote! {cfg};
    let endpoint_cfg_ident = format_ident!("{}_endpoints_config", table_name);

    let p_data = parse_data(ast.data);
    let data_struct = buil_data_struct(p_data);

    let main_lifetime = Lifetime::new("'a", Span::call_site());

    if operations.is_some() && operations.as_ref().unwrap().contains(&"create".to_string()) {
        (r, endpoint_config) = generate_create(&data_struct, &main_lifetime, r, &table_name_quote, &s_id, &s_ident, &endpoint, endpoint_config);
    }

    if operations.is_some() && operations.as_ref().unwrap().contains(&"read".to_string()) {
        (r, endpoint_config) = generate_read(r, &s_ident, &endpoint, &endpoint_w_param, endpoint_config);
    }
    // dbg! {&endpoint_config};

    if operations.is_some() && operations.as_ref().unwrap().contains(&"update".to_string()) {
        // todo : r = generate_update( r, &s_ident);
    }

    if operations.is_some() && operations.as_ref().unwrap().contains(&"patch".to_string()) {
        (r, endpoint_config) = generate_patch(&data_struct, &main_lifetime, r, &table_name_quote, &s_id, &s_ident, &endpoint_w_param, endpoint_config);
    }

    if operations.is_some() && operations.as_ref().unwrap().contains(&"delete".to_string()) {
        (r, endpoint_config) = generate_delete(r, &s_ident, &endpoint_w_param, endpoint_config);
    }

    if derived_traits.contains(&"Dummy".to_string()) {
        r = implement_faker(r, &s_id);
    }

    r = quote! {
        #r
        pub fn #endpoint_cfg_ident(cfg: &mut web::ServiceConfig) {
            #endpoint_config;
        }
    };

    r.into()
}

fn parse_attributes(attrs: &[Attribute]) -> (Vec<String>, Option<Vec<String>>, Option<String>) {
    let mut derived_traits: Vec<String> = vec!();
    let mut operations: Option<Vec<String>> = None;
    let mut table_name: Option<String> = None;

    for x in attrs.iter() {
        let id = x.path.segments.last().unwrap().ident.to_string();

        let m = x.parse_meta().unwrap();
        match id.as_str() {
            "derive" => {
                let ml: MetaList = parse2(m.to_token_stream()).unwrap();
                derived_traits = ml.nested.into_iter().map(|nm| {
                    let t: Path = parse2(nm.to_token_stream()).unwrap();
                    t.segments.last().unwrap().ident.to_string()
                }).collect();
            }
            "operations" => {
                let ml: MetaList = parse2(m.to_token_stream()).unwrap();
                operations = ml.nested.into_iter().map(|nm| {
                    let t: LitStr = parse2(nm.to_token_stream()).unwrap();
                    Some(t.value())
                }).collect();
            }
            "table_name" => {
                let mn: MetaNameValue = parse2(m.to_token_stream()).unwrap();
                let t: LitStr = parse2(mn.lit.to_token_stream()).unwrap();
                table_name = Some(t.value());
            }
            _ => {}
        }
    };
    (derived_traits, operations, table_name)
}

fn parse_inner_attributes(acc: (bool, bool), a: &Attribute) -> (bool, bool) {
    if acc.eq(&(true, true)) {
        return acc;
    }
    let (mut creatable, mut patchable) = acc;

    let id = a.path.segments.last().unwrap().ident.to_string();

    match id.as_str() {
        "creatable" => creatable = true,
        "patchable" => patchable = true,
        _ => {}
    }
    (creatable, patchable)
}

fn parse_data(data: Data) -> Punctuated<Field, Comma> {
    match data {
        Data::Struct(data) => {
            if let Fields::Named(fields) = data.fields {
                fields.named
            } else {
                // emit_error!(data);
                panic!("There should at least be one named field");
            }
        }
        _ => {
            // emit_error!(data, "This macro only accepts a structure");
            panic!("This macro only accepts a struct");
        }
    }
}

fn buil_data_struct(p_data: Punctuated<Field, Comma>) -> Vec<ParsedField> {
    p_data.into_iter().map(|f| {
        let (creatable, patchable) = f.attrs.iter().fold((false, false), parse_inner_attributes);
        ParsedField {
            ident: f.ident.unwrap(),
            ty: f.ty,
            vis: f.vis,
            creatable,
            patchable,
        }
    }).collect()
}

fn format_table_name(name: Option<String>, s_id: &str) -> (String, impl ToTokens) {
    let i = if name.is_some() {
        format!(r#"{}"#, name.unwrap())
    } else {
        format!(r#"{}s"#, camel_to_snake(s_id))
    };
    (i.clone(), quote! { #[table_name=#i]})
}
