use proc_macro2::{Ident, TokenStream};
use quote::quote;

pub fn generate_read_endpoints(mut r: TokenStream, s_ident: &Ident, endpoint: &str, endpoint_w_param: &str, mut endpoint_config: TokenStream) -> (TokenStream, TokenStream) {
    r = quote! {
        #r
        #[get(#endpoint_w_param)]
        async fn read(path: web::Path<i32>) -> impl Responder {
            let id = path.into_inner();
            let r = json!(#s_ident::read(id, &establish_connection()));

            HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(r.to_string())
        }

        #[get(#endpoint)]
        async fn get() -> impl Responder {
            let r = json!(#s_ident::get(50, &establish_connection()));

            HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(r.to_string())
        }
    };
    endpoint_config = quote! {#endpoint_config.service(read).service(get)};

    (r, endpoint_config)
}