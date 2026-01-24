use std::sync::Arc;

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
        auth::{
            forgot_password_handler::render_forgot_password,
            login_handler::{handle_login, render_login},
            reset_password_handler::render_reset_password,
            signup_handler::{handle_signup, render_signup},
        },
        inertia::version,
    },
    inertia::{dist_dir, is_dev, response_with_html},
};
use application::usecases::create_user_usecase::CreateUserUsecase;
use application::usecases::login_usecase::LoginUsecase;
use domain::repositories::user_repository::UserRepository;
use domain::traits::password_hasher::PasswordHasher;
use domain::traits::token_service::TokenService;

pub fn create_web_service(
    user_repository: Arc<dyn UserRepository>,
    password_hasher: Arc<dyn PasswordHasher>,
    token_service: Arc<dyn TokenService>,
) -> App<
    impl ServiceFactory<
        ServiceRequest,
        Response = ServiceResponse<impl MessageBody>,
        Config = (),
        InitError = (),
        Error = Error,
    >,
> {
    let dist = dist_dir();

    // Create use case instances (in production, this would come from DI container)
    let create_user_usecase = Arc::new(
        CreateUserUsecase::builder()
            .user_repository(user_repository.clone())
            .password_hasher(password_hasher.clone())
            .build(),
    );

    let login_usecase = Arc::new(
        LoginUsecase::builder()
            .user_repository(user_repository)
            .password_hasher(password_hasher)
            .token_service(token_service)
            .build(),
    );

    let mut app = App::new()
        .wrap(NormalizePath::trim())
        .wrap(Compress::default())
        .wrap(Logger::default())
        .app_data(web::Data::new(create_user_usecase))
        .app_data(web::Data::new(login_usecase))
        .route("/", web::get().to(index))
        .route("/login", web::get().to(render_login))
        .route("/login", web::post().to(handle_login))
        .route("/signup", web::get().to(render_signup))
        .route("/signup", web::post().to(handle_signup))
        .route("/forgot-password", web::get().to(render_forgot_password))
        .route("/reset-password", web::get().to(render_reset_password))
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
