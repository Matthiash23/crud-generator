use proc_macro2::{Punct, Spacing, TokenStream};
use quote::{quote, TokenStreamExt, ToTokens};
use syn::{AngleBracketedGenericArguments, GenericArgument, Ident, Lifetime, Path, PathArguments, PathSegment, Token, Type, TypePath, TypeReference, Visibility};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;

use crate::commons::ParsedField;

#[derive(Debug, Clone)]
struct PatchField<'i> {
    ident: &'i Ident,
    ty: Type,
    vis: &'i Visibility,
}


impl<'i> ToTokens for PatchField<'i> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.vis.to_tokens(tokens);
        self.ident.to_tokens(tokens);
        tokens.append(Punct::new(':', Spacing::Alone));
        self.ty.to_tokens(tokens)
    }
}

pub fn generate_patch_struct(data_struct: &[ParsedField], main_lifetime: &Lifetime,
                             mut r: TokenStream, table_name_quote: impl ToTokens, ps_id: &Ident) -> TokenStream {
    let patch_fs = build_patch_fields(data_struct, main_lifetime).unwrap();
    r = quote! {
            #r
            #[derive(Debug, AsChangeset, Deserialize)]
            #table_name_quote
            pub struct #ps_id<#main_lifetime> {
                #(#patch_fs),*
            }
        };
    r
}

fn build_patch_fields<'i>(fields: &'i [ParsedField], l: &Lifetime) -> Option<Vec<PatchField<'i>>> {
    let mut r: Vec<PatchField> = vec![];
    for x in fields.iter() {
        if x.patchable {
            r.push(build_patch_field(x, l));
        }
    }
    if !r.is_empty() {
        Some(r)
    } else {
        panic!("Patch feature requested but no field is marked as patchable")
    }
}

fn build_patch_field<'i>(x: &'i ParsedField, l: &Lifetime) -> PatchField<'i> {
    match &x.ty {
        Type::Path(t) => {
            let r: Punctuated<PathSegment, Token![::]> = t.path.segments.clone().into_iter().map(|mut p| {
                if p.ident.to_string().eq("String") {
                    p.ident = Ident::new("str", x.ident.span());
                }

                let mut n_args: Punctuated<GenericArgument, Token![,]> = Punctuated::new();
                let mut n_path: Punctuated<PathSegment, Token![::]> = Punctuated::new();


                n_path.push(p);
                n_args.push(GenericArgument::Type(Type::Reference(
                    TypeReference {
                        and_token: Default::default(),
                        lifetime: Some(l.clone()),
                        mutability: None,
                        elem: Box::new(Type::Path(TypePath {
                            qself: None,
                            path: Path {
                                leading_colon: None,
                                segments: n_path,
                            },
                        })),
                    })));

                PathSegment {
                    ident: Ident::new("Option", t.span()),
                    arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                        colon2_token: None,
                        lt_token: Default::default(),
                        args: n_args,
                        gt_token: Default::default(),
                    }),
                }
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

            PatchField {
                ident: &x.ident,
                ty: ct,
                vis: &x.vis,
            }
        }
        _ => {
            panic!("Unrecognized tab");
        }
    }
}