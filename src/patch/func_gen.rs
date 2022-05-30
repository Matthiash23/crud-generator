use proc_macro2::{Ident, TokenStream};
use quote::quote;

pub fn generate_patch_func(mut r: TokenStream, ps_id: &Ident, s_ident: &Ident) -> TokenStream {
    r = quote! {
        #r
        impl #s_ident {
            pub fn patch(id: i32, data: #ps_id, c: &PgConnection) -> #s_ident {
                diesel::update(table.filter(s_id.eq(id)))
                    .set(data)
                    .get_result(c)
                    .expect("error patch")
            }
        }
    };
    r
}