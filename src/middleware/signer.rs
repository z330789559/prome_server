
use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::error;
use futures_util::future::{LocalBoxFuture};
use std::future::{ready, Ready};
use actix_web::Error;
use log::info;

pub struct SignerValidator;

impl<S, B> Transform<S, ServiceRequest> for SignerValidator
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = SignerMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(SignerMiddleware { service }))
    }
}


pub struct  SignerMiddleware<S>{
    service: S,
}

impl<S, B> Service<ServiceRequest> for SignerMiddleware<S>
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // 进行鉴权操作，判断是否有权限
        if need_valid_and_pass(&req) {
            // 有权限，继续执行后续中间件
            let fut = self.service.call(req);
            Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            })
        } else {
            // 没有权限，立即返回响应
            Box::pin(async move {
                // 鉴权失败，返回未授权的响应，停止后续中间件的调用
                Err(error::ErrorUnauthorized("signature valid fail"))
            })
        }
    }
}

fn need_valid_and_pass(req: &ServiceRequest) -> bool{
    // 这里可以根据请求的信息，进行鉴权操作
    // 这里简单的返回true，表示有权限
    info!(target:"promote_node","/api/transaction");
    if req.path() == "/api/transaction" && req.method() == actix_web::http::Method::POST{

        return true
    }
    true
}