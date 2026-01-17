use actix_files::Files;
use actix_inertia::{VersionMiddleware, inertia_responder::InertiaResponder};
use actix_web::{
    App, Error, HttpRequest, Responder,
    body::MessageBody,
    dev::{ServiceFactory, ServiceRequest, ServiceResponse},
    middleware::{Compress, Logger, NormalizePath},
    web,
};

use crate::{
    Empty,
    handlers::{
        auth::{forgot_password, login, reset_password, signup},
        inertia::version,
    },
    inertia::{dist_dir, is_dev, response_with_html},
};

pub fn create_web_service() -> App<
    impl ServiceFactory<
        ServiceRequest,
        Response = ServiceResponse<impl MessageBody>,
        Config = (),
        InitError = (),
        Error = Error,
    >,
> {
    let dist = dist_dir();

    let mut app = App::new()
        .wrap(NormalizePath::trim())
        .wrap(Compress::default())
        .wrap(Logger::default())
        .route("/", web::get().to(index))
        .route("/login", web::get().to(login))
        .route("/signup", web::get().to(signup))
        .route("/forgot-password", web::get().to(forgot_password))
        .route("/reset-password", web::get().to(reset_password))
        .service(
            web::scope("/version")
                .wrap(VersionMiddleware::new("1".to_string()))
                .route("", web::get().to(version)),
        );

    // Production: serve built assets from dist/
    if !is_dev() {
        app = app
            .service(Files::new("/assets", dist.join("assets")).prefer_utf8(true))
            .service(Files::new("/", dist).prefer_utf8(true));
    }
    app
}

async fn index(req: HttpRequest) -> impl Responder {
    if req.headers().contains_key("x-inertia") {
        InertiaResponder::new("App", Empty).respond_to(&req)
    } else {
        response_with_html(&req, Empty, "App".to_string())
    }
}
