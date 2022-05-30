use proc_macro2::{Ident, TokenStream};
use quote::quote;

pub fn generate_patch_endpoint(mut r: TokenStream, s_ident: &Ident, ps_id: &Ident, endpoint_w_param: &str, mut endpoint_config: TokenStream) -> (TokenStream, TokenStream) {
    r = quote! {
        #r
        #[patch(#endpoint_w_param)]
        async fn patch(path: web::Path<i32>, req_body: String) -> impl Responder {
            let id = path.into_inner();
            if req_body.is_empty() || req_body.eq("{}") {
                return HttpResponse::BadRequest().body("No data passed");
            }
            let req: #ps_id = serde_json::from_str(req_body.as_str()).unwrap();
            let t = json!(#s_ident::patch(id, req, &establish_connection()));

            HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(t.to_string())
        }
    };
    endpoint_config = quote! {#endpoint_config.service(patch)};

    (r, endpoint_config)
}