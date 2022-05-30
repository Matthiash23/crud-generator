use std::ops::Add;

use proc_macro2::{Ident, TokenStream};
use quote::format_ident;
use quote::ToTokens;
use syn::Lifetime;

use crate::ParsedField;

mod struct_gen;
mod func_gen;
mod endpoint_gen;


pub fn generate_create(data_struct: &Vec<ParsedField>, main_lifetime: &Lifetime,
                       r: TokenStream, table_name_quote: impl ToTokens, s_id: &str, s_ident: &Ident, endpoint: &str, endpoint_config: TokenStream ) -> (TokenStream, TokenStream) {
    let ns_id = format_ident!("{}", "New".to_string().add(s_id));
    let (mut r, create_fs) = struct_gen::generate_create_struct(data_struct, main_lifetime, r, table_name_quote, &ns_id, s_ident);
    r = func_gen::generate_create_funcs(r, &ns_id, s_ident, create_fs);
    endpoint_gen::generate_create_endpoints(r, s_ident, &ns_id, endpoint, endpoint_config)
}