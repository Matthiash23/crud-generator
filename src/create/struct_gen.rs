use proc_macro2::{Punct, Spacing, TokenStream};
use quote::{quote, TokenStreamExt, ToTokens};
use syn::{Ident, Lifetime, Path, PathSegment, Token, Type, TypePath, TypeReference, Visibility};
use syn::punctuated::Punctuated;

use crate::commons::{IdentParsedField, ParsedField};

#[derive(Debug, Clone)]
pub struct CreateField<'i> {
    pub(crate) ident: &'i Ident,
    pub(crate) ty: TypeReference,
    vis: &'i Visibility,
}

impl<'i> ToTokens for CreateField<'i> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.vis.to_tokens(tokens);
        self.ident.to_tokens(tokens);
        tokens.append(Punct::new(':', Spacing::Alone));
        self.ty.to_tokens(tokens)
    }
}

pub fn generate_create_struct<'i>(data_struct: &'i [ParsedField], main_lifetime: &Lifetime,
                                  mut r: TokenStream, table_name_quote: impl ToTokens, ns_id: &Ident,
                                  s_ident: &Ident) -> (TokenStream, Vec<CreateField<'i>>) {
    let create_fs = build_create_fields(data_struct, main_lifetime).unwrap();
    r = quote! {
            #r
            #[derive(Debug, Insertable, Deserialize)]
            #table_name_quote
            pub struct #ns_id<#main_lifetime> {
                #(#create_fs),*
            }
        };

    let id_fs: Vec<IdentParsedField> = create_fs.iter().map(|x| {
        IdentParsedField {
            ident: x.ident
        }
    }).collect();

    r = quote! {
            #r

            impl<'a> From<&'a #s_ident> for #ns_id<'a> {
                fn from(x: &'a #s_ident) -> Self {
                    Self {
                        #(#id_fs: x.#id_fs.as_ref()),*
                    }
                }
            }

            impl<'a> FromMultiple<&'a Vec<#s_ident>> for #ns_id<'a> {
                fn from_multiple(y: &'a Vec<#s_ident>) -> Vec<Self> {
                    let mut r: Vec<Self> = vec![];
                    for x in y.iter()  {
                        let z: Self = Self {
                            #(#id_fs: x.#id_fs.as_ref()),*
                        };
                        r.push(z)
                    }
                    r
                }
            }
    };
    (r, create_fs)
}

fn build_create_fields<'i>(fields: &'i [ParsedField], l: &Lifetime) -> Option<Vec<CreateField<'i>>> {
    let mut r: Vec<CreateField> = vec![];
    for x in fields.iter() {
        if x.creatable {
            r.push(build_create_field(x, l));
        }
    }
    if !r.is_empty() {
        Some(r)
    } else {
        panic!("Create feature requested but no field is marked as creatable")
    }
}

fn build_create_field<'i>(x: &'i ParsedField, l: &Lifetime) -> CreateField<'i> {
    match &x.ty {
        Type::Path(t) => {
            let r: Punctuated<PathSegment, Token![::]> = t.path.segments.clone().into_iter().map(|mut x| {
                if x.ident.to_string().eq("String") {
                    x.ident = Ident::new("str", x.ident.span());
                    return x;
                }
                x
            }).collect();

            let p = Path {
                leading_colon: t.path.leading_colon,
                segments: r,
            };

            let tp = TypePath {
                qself: t.qself.clone(),
                path: p,
            };
            let ct = Type::from(tp);

            let ref_t = TypeReference {
                and_token: Default::default(),
                lifetime: Some(l.clone()),
                mutability: None,
                elem: Box::new(ct),
            };
            CreateField {
                ident: &x.ident,
                ty: ref_t,
                vis: &x.vis,
            }
        }
        _ => {
            panic!("Unrecognized tab");
        }
    }
}