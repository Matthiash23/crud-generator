use std::ops::Add;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, ToTokens};
use syn::Lifetime;

use crate::ParsedField;

mod struct_gen;
mod func_gen;
mod endpoint_gen;

pub fn generate_patch(data_struct: &Vec<ParsedField>, main_lifetime: &Lifetime,
                      mut r: TokenStream, table_name_quote: impl ToTokens, s_id: &str, s_ident: &Ident, endpoint_w_param: &str, endpoint_config: TokenStream) -> (TokenStream, TokenStream) {
    let ps_id = format_ident!("{}", "Patch".to_string().add(s_id));

    r = struct_gen::generate_patch_struct(data_struct, main_lifetime, r, table_name_quote, &ps_id);
    r = func_gen::generate_patch_func(r, &ps_id, s_ident);
    endpoint_gen::generate_patch_endpoint(r, s_ident, &ps_id, endpoint_w_param, endpoint_config)
}