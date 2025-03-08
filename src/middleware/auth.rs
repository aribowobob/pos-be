use actix_web::dev::Service;
use actix_web::dev::Transform;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::error::ErrorUnauthorized;
use actix_web::Error;
use std::future::{ready, Future, Ready};
use std::pin::Pin;
use std::task::{Context, Poll};

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
        ready(Ok(AuthMiddlewareService { service }))
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
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, request: ServiceRequest) -> Self::Future {
        // Skip authentication for /auth routes
        if request.path().starts_with("/auth") || request.path() == "/health" {
            let fut = self.service.call(request);
            return Box::pin(async move {
                let response = fut.await?;
                Ok(response)
            });
        }

        // Check for access_token cookie
        if let Some(cookie) = request.cookie("access_token") {
            let token = cookie.value().to_string();

            // Verify JWT token
            match crate::services::auth::verify_jwt(&token) {
                Ok(_claims) => {
                    let fut = self.service.call(request);
                    return Box::pin(async move {
                        let response = fut.await?;
                        Ok(response)
                    });
                }
                Err(_) => {
                    return Box::pin(async move { Err(ErrorUnauthorized("Invalid token")) });
                }
            }
        }

        Box::pin(async move { Err(ErrorUnauthorized("No authentication token")) })
    }
}
