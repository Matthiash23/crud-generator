use proc_macro2::{Ident, TokenStream};
use quote::quote;

pub fn generate_read_funcs(mut r: TokenStream, s_ident: &Ident) -> TokenStream {
    r = quote! {
        #r
        impl #s_ident {
            pub fn read(id: i32, c: &PgConnection) -> #s_ident {
                table.find(id)
                        .first(c)
                        .expect("Could not find track")
            }

            pub fn get(amount: i64, c: &PgConnection) -> Vec<#s_ident> {
                table.limit(amount)
                    .load::<#s_ident>(c)
                    .expect("Error loading tracks")
            }
        }
    };
    r
}