use proc_macro2::{Ident, TokenStream};

mod func_gen;
mod endpoint_gen;

pub fn generate_delete(mut r: TokenStream, s_ident: &Ident,  endpoint_w_param: &str, endpoint_config: TokenStream) -> (TokenStream, TokenStream) {
    r = func_gen::generate_delete_func(r, s_ident);
    endpoint_gen::generate_delete_endpoints(r, s_ident, endpoint_w_param, endpoint_config)
}