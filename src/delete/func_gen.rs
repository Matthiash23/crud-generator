use proc_macro2::{Ident, TokenStream};
use quote::quote;

pub fn generate_delete_func(mut r: TokenStream, s_ident: &Ident) -> TokenStream {
    r = quote! {
        #r
        impl #s_ident {
            pub fn delete(id: i32, c: &PgConnection) {
                diesel::delete(table.filter(s_id.eq(id)))
                    .execute(c)
                    .expect("Error");
            }
        }
    };
    r
}