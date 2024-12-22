use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};


#[proc_macro_attribute]
pub fn jwt_guard(args: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);

    // 生成函数前的鉴权逻辑
    let function_name = &input_fn.sig.ident;
    let block = &input_fn.block;

    // 保留其他宏（如 #[get]）
    let attrs = &input_fn.attrs;
    let vis = &input_fn.vis;
    let sig = &input_fn.sig;
    let params = &input_fn.sig.inputs;
    //get first param actix_web::HttpRequest
    let first_param = &params[0];
    // from HttpRequest header get Authorization


    let output = quote! {
        #(#attrs)*
        #vis #sig {
            // JWT 验证逻辑
            if let Ok(claim) = validate_token(#first_param) {
                //add claim to req extension
                let mut req = req.extensions_mut();
                req.insert(claim);
                      #block
            }else{
                    return actix_web::HttpResponse::Unauthorized().json(actix_web::web::Json({
                    "status": "error",
                    "message": format!("Unauthorized: {}", e)
                }));
            }

        }
    };


    output.into()
}
