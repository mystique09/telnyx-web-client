use std::sync::Arc;

use actix_files::Files;
use actix_inertia::{VersionMiddleware, inertia_responder::InertiaResponder};
use actix_session::{Session, SessionMiddleware};
use actix_web::{
    App, Error, HttpRequest, Responder, Result,
    body::MessageBody,
    cookie::Key,
    dev::{ServiceFactory, ServiceRequest, ServiceResponse},
    http::StatusCode,
    middleware::{Compress, ErrorHandlerResponse, ErrorHandlers, Logger, NormalizePath},
    web,
};
use serde::Serialize;

use crate::{
    Empty,
    dto::FlashProps,
    flash::{clear_flash, extract_flash},
    handlers::{
        auth::build_auth_service, conversations::build_conversations_service, inertia::version,
        phone_numbers::build_phone_numbers_service,
    },
    inertia::{Page, dist_dir, is_dev, response_with_html},
    middlewares::auth::ProtectedMiddleware,
};
use application::usecases::create_user_usecase::CreateUserUsecase;
use application::usecases::login_usecase::LoginUsecase;
use domain::repositories::conversation_repository::ConversationRepository;
use domain::repositories::phone_number_repository::PhoneNumberRepository;
use domain::repositories::user_repository::UserRepository;
use domain::traits::password_hasher::PasswordHasher;
use domain::traits::token_service::TokenService;

pub fn create_web_service(
    session_secret: String,
    user_repository: Arc<dyn UserRepository>,
    conversation_repository: Arc<dyn ConversationRepository>,
    phone_number_repository: Arc<dyn PhoneNumberRepository>,
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
            .token_service(token_service.clone())
            .build(),
    );

    let signing_key = Key::from(session_secret.as_bytes());

    let mut app = App::new()
        .wrap(
            SessionMiddleware::builder(
                actix_session::storage::CookieSessionStore::default(),
                signing_key,
            )
            .cookie_http_only(true)
            .cookie_secure(!is_dev())
            .build(),
        )
        .wrap(NormalizePath::trim())
        .wrap(Compress::default())
        .wrap(Logger::default())
        .wrap(ErrorHandlers::new().handler(StatusCode::NOT_FOUND, error_404_error_handler))
        .app_data(web::Data::new(create_user_usecase))
        .app_data(web::Data::new(login_usecase))
        .app_data(web::Data::new(conversation_repository))
        .app_data(web::Data::new(phone_number_repository))
        .app_data(web::Data::new(token_service.clone()))
        .route("/", web::get().to(index).wrap(ProtectedMiddleware::new()))
        .service(build_conversations_service())
        .service(build_phone_numbers_service())
        .service(build_auth_service())
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

#[derive(Debug, Serialize, bon::Builder)]
struct HomePageProps {
    pub flash: Option<FlashProps>,
}

async fn index(req: HttpRequest, session: Session) -> impl Responder {
    let flash = extract_flash(&session);

    if flash.is_some() {
        clear_flash(&session);
    }

    Page::builder()
        .req(req)
        .name("App")
        .props(HomePageProps::builder().maybe_flash(flash).build())
        .build()
        .to_responder()
}

fn error_404_error_handler<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let (req, _) = res.into_parts();

    let response = if req.headers().contains_key("x-inertia") {
        InertiaResponder::new("NotFound", Empty).respond_to(&req)
    } else {
        response_with_html(&req, Empty, "NotFound".to_string())
    };

    Ok(ErrorHandlerResponse::Response(
        ServiceResponse::new(req, response).map_into_right_body(),
    ))
}
