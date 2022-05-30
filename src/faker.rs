use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub fn implement_faker(mut r: TokenStream, s_id: &str) -> TokenStream {
    let s_ident = format_ident!("{}", &s_id);
    r = quote! {
        #r
        impl Generate for #s_ident {
            fn generate() -> #s_ident {
                let r: #s_ident = Faker.fake();
                r
            }

            fn generate_multiple(amount: u32) -> Vec<Self> {
                let mut r: Vec<Self> = vec![];
                for _n in 1..amount  {
                    let g: Self = Self::generate();
                    r.push(g)
                }

                r
            }
        }
    };
    r
}