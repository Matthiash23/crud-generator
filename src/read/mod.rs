use proc_macro2::{Ident, TokenStream};

mod func_gen;
mod endpoint_gen;

pub fn generate_read(mut r: TokenStream, s_ident: &Ident, endpoint: &str, endpoint_w_param: &str, endpoint_config: TokenStream) -> (TokenStream, TokenStream) {
    r = func_gen::generate_read_funcs(r, s_ident);
    endpoint_gen::generate_read_endpoints(r, s_ident, endpoint, endpoint_w_param, endpoint_config)
}


