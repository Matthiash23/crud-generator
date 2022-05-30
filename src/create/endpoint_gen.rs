use proc_macro2::{Ident, TokenStream};
use quote::quote;

pub fn generate_create_endpoints(mut r: TokenStream, s_ident: &Ident, ns_id: &Ident, endpoint: &str, mut endpoint_config: TokenStream) -> (TokenStream, TokenStream) {
    r = quote! {
        #r
        #[post(#endpoint)]
        async fn create(req_body: String) -> impl Responder {
            let req: #ns_id = serde_json::from_str(req_body.as_str()).unwrap();
            let t = json!(#s_ident::create(req, &establish_connection()));

            HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(t.to_string())
        }
    };
    endpoint_config = quote! {#endpoint_config.service(create)};

    (r, endpoint_config)
}