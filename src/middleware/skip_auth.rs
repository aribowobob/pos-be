use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures::future::{ok, Ready};
use std::marker::PhantomData;
use std::rc::Rc;
use std::task::{Context, Poll};

// Middleware marker to indicate routes that should skip authentication
pub struct SkipAuth;

impl<S, B> Transform<S, ServiceRequest> for SkipAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = SkipAuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(SkipAuthMiddleware {
            service: Rc::new(service),
            _phantom: PhantomData,
        })
    }
}

pub struct SkipAuthMiddleware<S> {
    service: Rc<S>,
    _phantom: PhantomData<()>,
}

impl<S, B> Service<ServiceRequest> for SkipAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = S::Future;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Mark this request to skip auth
        req.extensions_mut().insert(SkipAuthFlag);

        self.service.call(req)
    }
}

// Flag to indicate a request should skip auth
#[derive(Debug)]
pub struct SkipAuthFlag;
