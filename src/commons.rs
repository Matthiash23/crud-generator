use quote::ToTokens;
use syn::{Ident, Type, Visibility};

#[derive(Debug)]
pub struct ParsedField {
    pub(crate) ident: Ident,
    pub(crate) ty: Type,
    pub(crate) vis: Visibility,
    pub(crate) creatable: bool,
    pub(crate) patchable: bool,
}

// special struct holding only the ident, required for using the fields in a non-definition context
#[derive(Debug)]
pub struct IdentParsedField<'i> {
    pub(crate) ident: &'i Ident,
}

impl<'i> ToTokens for IdentParsedField<'i> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.ident.to_tokens(tokens);
    }
}
