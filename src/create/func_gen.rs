use proc_macro2::{Ident, Punct, Spacing, TokenStream};
use quote::{quote, TokenStreamExt, ToTokens};
use syn::TypeReference;

use super::struct_gen::CreateField;

struct ParamCreateField<'i> {
    ident: &'i Ident,
    ty: TypeReference,
}

impl<'i> ToTokens for ParamCreateField<'i> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.ident.to_tokens(tokens);
        tokens.append(Punct::new(':', Spacing::Alone));
        self.ty.to_tokens(tokens)
    }
}

struct IdentCreateField<'i> {
    ident: &'i Ident,
}

impl<'i> ToTokens for IdentCreateField<'i> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.ident.to_tokens(tokens)
    }
}

pub fn generate_create_funcs(mut r: TokenStream, ns_id: &Ident, s_ident: &Ident, create_fs: Vec<CreateField>) -> TokenStream {
    let (param_create_fs, ident_create_fs) = generate_fields(&create_fs);
    r = quote! {
        #r
        impl #s_ident {
            pub fn create(x: #ns_id, c: &PgConnection) -> #s_ident {
                diesel::insert_into(table)
                    .values(&x)
                    .get_result(c)
                    .expect("Error saving new track")
            }

            pub fn create_multiple(x: Vec<#ns_id>, c: &PgConnection) -> Vec<#s_ident> {
                diesel::insert_into(table)
                    .values(&x)
                    .get_results(c)
                    .expect("Error saving new tracks")
            }

            pub fn create_from( #(#param_create_fs),*, c: &PgConnection) -> Vec<#s_ident> {
                
                let new_s = #ns_id {
                    #(#ident_create_fs),*
                };
                
                diesel::insert_into(table)
                    .values(&new_s)
                    .get_results(c)
                    .expect("Error saving new tracks")
            }
        }
    };
    r
}

fn generate_fields<'i>(create_fs: &'i [CreateField]) -> (Vec<ParamCreateField<'i>>, Vec<IdentCreateField<'i>>) {
    let param_fs = create_fs.iter().map(|x| {
        let mut ty = x.ty.clone();
        ty.lifetime = None;
        ParamCreateField {
            ident: x.ident,
            ty: ty,
        }
    }).collect();

    let ident_fs = create_fs.iter().map(|x| {
        IdentCreateField {
            ident: x.ident
        }
    }).collect();

    (param_fs, ident_fs)
}
