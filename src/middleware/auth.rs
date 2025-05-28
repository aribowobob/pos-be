use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::header::AUTHORIZATION,
    Error, HttpMessage,
};
use futures::future::{ok, LocalBoxFuture, Ready};
use log::error;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::errors::ServiceError;
use crate::services::auth::verify_jwt;

pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddlewareService { service })
    }
}

pub struct AuthMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
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
        // Check if route is marked to skip auth
        if req.extensions().contains::<super::skip_auth::SkipAuthFlag>() {
            return Box::pin(self.service.call(req));
        }

        // Skip auth for certain paths
        let path = req.path();
        if path == "/" || path.starts_with("/auth") || path.starts_with("/health") {
            return Box::pin(self.service.call(req));
        }

        // Get token from cookies or Authorization header
        let token = req
            .cookie("access_token")
            .map(|c| c.value().to_string())
            .or_else(|| {
                req.headers()
                    .get(AUTHORIZATION)
                    .and_then(|auth| auth.to_str().ok())
                    .and_then(|auth_str| {
                        if auth_str.starts_with("Bearer ") {
                            Some(auth_str[7..].to_string())
                        } else {
                            None
                        }
                    })
            });

        let fut = self.service.call(req);

        Box::pin(async move {
            // If there's no token, proceed and let the handler handle it
            if token.is_none() {
                return fut.await;
            }

            // Verify the token
            match verify_jwt(&token.unwrap()) {
                Ok(_token_data) => { // Prefix with underscore to ignore unused variable
                    // Token is valid, proceed
                    fut.await
                }
                Err(e) => {
                    error!("JWT verification failed: {:?}", e);
                    // Just let the handler handle auth errors
                    fut.await
                }
            }
        })
    }
}
