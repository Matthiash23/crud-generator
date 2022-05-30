use proc_macro2::{Ident, TokenStream};
use quote::quote;

pub fn generate_delete_endpoints(mut r: TokenStream, s_ident: &Ident, endpoint_w_param: &str, mut endpoint_config: TokenStream) -> (TokenStream, TokenStream) {
    r = quote! {
        #r
        #[delete(#endpoint_w_param)]
        async fn delete(path: web::Path<i32>) -> impl Responder {
            let id = path.into_inner();
            let r = json!(#s_ident::delete(id, &establish_connection()));

            HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(r.to_string())
        }
    };
    endpoint_config = quote! {#endpoint_config.service(delete)};

    (r, endpoint_config)
}