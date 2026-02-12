use crate::session::clear_authenticated;
use actix_session::SessionExt;
use actix_web::{
    Responder,
    body::{BoxBody, EitherBody},
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
    web::Redirect,
};
use domain::traits::token_service::TokenService;
use futures_util::future::{LocalBoxFuture, Ready, ready};
use std::sync::Arc;

/// Redirects authenticated users to `/`. Use for guest-only routes (login, signup).
#[derive(Clone)]
pub struct GuestMiddleware;

impl GuestMiddleware {
    pub fn new() -> Self {
        Self
    }
}

impl<S, B> Transform<S, ServiceRequest> for GuestMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B, BoxBody>>;
    type Error = actix_web::Error;
    type Transform = GuestMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(GuestMiddlewareService { service }))
    }
}

pub struct GuestMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for GuestMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B, BoxBody>>;
    type Error = actix_web::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let session = req.get_session();
        let is_auth = session
            .get::<bool>("authenticated")
            .ok()
            .flatten()
            .unwrap_or(false);

        // Redirect authenticated users to home
        if is_auth {
            let response = Redirect::to("/")
                .see_other()
                .respond_to(req.request())
                .map_into_boxed_body();

            return Box::pin(async move { Ok(req.into_response(response).map_into_right_body()) });
        }

        let fut = self.service.call(req);
        Box::pin(async move { Ok(fut.await?.map_into_left_body()) })
    }
}

/// Redirects unauthenticated users to `/auth/login`. Use for protected routes.
#[derive(Clone)]
pub struct ProtectedMiddleware;

impl ProtectedMiddleware {
    pub fn new() -> Self {
        Self
    }
}

impl<S, B> Transform<S, ServiceRequest> for ProtectedMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B, BoxBody>>;
    type Error = actix_web::Error;
    type Transform = ProtectedMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ProtectedMiddlewareService { service }))
    }
}

pub struct ProtectedMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for ProtectedMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B, BoxBody>>;
    type Error = actix_web::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let session = req.get_session();
        let is_auth = session
            .get::<bool>("authenticated")
            .ok()
            .flatten()
            .unwrap_or(false);

        if is_auth {
            let token_opt: Option<String> = session.get("access_token").ok().flatten();

            if let Some(token) = token_opt {
                let token_service_opt: Option<Arc<dyn TokenService>> = req
                    .app_data::<actix_web::web::Data<Arc<dyn TokenService>>>()
                    .map(|data| data.get_ref().clone());

                if let Some(token_service) = token_service_opt {
                    let validation_result = token_service.validate_token(
                        token,
                        domain::traits::token_service::PasetoClaimPurpose::AccessToken,
                    );

                    if validation_result.is_err() {
                        clear_authenticated(&session);

                        let response = Redirect::to("/auth/login")
                            .see_other()
                            .respond_to(req.request())
                            .map_into_boxed_body();

                        return Box::pin(async move {
                            Ok(req.into_response(response).map_into_right_body())
                        });
                    }
                }
            }
        }

        // Redirect unauthenticated users to login
        if !is_auth {
            let response = Redirect::to("/auth/login")
                .see_other()
                .respond_to(req.request())
                .map_into_boxed_body();

            return Box::pin(async move { Ok(req.into_response(response).map_into_right_body()) });
        }

        let fut = self.service.call(req);
        Box::pin(async move { Ok(fut.await?.map_into_left_body()) })
    }
}
