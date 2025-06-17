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
        // Prioritize cookie-based authentication for better security
        let token = req
            .cookie("access_token")
            .map(|c| {
                // Log that we're using a cookie-based token for debugging
                // Menghapus logging ini di lingkungan produksi
                log::debug!("Using token from cookie for authentication");
                c.value().to_string()
            })
            .or_else(|| {
                // Fallback to Authorization header if cookie is not present
                req.headers()
                    .get(AUTHORIZATION)
                    .and_then(|auth| auth.to_str().ok())
                    .and_then(|auth_str| {
                        if auth_str.starts_with("Bearer ") {
                            log::debug!("Using Bearer token from Authorization header");
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
                log::debug!("No token found in middleware, letting handler manage authentication");
                return fut.await;
            }

            // Verify the token
            match verify_jwt(&token.unwrap()) {
                Ok(token_data) => { 
                    log::debug!("Token verified successfully for user: {}", token_data.claims.email);
                    // Token is valid, proceed
                    fut.await
                }
                Err(e) => {
                    error!("JWT verification failed: {:?}", e);
                    // Let the handler handle auth errors (this may be the issue)
                    fut.await
                }
            }
        })
    }
}
